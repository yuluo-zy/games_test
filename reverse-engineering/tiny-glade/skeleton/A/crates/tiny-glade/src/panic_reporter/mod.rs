// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/panic_reporter/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 53354501611ebae6430d02da550b4186cfa394090d84291fa1df92c56560cfa6
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 21, 145
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::setup_panic_hook
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":10976,"rva":2589888,"va":5371299008}
// 编译实例 RVAs: 0x2784c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140277a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e8e4f4ffff","size":5,"soft_fail":false,"target_va":5371296400,"va":5371299239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037f2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1076539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e83b6d1000","size":5,"soft_fail":false,"target_va":5372375744,"va":5371299200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1450603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e86b221600","size":5,"soft_fail":false,"target_va":5372749696,"va":5371299088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31678989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e80d62e301","size":5,"soft_fail":false,"target_va":5402978144,"va":5371299150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::set_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6153956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e8e4e65d00","size":5,"soft_fail":false,"target_va":5377453088,"va":5371299127,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::take_hook
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6154693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299008,"raw_hex":"e8c5e95d00","size":5,"soft_fail":false,"target_va":5377453744,"va":5371299046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":145,"record_offset":11108,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":11288,"parent_record_offset":10976,"ancestor_record_offset":10976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":11192,"parent_record_offset":11168,"ancestor_record_offset":10976,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":11216,"parent_record_offset":11192,"ancestor_record_offset":10976,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":11140,"parent_record_offset":11108,"ancestor_record_offset":10976,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":11168,"parent_record_offset":11140,"ancestor_record_offset":10976,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10976,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":11320,"rva":2590144,"va":5371299264}
// 编译实例 RVAs: 0x2785c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1625958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e89a30e7ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5371299697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402783a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8aafaffff","size":5,"soft_fail":false,"target_va":5371298720,"va":5371300081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1119384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e898141100","size":5,"soft_fail":false,"target_va":5372419712,"va":5371300323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1125738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e86a2d1100","size":5,"soft_fail":false,"target_va":5372426160,"va":5371300417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32293189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e845c1ec01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371300166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5068763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8db574d00","size":5,"soft_fail":false,"target_va":5376368480,"va":5371299712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::sync::mutex::MutexGuard<bool>,std::sync::poison::PoisonError<std::sync::mutex::MutexGuard<bool> > > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8aaeeffff","size":5,"soft_fail":false,"target_va":5371296064,"va":5371300497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::expect<tuple$<>,enum2$<tiny_glade::crash_pickup::Error> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e814f2ffff","size":5,"soft_fail":false,"target_va":5371296704,"va":5371300263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6053907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e813605c00","size":5,"soft_fail":false,"target_va":5377353776,"va":5371299864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6119138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8e25e5d00","size":5,"soft_fail":false,"target_va":5377418800,"va":5371299657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31675813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8a555e301","size":5,"soft_fail":false,"target_va":5402975328,"va":5371299510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31676011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e86b56e301","size":5,"soft_fail":false,"target_va":5402975504,"va":5371299488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31676147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8f356e301","size":5,"soft_fail":false,"target_va":5402975696,"va":5371299544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::crash_pickup::ClientHandle::send_message
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1293559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8f7bc1300","size":5,"soft_fail":false,"target_va":5372593808,"va":5371300244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::recover_game_state_after_crash
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e82d020000","size":5,"soft_fail":false,"target_va":5371300576,"va":5371300014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2138628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371299264,"raw_hex":"e8fc5ddfff","size":5,"soft_fail":false,"target_va":5369161440,"va":5371300063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::boxed::impl$49::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":145,"record_offset":13112,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":8413,"source_line":2033,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":11468,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::panic_reporter::panic_handler
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":11500,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":11354,"source_line":40,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":145,"record_offset":12744,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":12576,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":12772,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":12132,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":11772,"parent_record_offset":11712,"ancestor_record_offset":11320,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11320,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12412,"parent_record_offset":12224,"ancestor_record_offset":11320,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11712,"parent_record_offset":11680,"ancestor_record_offset":11320,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12504,"parent_record_offset":12472,"ancestor_record_offset":11320,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11968,"parent_record_offset":11936,"ancestor_record_offset":11320,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11744,"parent_record_offset":11712,"ancestor_record_offset":11320,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":11320,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":11320,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12528,"parent_record_offset":12504,"ancestor_record_offset":11320,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":145,"record_offset":12800,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":145,"record_offset":12660,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":145,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11320,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":145,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":11320,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":145,"record_offset":12604,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":7931,"source_line":2357,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":11636,"parent_record_offset":11500,"ancestor_record_offset":11320,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":11808,"parent_record_offset":11636,"ancestor_record_offset":11320,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":12188,"parent_record_offset":12160,"ancestor_record_offset":11320,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":12224,"parent_record_offset":12188,"ancestor_record_offset":11320,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":12036,"parent_record_offset":11840,"ancestor_record_offset":11320,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":11840,"parent_record_offset":11808,"ancestor_record_offset":11320,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":145,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":11320,"inlinee":7933,"source_line":327,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":145,"record_offset":11680,"parent_record_offset":11636,"ancestor_record_offset":11320,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":145,"record_offset":12472,"parent_record_offset":12188,"ancestor_record_offset":11320,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":145,"record_offset":11880,"parent_record_offset":11840,"ancestor_record_offset":11320,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::panic_handler
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C5A
// 首条位置: {"module":145,"inlinee":11354}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":145,"record_offset":12744,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":12576,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":12772,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":12132,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":145,"record_offset":12800,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":145,"record_offset":12660,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::process::abort
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\process.rs
//     evidence={"module":145,"record_offset":12604,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":7931,"source_line":2357,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":11636,"parent_record_offset":11500,"ancestor_record_offset":11500,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":12160,"parent_record_offset":12132,"ancestor_record_offset":11500,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":11772,"parent_record_offset":11712,"ancestor_record_offset":11500,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12060,"parent_record_offset":12036,"ancestor_record_offset":11500,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12412,"parent_record_offset":12224,"ancestor_record_offset":11500,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11712,"parent_record_offset":11680,"ancestor_record_offset":11500,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12504,"parent_record_offset":12472,"ancestor_record_offset":11500,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11968,"parent_record_offset":11936,"ancestor_record_offset":11500,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11744,"parent_record_offset":11712,"ancestor_record_offset":11500,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":11500,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12436,"parent_record_offset":12412,"ancestor_record_offset":11500,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":145,"record_offset":12528,"parent_record_offset":12504,"ancestor_record_offset":11500,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":145,"record_offset":11936,"parent_record_offset":11908,"ancestor_record_offset":11500,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":145,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":11500,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":11808,"parent_record_offset":11636,"ancestor_record_offset":11500,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":145,"record_offset":12188,"parent_record_offset":12160,"ancestor_record_offset":11500,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":12224,"parent_record_offset":12188,"ancestor_record_offset":11500,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":12036,"parent_record_offset":11840,"ancestor_record_offset":11500,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":145,"record_offset":11840,"parent_record_offset":11808,"ancestor_record_offset":11500,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::abort_internal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\mod.rs
//     evidence={"module":145,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":11500,"inlinee":7933,"source_line":327,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":145,"record_offset":11680,"parent_record_offset":11636,"ancestor_record_offset":11500,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":145,"record_offset":12472,"parent_record_offset":12188,"ancestor_record_offset":11500,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":145,"record_offset":11880,"parent_record_offset":11840,"ancestor_record_offset":11500,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":13144,"rva":2591456,"va":5371300576}
// 编译实例 RVAs: 0x278ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2413893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8bb2adbff","size":5,"soft_fail":false,"target_va":5368888176,"va":5371302064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14035d620
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t934224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e850410e00","size":5,"soft_fail":false,"target_va":5372237344,"va":5371303115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=1
//     target_aliases=89 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1011166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8de6d0f00","size":5,"soft_fail":false,"target_va":5372314208,"va":5371303037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1447725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e82d171600","size":5,"soft_fail":false,"target_va":5372749696,"va":5371301966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1448955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8fb1b1600","size":5,"soft_fail":false,"target_va":5372749712,"va":5371300752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=7
//     target_aliases=28 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3870238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e81e0e3b00","size":5,"soft_fail":false,"target_va":5375171056,"va":5371300813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31675219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e85353e301","size":5,"soft_fail":false,"target_va":5402978144,"va":5371302920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6229161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8a90c5f00","size":5,"soft_fail":false,"target_va":5377529888,"va":5371300722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$anyhow..Error$GT$::construct::h8068e61a17c505da (.llvm.2708284726122056777)
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t31647611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e87be7e201","size":5,"soft_fail":false,"target_va":5402948592,"va":5371300976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t6061990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8a67f5c00","size":5,"soft_fail":false,"target_va":5377364336,"va":5371302341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::path::PathBuf,alloc::string::String> > >
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t-2570882","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e87ec5d8ff","size":5,"soft_fail":false,"target_va":5368732400,"va":5371303277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<std::path::PathBuf,enum2$<core::option::Option<country_core::resources::session::ActiveSession> > >,enum2$<tiny_glade::crash_pickup::crash_payload::SaveRecoveryError> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e86ee3ffff","size":5,"soft_fail":false,"target_va":5371296144,"va":5371303453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2952699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8fb0d2d00","size":5,"soft_fail":false,"target_va":5374255184,"va":5371302480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6094671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e84fff5c00","size":5,"soft_fail":false,"target_va":5377395552,"va":5371300876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6107909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e805335d00","size":5,"soft_fail":false,"target_va":5377408752,"va":5371300838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_inputs
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1344147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e893821400","size":5,"soft_fail":false,"target_va":5372645424,"va":5371301272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_log_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1343462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8e67f1400","size":5,"soft_fail":false,"target_va":5372644912,"va":5371301445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_profile_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1346764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8cc8c1400","size":5,"soft_fail":false,"target_va":5372648560,"va":5371301791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_save
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1346052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8048a1400","size":5,"soft_fail":false,"target_va":5372647088,"va":5371301031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_dump::dump_settings_file
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1346265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8d98a1400","size":5,"soft_fail":false,"target_va":5372647888,"va":5371301618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::panic_screenshot::dump_screenshot
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1349418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e82a971400","size":5,"soft_fail":false,"target_va":5372650512,"va":5371301089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t2905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e8590b0000","size":5,"soft_fail":false,"target_va":5371304064,"va":5371301154,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5644382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371300576,"raw_hex":"e85e205600","size":5,"soft_fail":false,"target_va":5376945008,"va":5371300621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":13328,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":145,"record_offset":13948,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":145,"record_offset":15272,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":13408,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":15216,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4722,"source_line":682,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":15244,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":11364,"source_line":654,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":15832,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":145,"record_offset":14336,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":145,"record_offset":13668,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":13612,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":14056,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":13144,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":13144,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":13564,"parent_record_offset":13540,"ancestor_record_offset":13144,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":15544,"parent_record_offset":15512,"ancestor_record_offset":13144,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":13540,"parent_record_offset":13484,"ancestor_record_offset":13144,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":16248,"parent_record_offset":16220,"ancestor_record_offset":13144,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":13376,"parent_record_offset":13352,"ancestor_record_offset":13144,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":13512,"parent_record_offset":13484,"ancestor_record_offset":13144,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":15512,"parent_record_offset":15484,"ancestor_record_offset":13144,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":15484,"parent_record_offset":15456,"ancestor_record_offset":13144,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":15664,"parent_record_offset":15484,"ancestor_record_offset":13144,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":13484,"parent_record_offset":13456,"ancestor_record_offset":13144,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":15728,"parent_record_offset":15704,"ancestor_record_offset":13144,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":15428,"parent_record_offset":15380,"ancestor_record_offset":13144,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":15456,"parent_record_offset":15428,"ancestor_record_offset":13144,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":15308,"parent_record_offset":15272,"ancestor_record_offset":13144,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":145,"record_offset":15704,"parent_record_offset":15380,"ancestor_record_offset":13144,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":145,"record_offset":15344,"parent_record_offset":15308,"ancestor_record_offset":13144,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":145,"record_offset":15380,"parent_record_offset":15344,"ancestor_record_offset":13144,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":145,"record_offset":14016,"parent_record_offset":13980,"ancestor_record_offset":13144,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":145,"record_offset":13980,"parent_record_offset":13948,"ancestor_record_offset":13144,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":145,"record_offset":13720,"parent_record_offset":13668,"ancestor_record_offset":13144,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":145,"record_offset":15752,"parent_record_offset":15728,"ancestor_record_offset":13144,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":145,"record_offset":15776,"parent_record_offset":15752,"ancestor_record_offset":13144,"inlinee":8548,"source_line":813,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":16480,"parent_record_offset":16444,"ancestor_record_offset":13144,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":13352,"parent_record_offset":13328,"ancestor_record_offset":13144,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":13432,"parent_record_offset":13408,"ancestor_record_offset":13144,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":15620,"parent_record_offset":15592,"ancestor_record_offset":13144,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":145,"record_offset":13816,"parent_record_offset":13792,"ancestor_record_offset":13144,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":145,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13144,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":145,"record_offset":13840,"parent_record_offset":13816,"ancestor_record_offset":13144,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":145,"record_offset":13916,"parent_record_offset":13668,"ancestor_record_offset":13144,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":145,"record_offset":13692,"parent_record_offset":13668,"ancestor_record_offset":13144,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":13144,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":13144,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":13768,"parent_record_offset":13744,"ancestor_record_offset":13144,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":145,"record_offset":13744,"parent_record_offset":13720,"ancestor_record_offset":13144,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":14368,"parent_record_offset":14336,"ancestor_record_offset":13144,"inlinee":11359,"source_line":103,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":14572,"parent_record_offset":14540,"ancestor_record_offset":13144,"inlinee":11360,"source_line":106,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":14776,"parent_record_offset":14744,"ancestor_record_offset":13144,"inlinee":11361,"source_line":108,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":14980,"parent_record_offset":14948,"ancestor_record_offset":13144,"inlinee":11362,"source_line":111,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":15184,"parent_record_offset":15152,"ancestor_record_offset":13144,"inlinee":11363,"source_line":114,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$7
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":16220,"parent_record_offset":16176,"ancestor_record_offset":13144,"inlinee":11365,"source_line":132,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":16444,"parent_record_offset":16396,"ancestor_record_offset":13144,"inlinee":11366,"source_line":134,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":16512,"parent_record_offset":16480,"ancestor_record_offset":13144,"inlinee":11368,"source_line":136,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":17404,"rva":2594944,"va":5371304064}
// 编译实例 RVAs: 0x279880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6225505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371304064,"raw_hex":"e861fe5e00","size":5,"soft_fail":false,"target_va":5377529888,"va":5371304378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t6059944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371304064,"raw_hex":"e8a8775c00","size":5,"soft_fail":false,"target_va":5377364336,"va":5371304387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6114512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371304064,"raw_hex":"e8d04c5d00","size":5,"soft_fail":false,"target_va":5377418800,"va":5371304283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":17604,"parent_record_offset":17404,"ancestor_record_offset":17404,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":145,"record_offset":17576,"parent_record_offset":17404,"ancestor_record_offset":17404,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":17688,"parent_record_offset":17404,"ancestor_record_offset":17404,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":17652,"parent_record_offset":17628,"ancestor_record_offset":17404,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":17628,"parent_record_offset":17604,"ancestor_record_offset":17404,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C5F
// 首条位置: {"module":145,"inlinee":11359}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C60
// 首条位置: {"module":145,"inlinee":11360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C61
// 首条位置: {"module":145,"inlinee":11361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C62
// 首条位置: {"module":145,"inlinee":11362}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C63
// 首条位置: {"module":145,"inlinee":11363}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1825
// 首条位置: {"module":21,"inlinee":6181}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":28556,"parent_record_offset":27944,"ancestor_record_offset":27944,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":21,"record_offset":27996,"parent_record_offset":27944,"ancestor_record_offset":27944,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":28640,"parent_record_offset":27944,"ancestor_record_offset":27944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":28336,"parent_record_offset":28312,"ancestor_record_offset":27944,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":28360,"parent_record_offset":28336,"ancestor_record_offset":27944,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":28820,"parent_record_offset":28796,"ancestor_record_offset":27944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":28312,"parent_record_offset":28284,"ancestor_record_offset":27944,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":21,"record_offset":28796,"parent_record_offset":28740,"ancestor_record_offset":27944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":21,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":27944,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":28768,"parent_record_offset":28740,"ancestor_record_offset":27944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":28284,"parent_record_offset":28256,"ancestor_record_offset":27944,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":27944,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":28432,"parent_record_offset":28256,"ancestor_record_offset":27944,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":21,"record_offset":28740,"parent_record_offset":28712,"ancestor_record_offset":27944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":28192,"parent_record_offset":28164,"ancestor_record_offset":27944,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":28164,"parent_record_offset":28136,"ancestor_record_offset":27944,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":28108,"parent_record_offset":28080,"ancestor_record_offset":27944,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":28136,"parent_record_offset":28108,"ancestor_record_offset":27944,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":21,"record_offset":28080,"parent_record_offset":28052,"ancestor_record_offset":27944,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":21,"record_offset":28052,"parent_record_offset":28024,"ancestor_record_offset":27944,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":21,"record_offset":28024,"parent_record_offset":27996,"ancestor_record_offset":27944,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":21,"record_offset":28228,"parent_record_offset":28192,"ancestor_record_offset":27944,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":21,"record_offset":28492,"parent_record_offset":28468,"ancestor_record_offset":27944,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":21,"record_offset":28580,"parent_record_offset":28556,"ancestor_record_offset":27944,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":21,"record_offset":28468,"parent_record_offset":28192,"ancestor_record_offset":27944,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":27944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":21,"record_offset":28388,"parent_record_offset":28360,"ancestor_record_offset":27944,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00021.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$7
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C65
// 首条位置: {"module":145,"inlinee":11365}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":16248,"parent_record_offset":16220,"ancestor_record_offset":16220,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":16332,"parent_record_offset":16220,"ancestor_record_offset":16220,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":145,"record_offset":16296,"parent_record_offset":16272,"ancestor_record_offset":16220,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":16220,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C66
// 首条位置: {"module":145,"inlinee":11366}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":16480,"parent_record_offset":16444,"ancestor_record_offset":16444,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":16548,"parent_record_offset":16444,"ancestor_record_offset":16444,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":16772,"parent_record_offset":16748,"ancestor_record_offset":16444,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":16748,"parent_record_offset":16692,"ancestor_record_offset":16444,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":16720,"parent_record_offset":16692,"ancestor_record_offset":16444,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":16692,"parent_record_offset":16668,"ancestor_record_offset":16444,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":16444,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/panic_reporter/mod.rs
//     evidence={"module":145,"record_offset":16512,"parent_record_offset":16480,"ancestor_record_offset":16444,"inlinee":11368,"source_line":136,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C68
// 首条位置: {"module":145,"inlinee":11368}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
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
// 函数: tiny_glade::panic_reporter::panic_handler
// 信息 ID: 5a22e0309c266810d35a08ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::setup_panic_hook::closure$0; module=145, inline_record=11500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash
// 信息 ID: f18f83c47c777937e3e19497
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1> @ 0x1a99f1 -> 0x278ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0x2788ae -> 0x278ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$0
// 信息 ID: 2ef7ebeed36747a4fee5c56a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=14368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$1
// 信息 ID: b3f99d2a0928f4cf2d6f76e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=14572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$2
// 信息 ID: fae4f21b62314d9d7c1a04c0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=14776; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$3
// 信息 ID: 93a9ebf0a6b9aeb3e4f38f4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=14980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$4
// 信息 ID: 3edd35103d3e73c1630b39da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=15184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$5
// 信息 ID: d1a3d26f68c4d208710d88b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5 <- core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<sentry_types::protocol::v7::Breadcrumb,core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::vec::impl$19::extend_trusted::closure_env$0<sentry_types::protocol::v7::Breadcrumb,alloc::alloc::Global,core::iter::adapters::map::Map<core::option::IntoIter<ref$<anyhow::Error> >,tiny_glade::panic_reporter::recover_game_state_after_crash::closure_env$5> > > >; module=21, inline_record=27944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$7
// 信息 ID: a7ee2d008ebecc9efd6fe1f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=16220; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8
// 信息 ID: 0ff7f49663cdfe14d0980e49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=16444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::closure$8::closure$0
// 信息 ID: 8b51b900377405edbd1995b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "tiny_glade::panic_reporter::recover_game_state_after_crash", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::panic_reporter::recover_game_state_after_crash; module=145, inline_record=16512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::recover_game_state_after_crash::logging_map_error
// 信息 ID: 76a9947caa6680ae47e264cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278d22 -> 0x279880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278dcf -> 0x279880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278e7c -> 0x279880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278f29 -> 0x279880 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::recover_game_state_after_crash @ 0x278fd6 -> 0x279880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook
// 信息 ID: 3e57cbe2424ddafaa4292546
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x39a14c -> 0x2784c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::panic_reporter::setup_panic_hook::closure$0
// 信息 ID: a3956bfb55d4003d60bcef83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::panic_reporter::setup_panic_hook", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<core::panic::panic_info::PanicInfo> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::panic_reporter::setup_panic_hook::closure_env$0 <- core::ptr::drop_in_place<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 2 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<tiny_glade::panic_reporter::setup_panic_hook::closure_env$0,tuple$<ref$<core::panic::panic_info::PanicInfo> > > @ 0x2777f6 -> 0x2785c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
