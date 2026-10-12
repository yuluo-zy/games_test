// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/crash_pickup/client.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: abf83b348d08b2741bb3ed4c9af8f94704dce0a88fc5bd50f999241dd3d6c8ff
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: client.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 31, 181
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use crash_handler; // namespace candidate
//   package identities: [{"name":"crash-handler","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c","version":"0.6.1"}]
// use minidumper; // namespace candidate
//   package identities: [{"name":"minidumper","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c","version":"0.8.2"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":181,"record_offset":74064,"rva":4759920,"va":5373469040}
// 编译实例 RVAs: 0x48a170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$minidumper..ipc..windows..Socket$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2876990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e83ee62b00","size":5,"soft_fail":false,"target_va":5376346592,"va":5373469597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402e5f70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1721259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e855bce5ff","size":5,"soft_fail":false,"target_va":5371748208,"va":5373469462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140538f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t716172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e88ced0a00","size":5,"soft_fail":false,"target_va":5374185264,"va":5373469087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1612247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e8d7991800","size":5,"soft_fail":false,"target_va":5375081488,"va":5373469236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t38071457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e8a1ec4402","size":5,"soft_fail":false,"target_va":5411540992,"va":5373469530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9214164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e8d4988c00","size":5,"soft_fail":false,"target_va":5382683488,"va":5373469319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidumper::errors::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e881faffff","size":5,"soft_fail":false,"target_va":5373467776,"va":5373469178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_handler::windows::CrashHandler::attach
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"crash-handler","version":"0.6.1","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     evidence={"assembly":"\tcallq\t2890750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e8fe1b2c00","size":5,"soft_fail":false,"target_va":5376360160,"va":5373469405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9222025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e889b78c00","size":5,"soft_fail":false,"target_va":5382691200,"va":5373469170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2105311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373469040,"raw_hex":"e821e0dfff","size":5,"soft_fail":false,"target_va":5371363984,"va":5373469290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*47219063(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5373469040,"raw_hex":"ff157781d002","size":6,"soft_fail":false,"va":5373469299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":74696,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":181,"record_offset":74784,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":181,"record_offset":74272,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":181,"record_offset":76112,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] crash_handler::make_crash_event
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"crash-handler","version":"0.6.1","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/crash-handler/src/lib.rs
//     evidence={"module":181,"record_offset":75672,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":14128,"source_line":124,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::with_name
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":181,"record_offset":74208,"parent_record_offset":74064,"ancestor_record_offset":74064,"inlinee":8163,"source_line":24,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":74472,"parent_record_offset":74448,"ancestor_record_offset":74064,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":74496,"parent_record_offset":74472,"ancestor_record_offset":74064,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":74416,"parent_record_offset":74380,"ancestor_record_offset":74064,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":74448,"parent_record_offset":74416,"ancestor_record_offset":74064,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":181,"record_offset":74380,"parent_record_offset":74348,"ancestor_record_offset":74064,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":75060,"parent_record_offset":75024,"ancestor_record_offset":74064,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":74348,"parent_record_offset":74316,"ancestor_record_offset":74064,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":75024,"parent_record_offset":75000,"ancestor_record_offset":74064,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":181,"record_offset":74316,"parent_record_offset":74272,"ancestor_record_offset":74064,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":181,"record_offset":74568,"parent_record_offset":74380,"ancestor_record_offset":74064,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":181,"record_offset":75092,"parent_record_offset":75060,"ancestor_record_offset":74064,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":181,"record_offset":74524,"parent_record_offset":74496,"ancestor_record_offset":74064,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":181,"record_offset":74236,"parent_record_offset":74208,"ancestor_record_offset":74064,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":74724,"parent_record_offset":74696,"ancestor_record_offset":74064,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":76004,"parent_record_offset":75968,"ancestor_record_offset":74064,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":74748,"parent_record_offset":74724,"ancestor_record_offset":74064,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":75120,"parent_record_offset":75060,"ancestor_record_offset":74064,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":75152,"parent_record_offset":75024,"ancestor_record_offset":74064,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\os\windows\io\handle.rs
//     evidence={"module":181,"record_offset":74912,"parent_record_offset":74888,"ancestor_record_offset":74064,"inlinee":5147,"source_line":386,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::impl$6::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/mod.rs
//     evidence={"module":181,"record_offset":76136,"parent_record_offset":76112,"ancestor_record_offset":74064,"inlinee":9829,"source_line":14,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F8B
// 首条位置: {"module":31,"inlinee":8075}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":7380,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":7700,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":7636,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":8084,"source_line":607,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":31,"record_offset":7348,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":8077,"source_line":244,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":31,"record_offset":7408,"parent_record_offset":7300,"ancestor_record_offset":7300,"inlinee":8079,"source_line":186,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":7812,"parent_record_offset":7776,"ancestor_record_offset":7300,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":7300,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":7576,"parent_record_offset":7408,"ancestor_record_offset":7300,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":7752,"parent_record_offset":7728,"ancestor_record_offset":7300,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":31,"record_offset":7844,"parent_record_offset":7812,"ancestor_record_offset":7300,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":7664,"parent_record_offset":7636,"ancestor_record_offset":7300,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":7456,"parent_record_offset":7408,"ancestor_record_offset":7300,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":7604,"parent_record_offset":7408,"ancestor_record_offset":7300,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":7872,"parent_record_offset":7812,"ancestor_record_offset":7300,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":31,"record_offset":7904,"parent_record_offset":7776,"ancestor_record_offset":7300,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":31,"record_offset":7492,"parent_record_offset":7408,"ancestor_record_offset":7300,"inlinee":8081,"source_line":474,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":31,"record_offset":7516,"parent_record_offset":7492,"ancestor_record_offset":7300,"inlinee":8082,"source_line":479,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FDC
// 首条位置: {"module":31,"inlinee":8156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":31,"record_offset":34896,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":31,"record_offset":35280,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":31,"record_offset":36136,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":7173,"source_line":624,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":35252,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":35220,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":35144,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":31,"record_offset":34924,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":8079,"source_line":186,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":31,"record_offset":36604,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":8162,"source_line":166,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":31,"record_offset":35336,"parent_record_offset":34768,"ancestor_record_offset":34768,"inlinee":8157,"source_line":2185,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":35564,"parent_record_offset":35540,"ancestor_record_offset":34768,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":35588,"parent_record_offset":35564,"ancestor_record_offset":34768,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":35884,"parent_record_offset":35860,"ancestor_record_offset":34768,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":35540,"parent_record_offset":35512,"ancestor_record_offset":34768,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":31,"record_offset":35860,"parent_record_offset":35804,"ancestor_record_offset":34768,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":31,"record_offset":36296,"parent_record_offset":36268,"ancestor_record_offset":34768,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35452,"parent_record_offset":35424,"ancestor_record_offset":34768,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35832,"parent_record_offset":35804,"ancestor_record_offset":34768,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35804,"parent_record_offset":35780,"ancestor_record_offset":34768,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35512,"parent_record_offset":35480,"ancestor_record_offset":34768,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35480,"parent_record_offset":35452,"ancestor_record_offset":34768,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":31,"record_offset":35780,"parent_record_offset":35756,"ancestor_record_offset":34768,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":35396,"parent_record_offset":35336,"ancestor_record_offset":34768,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":31,"record_offset":35424,"parent_record_offset":35396,"ancestor_record_offset":34768,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::filter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":31,"record_offset":35112,"parent_record_offset":34924,"ancestor_record_offset":34768,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":31,"record_offset":35188,"parent_record_offset":35144,"ancestor_record_offset":34768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":34968,"parent_record_offset":34924,"ancestor_record_offset":34768,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":31,"record_offset":35084,"parent_record_offset":34924,"ancestor_record_offset":34768,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":31,"record_offset":35000,"parent_record_offset":34924,"ancestor_record_offset":34768,"inlinee":8081,"source_line":474,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"minidumper","version":"0.8.2","source":"git+https://github.com/PounceLight/crash-handling.git?rev=acd03d6967e428100f37454766110e82118f5d7c#acd03d6967e428100f37454766110e82118f5d7c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-17cda9e67a8ee24f/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":31,"record_offset":35024,"parent_record_offset":35000,"ancestor_record_offset":34768,"inlinee":8082,"source_line":479,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":31,"record_offset":35672,"parent_record_offset":35644,"ancestor_record_offset":34768,"inlinee":8160,"source_line":25,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":31,"record_offset":35696,"parent_record_offset":35672,"ancestor_record_offset":34768,"inlinee":8161,"source_line":49,"archive":"_pdb/B/modules/00031.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":31,"record_offset":35644,"parent_record_offset":35336,"ancestor_record_offset":34768,"inlinee":8158,"source_line":2149,"archive":"_pdb/B/modules/00031.jsonl.gz"}
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
// 完整结构及原始类型记录: client.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: client.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: client.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>", "bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>", "bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>", "bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>", "bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_decorator::backup::DecoratorBackup>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::StairsState>
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start
// 信息 ID: a4a32a0456ad234820dbdd21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::CrashReporter::spawn_server @ 0x15b28a -> 0x48a170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$0
// 信息 ID: 4a9c4ef650480090b110aa11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::thread::spawn<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; module=31, inline_record=7300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$1
// 信息 ID: 0d12b22c80a565eb25c3a122
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- core::ptr::drop_in_place<crash_handler::make_crash_event::Wrapper<tiny_glade::crash_pickup::client::start::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; module=31, inline_record=34768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
