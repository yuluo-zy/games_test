// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/crash_pickup/client.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 458136acc8420567b119e0dba5eda7dd8a44d2f3ae6d4b9b66c8d5ea34384d95
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: client.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 14, 107, 116
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use crash_handler; // namespace candidate
//   package identities: []
// use minidumper; // namespace candidate
//   package identities: []
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start
// 记录形式: compiled_procedure; instances=1
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":107,"record_offset":14224,"rva":1535408,"va":5370244528}
// 编译实例 RVAs: 0x176db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$minidumper..ipc..windows..Socket$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3517371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8bbab3500","size":5,"soft_fail":false,"target_va":5373762432,"va":5370245056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14037fc40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2133381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8858d2000","size":5,"soft_fail":false,"target_va":5372378176,"va":5370244790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: client.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2504967","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e807392600","size":5,"soft_fail":false,"target_va":5372749696,"va":5370244724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32733154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8e277f301","size":5,"soft_fail":false,"target_va":5402978144,"va":5370244985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t1406393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8b9751500","size":5,"soft_fail":false,"target_va":5371651312,"va":5370244914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<minidumper::errors::Error> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e821fbffff","size":5,"soft_fail":false,"target_va":5370243424,"va":5370244666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] crash_handler::windows::CrashHandler::attach
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3523554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8e2c33500","size":5,"soft_fail":false,"target_va":5373768416,"va":5370244857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::windows::UnixStream::connect
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t1314998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8b6101400","size":5,"soft_fail":false,"target_va":5371559584,"va":5370244581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::sleep
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7148537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e8f9136d00","size":5,"soft_fail":false,"target_va":5377393200,"va":5370244658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t422718","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370244528,"raw_hex":"e83e730600","size":5,"soft_fail":false,"target_va":5370667504,"va":5370244781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":107,"record_offset":14856,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":15496,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":107,"record_offset":14432,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":107,"record_offset":15468,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] crash_handler::make_crash_event
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/crash-handler/src/lib.rs
//     evidence={"module":107,"record_offset":15028,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":9959,"source_line":124,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::with_name
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":107,"record_offset":14368,"parent_record_offset":14224,"ancestor_record_offset":14224,"inlinee":5915,"source_line":24,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":14632,"parent_record_offset":14608,"ancestor_record_offset":14224,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":14656,"parent_record_offset":14632,"ancestor_record_offset":14224,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":14576,"parent_record_offset":14540,"ancestor_record_offset":14224,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":107,"record_offset":14608,"parent_record_offset":14576,"ancestor_record_offset":14224,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":107,"record_offset":14540,"parent_record_offset":14508,"ancestor_record_offset":14224,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":107,"record_offset":15556,"parent_record_offset":15520,"ancestor_record_offset":14224,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":107,"record_offset":14508,"parent_record_offset":14476,"ancestor_record_offset":14224,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":107,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":14224,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":107,"record_offset":14476,"parent_record_offset":14432,"ancestor_record_offset":14224,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":14728,"parent_record_offset":14540,"ancestor_record_offset":14224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":107,"record_offset":15588,"parent_record_offset":15556,"ancestor_record_offset":14224,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":107,"record_offset":14684,"parent_record_offset":14656,"ancestor_record_offset":14224,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":107,"record_offset":14396,"parent_record_offset":14368,"ancestor_record_offset":14224,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14884,"parent_record_offset":14856,"ancestor_record_offset":14224,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15360,"parent_record_offset":15324,"ancestor_record_offset":14224,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":14908,"parent_record_offset":14884,"ancestor_record_offset":14224,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":14224,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":107,"record_offset":15416,"parent_record_offset":15324,"ancestor_record_offset":14224,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00107.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 32
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15C6
// 首条位置: {"module":14,"inlinee":5574}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":11928,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":12276,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":12212,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":14,"record_offset":11900,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":5319,"source_line":258,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":14,"record_offset":11956,"parent_record_offset":11852,"ancestor_record_offset":11852,"inlinee":5575,"source_line":186,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":12360,"parent_record_offset":12324,"ancestor_record_offset":11852,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":14,"record_offset":12324,"parent_record_offset":12300,"ancestor_record_offset":11852,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":12124,"parent_record_offset":11956,"ancestor_record_offset":11852,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":14,"record_offset":12148,"parent_record_offset":12124,"ancestor_record_offset":11852,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":14,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":11852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":14,"record_offset":12392,"parent_record_offset":12360,"ancestor_record_offset":11852,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":12240,"parent_record_offset":12212,"ancestor_record_offset":11852,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":12004,"parent_record_offset":11956,"ancestor_record_offset":11852,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":14,"record_offset":12180,"parent_record_offset":11956,"ancestor_record_offset":11852,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":12420,"parent_record_offset":12360,"ancestor_record_offset":11852,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":14,"record_offset":12452,"parent_record_offset":12324,"ancestor_record_offset":11852,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":14,"record_offset":12040,"parent_record_offset":11956,"ancestor_record_offset":11852,"inlinee":5577,"source_line":474,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":14,"record_offset":12064,"parent_record_offset":12040,"ancestor_record_offset":11852,"inlinee":5578,"source_line":479,"archive":"_pdb/A/modules/00014.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::client::start::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x27C7
// 首条位置: {"module":116,"inlinee":10183}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":116,"record_offset":14484,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":116,"record_offset":14892,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":116,"record_offset":15712,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":116,"record_offset":14864,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":116,"record_offset":14832,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":116,"record_offset":14760,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::ping
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":116,"record_offset":14512,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":5575,"source_line":186,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] minidumper::ipc::client::Client::send_message
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/client.rs
//     evidence={"module":116,"record_offset":16176,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":5328,"source_line":166,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::to_vec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":116,"record_offset":14948,"parent_record_offset":14356,"ancestor_record_offset":14356,"inlinee":10184,"source_line":2185,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":116,"record_offset":15132,"parent_record_offset":15108,"ancestor_record_offset":14356,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":116,"record_offset":15156,"parent_record_offset":15132,"ancestor_record_offset":14356,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":116,"record_offset":15488,"parent_record_offset":15464,"ancestor_record_offset":14356,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":116,"record_offset":15108,"parent_record_offset":15080,"ancestor_record_offset":14356,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":116,"record_offset":15464,"parent_record_offset":15408,"ancestor_record_offset":14356,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":116,"record_offset":15872,"parent_record_offset":15844,"ancestor_record_offset":14356,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":116,"record_offset":15436,"parent_record_offset":15408,"ancestor_record_offset":14356,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":116,"record_offset":15080,"parent_record_offset":15052,"ancestor_record_offset":14356,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":116,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14356,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":116,"record_offset":15228,"parent_record_offset":15052,"ancestor_record_offset":14356,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":116,"record_offset":15408,"parent_record_offset":15380,"ancestor_record_offset":14356,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":116,"record_offset":14996,"parent_record_offset":14948,"ancestor_record_offset":14356,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":116,"record_offset":15024,"parent_record_offset":14996,"ancestor_record_offset":14356,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":116,"record_offset":14700,"parent_record_offset":14512,"ancestor_record_offset":14356,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":116,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":14356,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":116,"record_offset":14800,"parent_record_offset":14760,"ancestor_record_offset":14356,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":116,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":14356,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":116,"record_offset":14556,"parent_record_offset":14512,"ancestor_record_offset":14356,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":116,"record_offset":14672,"parent_record_offset":14512,"ancestor_record_offset":14356,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":116,"record_offset":14588,"parent_record_offset":14512,"ancestor_record_offset":14356,"inlinee":5577,"source_line":474,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] minidumper::ipc::windows::UnixStream::recv_vectored
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/crash-handling-ce90d79cafbe7f7c/acd03d6/minidumper/src/ipc/windows.rs
//     evidence={"module":116,"record_offset":14612,"parent_record_offset":14588,"ancestor_record_offset":14356,"inlinee":5578,"source_line":479,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":116,"record_offset":15296,"parent_record_offset":15268,"ancestor_record_offset":14356,"inlinee":10187,"source_line":25,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Serializer::with_formatter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":116,"record_offset":15320,"parent_record_offset":15296,"ancestor_record_offset":14356,"inlinee":10188,"source_line":49,"archive":"_pdb/A/modules/00116.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::to_writer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":116,"record_offset":15268,"parent_record_offset":14948,"ancestor_record_offset":14356,"inlinee":10185,"source_line":2149,"archive":"_pdb/A/modules/00116.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start
// 信息 ID: 6361fa7eedd8d16c0a7f9b0d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::crash_pickup::CrashReporter::spawn_server @ 0x3b4cfe -> 0x176db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$0
// 信息 ID: f6f70a6710fbcea6d2679573
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- std::thread::spawn<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$0 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] std::sys_common::backtrace::__rust_begin_short_backtrace<tiny_glade::crash_pickup::client::start::closure_env$0,tuple$<> >; module=14, inline_record=11852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::client::start::closure$1
// 信息 ID: 2d19de64eb74692eaf734f11
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::client::start", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::client::start::closure_env$1 <- core::ptr::drop_in_place<crash_handler::make_crash_event::Wrapper<tiny_glade::crash_pickup::client::start::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] crash_handler::make_crash_event::impl$0::on_crash<tiny_glade::crash_pickup::client::start::closure_env$1>; module=116, inline_record=14356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
