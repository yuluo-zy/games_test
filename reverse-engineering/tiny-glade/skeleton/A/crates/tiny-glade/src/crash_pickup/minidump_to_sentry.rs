// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 7c69ae4b764ceeefdfd7526d029d23c4270733d7ab96539637b0ce802c0ded1a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: minidump_to_sentry.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 213
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use minidump_unwind; // namespace candidate
//   package identities: [{"name":"minidump-unwind","source":"PDB_registry_path_only","version":"0.21.1"}]
// use sentry_types; // namespace candidate
//   package identities: [{"name":"sentry-types","source":"PDB_registry_path_only","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":213,"record_offset":11304,"rva":3353184,"va":5372062304}
// 编译实例 RVAs: 0x332a60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$T$u20$as$u20$alloc..borrow..ToOwned$GT$::to_owned
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-387519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e84116faff","size":5,"soft_fail":false,"target_va":5371676496,"va":5372064010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140065920
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2939189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8cb26d3ff","size":5,"soft_fail":false,"target_va":5369125152,"va":5372064336,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e09a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2435618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8ded5daff","size":5,"soft_fail":false,"target_va":5369629088,"va":5372064701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14017c870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1796432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b096e4ff","size":5,"soft_fail":false,"target_va":5370267760,"va":5372064187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14017dbc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1791816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b8a8e4ff","size":5,"soft_fail":false,"target_va":5370272704,"va":5372064515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cf70
//     owner=unresolved_namespace; count=2
//     target_aliases=31 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t170735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8ef9a0200","size":5,"soft_fail":false,"target_va":5372235632,"va":5372064892,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037ad40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t294260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8747d0400","size":5,"soft_fail":false,"target_va":5372357952,"va":5372063687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037e430
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t308713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8e9b50400","size":5,"soft_fail":false,"target_va":5372372016,"va":5372063298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037e560
//     owner=unresolved_namespace; count=2
//     target_aliases=12 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t307342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e88eb00400","size":5,"soft_fail":false,"target_va":5372372320,"va":5372064973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fda0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t313996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e88cca0400","size":5,"soft_fail":false,"target_va":5372378528,"va":5372064527,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403867c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t340706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8e2320500","size":5,"soft_fail":false,"target_va":5372405696,"va":5372064985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403870e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t342469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8c5390500","size":5,"soft_fail":false,"target_va":5372408032,"va":5372065558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038ce40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t366007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b7950500","size":5,"soft_fail":false,"target_va":5372431936,"va":5372065924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038f8c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t377381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e825c20500","size":5,"soft_fail":false,"target_va":5372442816,"va":5372065430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t686367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e81f790a00","size":5,"soft_fail":false,"target_va":5372749712,"va":5372063340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30837786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e81a8cd601","size":5,"soft_fail":false,"target_va":5402900112,"va":5372062321,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31530931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b31fe101","size":5,"soft_fail":false,"target_va":5403593360,"va":5372062424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e87be5ffff","size":5,"soft_fail":false,"target_va":5372057408,"va":5372064192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h17e9edfa19878bd5 (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t30882880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8403cd701","size":5,"soft_fail":false,"target_va":5402946160,"va":5372063275,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h2a54626eb80096ed (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t30884436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e85442d701","size":5,"soft_fail":false,"target_va":5402946992,"va":5372062551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidump::minidump::UnifiedMemoryList> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e806e2ffff","size":5,"soft_fail":false,"target_va":5372057280,"va":5372064949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::context::MinidumpContext::get_stack_pointer
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b2e6ffff","size":5,"soft_fail":false,"target_va":5372057472,"va":5372063945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::Minidump$LT$T$GT$::get_stream
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t430898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e832930600","size":5,"soft_fail":false,"target_va":5372493360,"va":5372062457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::Minidump$LT$memmap2..Mmap$GT$::read_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t433828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8a49e0600","size":5,"soft_fail":false,"target_va":5372496208,"va":5372062375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::context
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1669413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e825791900","size":5,"soft_fail":false,"target_va":5373732640,"va":5372063222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpException::get_crash_reason
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1668148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e834741900","size":5,"soft_fail":false,"target_va":5373732800,"va":5372064647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::MinidumpSystemInfo::cpu_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1659009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e881501900","size":5,"soft_fail":false,"target_va":5373723312,"va":5372064298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump::minidump::UnifiedMemoryList::memory_at_address
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1651212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e80c321900","size":5,"soft_fail":false,"target_va":5373715184,"va":5372063967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::CallStack::with_context
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1243827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e8b3fa1200","size":5,"soft_fail":false,"target_va":5373307920,"va":5372064088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidump_unwind::symbols::MultiSymbolProvider::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1242267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e89bf41200","size":5,"soft_fail":false,"target_va":5373306528,"va":5372064256,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] sentry_types::protocol::v7::Stacktrace::from_frames_reversed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1027908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e844af0f00","size":5,"soft_fail":false,"target_va":5373092640,"va":5372064727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t5333037","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372062304,"raw_hex":"e82d605100","size":5,"soft_fail":false,"target_va":5377395552,"va":5372062510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":213,"record_offset":12956,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":5874,"source_line":323,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::borrow::impl$4::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\borrow.rs
//     evidence={"module":213,"record_offset":12928,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":7440,"source_line":338,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":213,"record_offset":13076,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":213,"record_offset":13200,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":213,"record_offset":11528,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$1::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":213,"record_offset":12504,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":7558,"source_line":91,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":213,"record_offset":13312,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":213,"record_offset":13256,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":213,"record_offset":12476,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":213,"record_offset":13112,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":213,"record_offset":11932,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":12604,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":11768,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":12640,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":12969,"source_line":1380,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":11740,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::MultiSymbolProvider::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":213,"record_offset":12992,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":12972,"source_line":154,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::symbols::debuginfo::DebugInfoSymbolProviderBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/symbols/debuginfo.rs
//     evidence={"module":213,"record_offset":13048,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":12976,"source_line":406,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] minidump_unwind::walk_stack
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/lib.rs
//     evidence={"module":213,"record_offset":13172,"parent_record_offset":11304,"ancestor_record_offset":11304,"inlinee":12979,"source_line":725,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":12112,"parent_record_offset":12088,"ancestor_record_offset":11304,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":11304,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":13496,"parent_record_offset":13468,"ancestor_record_offset":11304,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":11304,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":213,"record_offset":13336,"parent_record_offset":13312,"ancestor_record_offset":11304,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":213,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":11304,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_display
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":213,"record_offset":12544,"parent_record_offset":12504,"ancestor_record_offset":11304,"inlinee":7560,"source_line":143,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":213,"record_offset":11568,"parent_record_offset":11528,"ancestor_record_offset":11304,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":213,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":11304,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":213,"record_offset":11960,"parent_record_offset":11932,"ancestor_record_offset":11304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":213,"record_offset":13224,"parent_record_offset":13200,"ancestor_record_offset":11304,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":12696,"parent_record_offset":12668,"ancestor_record_offset":11304,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] minidump_unwind::symbols::impl$1::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"minidump-unwind","version":"0.21.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/minidump-unwind-0.21.1/src/symbols/mod.rs
//     evidence={"module":213,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":11304,"inlinee":12974,"source_line":148,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":213,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":11304,"inlinee":12970,"source_line":78,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":213,"record_offset":13140,"parent_record_offset":13112,"ancestor_record_offset":11304,"inlinee":12977,"source_line":101,"archive":"_pdb/A/modules/00213.jsonl.gz"}
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
// 首条位置: {"module":213,"record_offset":5976,"rva":3348432,"va":5372057552}
// 编译实例 RVAs: 0x3317d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$minidump..minidump..CrashReason$u20$as$u20$core..fmt..Display$GT$::fmt
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1666522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8da6d1900","size":5,"soft_fail":false,"target_va":5373725808,"va":5372059281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$sentry_types..protocol..v7..Event$u20$as$u20$core..default..Default$GT$::default
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1039347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8f3db0f00","size":5,"soft_fail":false,"target_va":5373097520,"va":5372058168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035cc60
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t173450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e88aa50200","size":5,"soft_fail":false,"target_va":5372234848,"va":5372061393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037cdc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t307126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8b6af0400","size":5,"soft_fail":false,"target_va":5372366272,"va":5372059141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t304765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e87da60400","size":5,"soft_fail":false,"target_va":5372366736,"va":5372061966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037ede0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t312808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8e8c50400","size":5,"soft_fail":false,"target_va":5372374496,"va":5372061683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t692059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e85b8f0a00","size":5,"soft_fail":false,"target_va":5372749696,"va":5372057632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t688992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e860830a00","size":5,"soft_fail":false,"target_va":5372749712,"va":5372060715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31535290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8ba30e101","size":5,"soft_fail":false,"target_va":5403593360,"va":5372058065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t30917337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8d9c2d701","size":5,"soft_fail":false,"target_va":5402978144,"va":5372060802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5471873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e8817e5300","size":5,"soft_fail":false,"target_va":5377529888,"va":5372058010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t5303604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e834ed5000","size":5,"soft_fail":false,"target_va":5377364336,"va":5372060727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30919253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372057552,"raw_hex":"e855cad701","size":5,"soft_fail":false,"target_va":5402980128,"va":5372060870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":6156,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":213,"record_offset":10460,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":213,"record_offset":7420,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$31::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":213,"record_offset":8056,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":5370,"source_line":2549,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":213,"record_offset":6304,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":213,"record_offset":7560,"parent_record_offset":5976,"ancestor_record_offset":5976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":5976,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":6236,"parent_record_offset":6212,"ancestor_record_offset":5976,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":5976,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":6188,"parent_record_offset":6156,"ancestor_record_offset":5976,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":213,"record_offset":7804,"parent_record_offset":7712,"ancestor_record_offset":5976,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":213,"record_offset":7468,"parent_record_offset":7444,"ancestor_record_offset":5976,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":7740,"parent_record_offset":7712,"ancestor_record_offset":5976,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":6592,"parent_record_offset":6564,"ancestor_record_offset":5976,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":6564,"parent_record_offset":6536,"ancestor_record_offset":5976,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":6736,"parent_record_offset":6564,"ancestor_record_offset":5976,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":213,"record_offset":7712,"parent_record_offset":7688,"ancestor_record_offset":5976,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":213,"record_offset":6500,"parent_record_offset":6472,"ancestor_record_offset":5976,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":213,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":5976,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":213,"record_offset":6416,"parent_record_offset":6388,"ancestor_record_offset":5976,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":213,"record_offset":6444,"parent_record_offset":6416,"ancestor_record_offset":5976,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":213,"record_offset":6388,"parent_record_offset":6360,"ancestor_record_offset":5976,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":213,"record_offset":6360,"parent_record_offset":6332,"ancestor_record_offset":5976,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":213,"record_offset":8108,"parent_record_offset":8056,"ancestor_record_offset":5976,"inlinee":5371,"source_line":448,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":213,"record_offset":6332,"parent_record_offset":6304,"ancestor_record_offset":5976,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":213,"record_offset":6536,"parent_record_offset":6500,"ancestor_record_offset":5976,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":213,"record_offset":7624,"parent_record_offset":7592,"ancestor_record_offset":5976,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":213,"record_offset":8136,"parent_record_offset":8056,"ancestor_record_offset":5976,"inlinee":5373,"source_line":274,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":213,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":5976,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":213,"record_offset":7768,"parent_record_offset":7740,"ancestor_record_offset":5976,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":213,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":5976,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":213,"record_offset":6772,"parent_record_offset":6500,"ancestor_record_offset":5976,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":213,"record_offset":7592,"parent_record_offset":7560,"ancestor_record_offset":5976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":213,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":5976,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":8164,"parent_record_offset":8056,"ancestor_record_offset":5976,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   [pdb_inline_transitive] sentry_types::protocol::v7::impl$2::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"sentry-types","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-types-0.31.7/src/protocol/v7.rs
//     evidence={"module":213,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":5976,"inlinee":5581,"source_line":84,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32AA
// 首条位置: {"module":213,"inlinee":12970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":213,"record_offset":12696,"parent_record_offset":12668,"ancestor_record_offset":12668,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00213.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x32B1
// 首条位置: {"module":213,"inlinee":12977}
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
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":21,"record_offset":40004,"rva":182000,"va":5368891120}
// 编译实例 RVAs: 0x2c6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140366790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3382929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e8919e3300","size":5,"soft_fail":false,"target_va":5372274576,"va":5368891642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140375db0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3445500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e8fc923400","size":5,"soft_fail":false,"target_va":5372337584,"va":5368892079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037ad40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3465896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e8a8e23400","size":5,"soft_fail":false,"target_va":5372357952,"va":5368892051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403878f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e861ae3500","size":5,"soft_fail":false,"target_va":5372410096,"va":5368892042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3858319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e88fdf3a00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368891388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34702091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368891120,"raw_hex":"e80b831102","size":5,"soft_fail":false,"target_va":5403593360,"va":5368891264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: minidump_to_sentry.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t3857744","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5368891120,"raw_hex":"e950dd3a00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368891963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":40172,"parent_record_offset":40004,"ancestor_record_offset":40004,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":40496,"parent_record_offset":40004,"ancestor_record_offset":40004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":40432,"parent_record_offset":40408,"ancestor_record_offset":40004,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":40408,"parent_record_offset":40352,"ancestor_record_offset":40004,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":40380,"parent_record_offset":40352,"ancestor_record_offset":40004,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":40352,"parent_record_offset":40328,"ancestor_record_offset":40004,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":21,"record_offset":40520,"parent_record_offset":40496,"ancestor_record_offset":40004,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":21,"record_offset":41532,"parent_record_offset":41504,"ancestor_record_offset":40004,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":21,"record_offset":41556,"parent_record_offset":41532,"ancestor_record_offset":40004,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":21,"record_offset":41112,"parent_record_offset":41084,"ancestor_record_offset":40004,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":40228,"parent_record_offset":40200,"ancestor_record_offset":40004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":21,"record_offset":41620,"parent_record_offset":41596,"ancestor_record_offset":40004,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":21,"record_offset":41596,"parent_record_offset":41472,"ancestor_record_offset":40004,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41472,"parent_record_offset":41440,"ancestor_record_offset":40004,"inlinee":6232,"source_line":3289,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41384,"parent_record_offset":41360,"ancestor_record_offset":40004,"inlinee":6229,"source_line":2430,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41440,"parent_record_offset":41384,"ancestor_record_offset":40004,"inlinee":6231,"source_line":3259,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41412,"parent_record_offset":41384,"ancestor_record_offset":40004,"inlinee":6230,"source_line":2827,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41504,"parent_record_offset":41472,"ancestor_record_offset":40004,"inlinee":6234,"source_line":256,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":21,"record_offset":41360,"parent_record_offset":41336,"ancestor_record_offset":40004,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/minidump_to_sentry.rs
//     evidence={"module":21,"record_offset":40200,"parent_record_offset":40172,"ancestor_record_offset":40004,"inlinee":6228,"source_line":117,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1854
// 首条位置: {"module":21,"inlinee":6228}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":40228,"parent_record_offset":40200,"ancestor_record_offset":40200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":40432,"parent_record_offset":40408,"ancestor_record_offset":40200,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":40408,"parent_record_offset":40352,"ancestor_record_offset":40200,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":40380,"parent_record_offset":40352,"ancestor_record_offset":40200,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":40352,"parent_record_offset":40328,"ancestor_record_offset":40200,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":40256,"parent_record_offset":40228,"ancestor_record_offset":40200,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] serde::__private::de::content::ContentDeserializer<serde_json::error::Error> (*)(enum2$<serde::__private::de::content::Content>)
// 候选参数类型: ["enum2$<serde::__private::de::content::Content>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<i64,winit::monitor::VideoMode> >,ref$<tuple$<i64,winit::monitor::VideoMode> >)
// 候选参数类型: ["ref$<tuple$<i64,winit::monitor::VideoMode> >", "ref$<tuple$<i64,winit::monitor::VideoMode> >"]
// [文件级接口候选，具体函数归属 unknown] sentry_types::protocol::v7::Breadcrumb (*)(ref$<sentry_types::protocol::v7::Breadcrumb>)
// 候选参数类型: ["ref$<sentry_types::protocol::v7::Breadcrumb>"]

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::crash_trace_to_sentry_event
// 信息 ID: be1f1dcbf31fa273566d7a69
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::panic_reporter::panic_report::on_game_crashed::closure_env$5,enum2$<core::result::Result<alloc::string::String,anyhow::Error> > > @ 0x1cd8f -> 0x3317d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace
// 信息 ID: 16fa4bc75579cc824074cd18
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::panic_report::on_game_crashed @ 0x715a -> 0x332a60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$0
// 信息 ID: b563d4fdd0e695fef50c0515
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=213, inline_record=12668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$1
// 信息 ID: 3dc69953e78ea93909475004
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace; module=213, inline_record=13140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2
// 信息 ID: 596c31a0c669859980882c4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Frame,alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Frame,alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2 <- alloc::vec::in_place_collect::impl$1::from_iter<sentry_types::protocol::v7::Frame,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 3 条。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Frame,alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Frame,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Frame,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<minidump_unwind::StackFrame,alloc::alloc::Global>,tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure_env$2> > > > @ 0x2a3ef -> 0x2c6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2::closure$1
// 信息 ID: e5a4acf131020b3beeb107cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::minidump_to_sentry::minidump_to_crash_trace::closure$2; module=21, inline_record=40200; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
