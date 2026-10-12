// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/tiny-glade/src/crash_pickup/server.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f81cc6de2c74a28d5105a424281c058b3c604e1b2f9b264ce0ab7588794d7805
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: server.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 10, 201
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use sentry_core; // namespace candidate
//   package identities: [{"name":"sentry-core","source":"PDB_registry_path_only","version":"0.31.7"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::Handler::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14C6
// 首条位置: {"module":10,"inlinee":5318}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::Handler::on_client_message
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14B4
// 首条位置: {"module":10,"inlinee":5300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":7492,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":10,"record_offset":7464,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":7748,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":7884,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7420,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":7520,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":7692,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":10,"record_offset":8208,"parent_record_offset":7224,"ancestor_record_offset":7224,"inlinee":5307,"source_line":151,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":7224,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":8124,"parent_record_offset":8068,"ancestor_record_offset":7224,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":8096,"parent_record_offset":8068,"ancestor_record_offset":7224,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":8068,"parent_record_offset":8040,"ancestor_record_offset":7224,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":10,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":7224,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":7780,"parent_record_offset":7748,"ancestor_record_offset":7224,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":7844,"parent_record_offset":7808,"ancestor_record_offset":7224,"inlinee":5293,"source_line":2035,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":7912,"parent_record_offset":7884,"ancestor_record_offset":7224,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7628,"parent_record_offset":7544,"ancestor_record_offset":7224,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":7224,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":7224,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7224,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":7224,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":7224,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":7224,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":7224,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":7224,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":10,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":7224,"inlinee":5309,"source_line":51,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":7808,"parent_record_offset":7780,"ancestor_record_offset":7224,"inlinee":5291,"source_line":103,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31EE
// 首条位置: {"module":201,"inlinee":12782}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31E1
// 首条位置: {"module":201,"inlinee":12769}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x31E0
// 首条位置: {"module":201,"inlinee":12768}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::create_minidump_file
// 记录形式: compiled_procedure; instances=1
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":1404,"rva":88912,"va":5368798032}
// 编译实例 RVAs: 0x15b50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3951424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e8404b3c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368798283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=3
//     target_aliases=28 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6372979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e8733e6100","size":5,"soft_fail":false,"target_va":5375171056,"va":5368798072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8731676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e81c3c8500","size":5,"soft_fail":false,"target_va":5377529888,"va":5368798207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::DirBuilder::_create
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8610660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e864638300","size":5,"soft_fail":false,"target_va":5377408752,"va":5368798087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8609657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e8795f8300","size":5,"soft_fail":false,"target_va":5377408016,"va":5368798354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8641512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e8e8db8300","size":5,"soft_fail":false,"target_va":5377439776,"va":5368798259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] uuid::v4::_$LT$impl$u20$uuid..Uuid$GT$::new_v4
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7553955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798032,"raw_hex":"e8a3437300","size":5,"soft_fail":false,"target_va":5376352096,"va":5368798136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":10,"record_offset":1888,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":2980,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":2924,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":1860,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":2544,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":5265,"source_line":403,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] std::fs::create_dir_all
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":1580,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4768,"source_line":2306,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":2108,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":1968,"parent_record_offset":1404,"ancestor_record_offset":1404,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1404,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":2268,"parent_record_offset":2212,"ancestor_record_offset":1404,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":10,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1404,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":1404,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":2212,"parent_record_offset":2188,"ancestor_record_offset":1404,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":10,"record_offset":1632,"parent_record_offset":1580,"ancestor_record_offset":1404,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":1912,"parent_record_offset":1888,"ancestor_record_offset":1404,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":2140,"parent_record_offset":2108,"ancestor_record_offset":1404,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":1728,"parent_record_offset":1704,"ancestor_record_offset":1404,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1404,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1404,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":1828,"parent_record_offset":1580,"ancestor_record_offset":1404,"inlinee":4771,"source_line":2561,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::DirBuilder::recursive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":1404,"inlinee":4770,"source_line":2537,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":1404,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":2888,"parent_record_offset":2544,"ancestor_record_offset":1404,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":1704,"parent_record_offset":1680,"ancestor_record_offset":1404,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1404,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1404,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::create
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":2632,"parent_record_offset":2568,"ancestor_record_offset":1404,"inlinee":5270,"source_line":218,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::truncate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":2660,"parent_record_offset":2568,"ancestor_record_offset":1404,"inlinee":5271,"source_line":215,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":2604,"parent_record_offset":2568,"ancestor_record_offset":1404,"inlinee":5269,"source_line":209,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 记录形式: compiled_procedure; instances=1
// 源码行: 92
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":3788,"rva":89520,"va":5368798640}
// 编译实例 RVAs: 0x15db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..fs..File$u20$as$u20$std..io..Read$GT$::read_to_end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8608688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8b05b8300","size":5,"soft_fail":false,"target_va":5377407600,"va":5368798907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140307a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3087010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8a21a2f00","size":5,"soft_fail":false,"target_va":5371886224,"va":5368799209,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3515148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e80ca33500","size":5,"soft_fail":false,"target_va":5372314208,"va":5368799055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140372930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3524719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e86fc83500","size":5,"soft_fail":false,"target_va":5372324144,"va":5368799420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3626721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8e1563700","size":5,"soft_fail":false,"target_va":5372426160,"va":5368799434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3950744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e898483c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368798963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6372201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8693b6100","size":5,"soft_fail":false,"target_va":5375171056,"va":5368798850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34794061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e84dea1202","size":5,"soft_fail":false,"target_va":5403593360,"va":5368799294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e827f7ffff","size":5,"soft_fail":false,"target_va":5368797760,"va":5368800020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<minidumper::MinidumpBinary,enum2$<minidumper::errors::Error> > > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8c9fcffff","size":5,"soft_fail":false,"target_va":5368797872,"va":5368798690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34180486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8868d0902","size":5,"soft_fail":false,"target_va":5402980128,"va":5368799637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t34169831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8e7630902","size":5,"soft_fail":false,"target_va":5402968992,"va":5368799156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t34171243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e86b690902","size":5,"soft_fail":false,"target_va":5402970800,"va":5368799552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8609143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368798640,"raw_hex":"e8775d8300","size":5,"soft_fail":false,"target_va":5377408016,"va":5368798868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*42485470(%rip)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5368798640,"raw_hex":"ff15de468802","size":6,"soft_fail":false,"va":5368799084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":5464,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":5664,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":3952,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":5808,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":5492,"parent_record_offset":3788,"ancestor_record_offset":3788,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3788,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4672,"parent_record_offset":4616,"ancestor_record_offset":3788,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":3788,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":3788,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":4124,"parent_record_offset":4064,"ancestor_record_offset":3788,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":10,"record_offset":4236,"parent_record_offset":4152,"ancestor_record_offset":3788,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":5700,"parent_record_offset":5664,"ancestor_record_offset":3788,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":4028,"parent_record_offset":3992,"ancestor_record_offset":3788,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":5768,"parent_record_offset":5732,"ancestor_record_offset":3788,"inlinee":5293,"source_line":2035,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=32
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":4544,"parent_record_offset":4508,"ancestor_record_offset":3788,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":5600,"parent_record_offset":5516,"ancestor_record_offset":3788,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":5624,"parent_record_offset":5600,"ancestor_record_offset":3788,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4508,"parent_record_offset":4064,"ancestor_record_offset":3788,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4748,"parent_record_offset":4064,"ancestor_record_offset":3788,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4468,"parent_record_offset":4064,"ancestor_record_offset":3788,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":6204,"parent_record_offset":6172,"ancestor_record_offset":3788,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":5544,"parent_record_offset":5516,"ancestor_record_offset":3788,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":6228,"parent_record_offset":6204,"ancestor_record_offset":3788,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":5568,"parent_record_offset":5544,"ancestor_record_offset":3788,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":6148,"parent_record_offset":6124,"ancestor_record_offset":3788,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":3788,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":6172,"parent_record_offset":6148,"ancestor_record_offset":3788,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3788,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":3788,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3788,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4152,"parent_record_offset":4064,"ancestor_record_offset":3788,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3788,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4432,"parent_record_offset":4152,"ancestor_record_offset":3788,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":10,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":3788,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3788,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3788,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3788,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":3788,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":5732,"parent_record_offset":5700,"ancestor_record_offset":3788,"inlinee":5291,"source_line":103,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":3992,"parent_record_offset":3952,"ancestor_record_offset":3788,"inlinee":5275,"source_line":94,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":4064,"parent_record_offset":4028,"ancestor_record_offset":3788,"inlinee":5277,"source_line":95,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x149B
// 首条位置: {"module":10,"inlinee":5275}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":4028,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":3992,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4672,"parent_record_offset":4616,"ancestor_record_offset":3992,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":3992,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":3992,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":4124,"parent_record_offset":4064,"ancestor_record_offset":3992,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":10,"record_offset":4236,"parent_record_offset":4152,"ancestor_record_offset":3992,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":4544,"parent_record_offset":4508,"ancestor_record_offset":3992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4508,"parent_record_offset":4064,"ancestor_record_offset":3992,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4748,"parent_record_offset":4064,"ancestor_record_offset":3992,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4468,"parent_record_offset":4064,"ancestor_record_offset":3992,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":3992,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":3992,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":3992,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4152,"parent_record_offset":4064,"ancestor_record_offset":3992,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":3992,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4432,"parent_record_offset":4152,"ancestor_record_offset":3992,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":10,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":3992,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3992,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":3992,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":3992,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":3992,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":4064,"parent_record_offset":4028,"ancestor_record_offset":3992,"inlinee":5277,"source_line":95,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x149D
// 首条位置: {"module":10,"inlinee":5277}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":10,"record_offset":4124,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":4816,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4508,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4748,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":4468,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4152,"parent_record_offset":4064,"ancestor_record_offset":4064,"inlinee":5278,"source_line":374,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4696,"parent_record_offset":4672,"ancestor_record_offset":4064,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":4672,"parent_record_offset":4616,"ancestor_record_offset":4064,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4644,"parent_record_offset":4616,"ancestor_record_offset":4064,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4064,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":10,"record_offset":4236,"parent_record_offset":4152,"ancestor_record_offset":4064,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":4544,"parent_record_offset":4508,"ancestor_record_offset":4064,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4332,"parent_record_offset":4308,"ancestor_record_offset":4064,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4380,"parent_record_offset":4356,"ancestor_record_offset":4064,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":10,"record_offset":4356,"parent_record_offset":4332,"ancestor_record_offset":4064,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4176,"parent_record_offset":4152,"ancestor_record_offset":4064,"inlinee":5267,"source_line":925,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":10,"record_offset":4432,"parent_record_offset":4152,"ancestor_record_offset":4064,"inlinee":5272,"source_line":1130,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::os::windows::io::handle::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\os\windows\io\handle.rs
//     evidence={"module":10,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":4064,"inlinee":5199,"source_line":392,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":4064,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4284,"parent_record_offset":4260,"ancestor_record_offset":4064,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":10,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":4064,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":10,"record_offset":4204,"parent_record_offset":4176,"ancestor_record_offset":4064,"inlinee":5279,"source_line":206,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_message
// 记录形式: compiled_procedure; instances=1
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":7088,"rva":91168,"va":5368800288}
// 编译实例 RVAs: 0x16420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140293640
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2609620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8d4d12700","size":5,"soft_fail":false,"target_va":5371409984,"va":5368800359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140307a90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3085541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8e5142f00","size":5,"soft_fail":false,"target_va":5371886224,"va":5368800678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030a5a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3096128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8403e2f00","size":5,"soft_fail":false,"target_va":5371897248,"va":5368801115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030a700
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3096595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e813402f00","size":5,"soft_fail":false,"target_va":5371897600,"va":5368801000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14030adc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3098396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e81c472f00","size":5,"soft_fail":false,"target_va":5371899328,"va":5368800927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140371b00
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3518508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e82cb03500","size":5,"soft_fail":false,"target_va":5372320512,"va":5368801999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140372930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3523299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8e3c23500","size":5,"soft_fail":false,"target_va":5372324144,"va":5368800840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389e80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3618228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8b4353700","size":5,"soft_fail":false,"target_va":5372419712,"va":5368801479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038b7b0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3624599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8974e3700","size":5,"soft_fail":false,"target_va":5372426160,"va":5368801556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14038bd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3625230","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e80e513700","size":5,"soft_fail":false,"target_va":5372427520,"va":5368802285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3948558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e80e403c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368801149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=13
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34792939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8ebe51202","size":5,"soft_fail":false,"target_va":5403593360,"va":5368800416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<tiny_glade::crash_pickup::crash_payload::CrashPayload> > > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8aeeeffff","size":5,"soft_fail":false,"target_va":5368797760,"va":5368802189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t34178257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8d1840902","size":5,"soft_fail":false,"target_va":5402980128,"va":5368801866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t34168369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8315e0902","size":5,"soft_fail":false,"target_va":5402968992,"va":5368800618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=6
//     evidence={"assembly":"\tcallq\t34169767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8a7630902","size":5,"soft_fail":false,"target_va":5402970800,"va":5368801028,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::io::stdio::_eprint
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t8618221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368800288,"raw_hex":"e8ed808300","size":5,"soft_fail":false,"target_va":5377418800,"va":5368800574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::on_client_message
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":7224,"parent_record_offset":7088,"ancestor_record_offset":7088,"inlinee":5300,"source_line":35,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":8148,"parent_record_offset":8124,"ancestor_record_offset":7088,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":8124,"parent_record_offset":8068,"ancestor_record_offset":7088,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":8096,"parent_record_offset":8068,"ancestor_record_offset":7088,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":8068,"parent_record_offset":8040,"ancestor_record_offset":7088,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":7492,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":10,"record_offset":7716,"parent_record_offset":7692,"ancestor_record_offset":7088,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":10,"record_offset":7464,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":10,"record_offset":7780,"parent_record_offset":7748,"ancestor_record_offset":7088,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":7748,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":5289,"source_line":1660,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$6::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":7844,"parent_record_offset":7808,"ancestor_record_offset":7088,"inlinee":5293,"source_line":2035,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":7884,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7420,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7628,"parent_record_offset":7544,"ancestor_record_offset":7088,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":7088,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":8600,"parent_record_offset":8568,"ancestor_record_offset":7088,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7088,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":8624,"parent_record_offset":8600,"ancestor_record_offset":7088,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":7596,"parent_record_offset":7572,"ancestor_record_offset":7088,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":7520,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":7692,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":8544,"parent_record_offset":8520,"ancestor_record_offset":7088,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":7544,"parent_record_offset":7520,"ancestor_record_offset":7088,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":10,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":7088,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::api::configure_scope
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/api.rs
//     evidence={"module":10,"record_offset":8208,"parent_record_offset":7224,"ancestor_record_offset":7088,"inlinee":5307,"source_line":151,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] sentry_core::hub::Hub::with_active
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"sentry-core","version":"0.31.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/sentry-core-0.31.7/src/hub.rs
//     evidence={"module":10,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":7088,"inlinee":5309,"source_line":51,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::crash_payload::impl$10::default
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/crash_payload.rs
//     evidence={"module":10,"record_offset":7808,"parent_record_offset":7780,"ancestor_record_offset":7088,"inlinee":5291,"source_line":103,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":11044,"rva":13200,"va":5368722320}
// 编译实例 RVAs: 0x3390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":10,"record_offset":11184,"rva":93968,"va":5368803088}
// 编译实例 RVAs: 0x16f10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cea10
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2848343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e857762b00","size":5,"soft_fail":false,"target_va":5371652624,"va":5368804276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140377840
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3540479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e8ff053600","size":5,"soft_fail":false,"target_va":5372344384,"va":5368803900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140381520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3579635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e8f39e3600","size":5,"soft_fail":false,"target_va":5372384544,"va":5368804904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140389560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3612279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e8771e3700","size":5,"soft_fail":false,"target_va":5372417376,"va":5368805092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3946534","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e826383c00","size":5,"soft_fail":false,"target_va":5372749696,"va":5368803157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3946303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e83f373c00","size":5,"soft_fail":false,"target_va":5372749712,"va":5368803404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: server.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34789253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e885d71202","size":5,"soft_fail":false,"target_va":5403593360,"va":5368804102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34173648","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e8d0720902","size":5,"soft_fail":false,"target_va":5402978144,"va":5368804491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t2853811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e8b38b2b00","size":5,"soft_fail":false,"target_va":5371658128,"va":5368804312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t34174359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e897750902","size":5,"soft_fail":false,"target_va":5402978912,"va":5368804548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8550195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e833778200","size":5,"soft_fail":false,"target_va":5377353776,"va":5368803576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::server::Server::run
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4948512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e820824b00","size":5,"soft_fail":false,"target_va":5373752352,"va":5368803835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] minidumper::ipc::server::Server::with_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t62589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368803088,"raw_hex":"e87df40000","size":5,"soft_fail":false,"target_va":5368865888,"va":5368803294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":13392,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":5320,"source_line":2282,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":11364,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":10,"record_offset":13308,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":10,"record_offset":13336,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::mem::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":10,"record_offset":13768,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":13676,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":10,"record_offset":14320,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":14352,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":11884,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":11852,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":13188,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":14692,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::get_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":10,"record_offset":13712,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":5325,"source_line":241,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":10,"record_offset":13364,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":10,"record_offset":13224,"parent_record_offset":11184,"ancestor_record_offset":11184,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":11504,"parent_record_offset":11480,"ancestor_record_offset":11184,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":11528,"parent_record_offset":11504,"ancestor_record_offset":11184,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12176,"parent_record_offset":12152,"ancestor_record_offset":11184,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":11448,"parent_record_offset":11420,"ancestor_record_offset":11184,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":11480,"parent_record_offset":11448,"ancestor_record_offset":11184,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12152,"parent_record_offset":12096,"ancestor_record_offset":11184,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":10,"record_offset":11420,"parent_record_offset":11392,"ancestor_record_offset":11184,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":12124,"parent_record_offset":12096,"ancestor_record_offset":11184,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":12096,"parent_record_offset":12072,"ancestor_record_offset":11184,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":13456,"parent_record_offset":13420,"ancestor_record_offset":11184,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::is_unique
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":13420,"parent_record_offset":13392,"ancestor_record_offset":11184,"inlinee":5321,"source_line":2367,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":11184,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":12480,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":14376,"parent_record_offset":14352,"ancestor_record_offset":11184,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":10,"record_offset":13736,"parent_record_offset":13712,"ancestor_record_offset":11184,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":10,"record_offset":12396,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":10,"record_offset":12424,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":10,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":11184,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":11928,"parent_record_offset":11884,"ancestor_record_offset":11184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":10,"record_offset":13496,"parent_record_offset":13456,"ancestor_record_offset":11184,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":13820,"parent_record_offset":13792,"ancestor_record_offset":11184,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":11552,"parent_record_offset":11528,"ancestor_record_offset":11184,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":13848,"parent_record_offset":13792,"ancestor_record_offset":11184,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":12900,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":11184,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":14412,"parent_record_offset":14376,"ancestor_record_offset":11184,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":11184,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":13612,"parent_record_offset":13420,"ancestor_record_offset":11184,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":11184,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":13524,"parent_record_offset":13456,"ancestor_record_offset":11184,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":11184,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":13636,"parent_record_offset":13612,"ancestor_record_offset":11184,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":14436,"parent_record_offset":14412,"ancestor_record_offset":11184,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":14468,"parent_record_offset":14376,"ancestor_record_offset":11184,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":10,"record_offset":12872,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":5319,"source_line":258,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":10,"record_offset":12452,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":10,"record_offset":12312,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":12652,"parent_record_offset":12240,"ancestor_record_offset":11184,"inlinee":5318,"source_line":23,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::crash_pickup::server::run::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":12240,"parent_record_offset":11884,"ancestor_record_offset":11184,"inlinee":5314,"source_line":135,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::crash_pickup::server::run::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14C2
// 首条位置: {"module":10,"inlinee":5314}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":10,"record_offset":12680,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":10,"record_offset":12480,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":10,"record_offset":12396,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":10,"record_offset":12424,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":12928,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":10,"record_offset":12900,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\time.rs
//     evidence={"module":10,"record_offset":12872,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":5319,"source_line":258,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":10,"record_offset":12452,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":10,"record_offset":12312,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::crash_pickup::server::Handler::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/crash_pickup/server.rs
//     evidence={"module":10,"record_offset":12652,"parent_record_offset":12240,"ancestor_record_offset":12240,"inlinee":5318,"source_line":23,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12772,"parent_record_offset":12748,"ancestor_record_offset":12240,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12796,"parent_record_offset":12772,"ancestor_record_offset":12240,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":13120,"parent_record_offset":13096,"ancestor_record_offset":12240,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12716,"parent_record_offset":12680,"ancestor_record_offset":12240,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":12748,"parent_record_offset":12716,"ancestor_record_offset":12240,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":10,"record_offset":13096,"parent_record_offset":13072,"ancestor_record_offset":12240,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":10,"record_offset":13072,"parent_record_offset":13048,"ancestor_record_offset":12240,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":12952,"parent_record_offset":12928,"ancestor_record_offset":12240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":10,"record_offset":12824,"parent_record_offset":12796,"ancestor_record_offset":12240,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12508,"parent_record_offset":12480,"ancestor_record_offset":12240,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":12240,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12532,"parent_record_offset":12508,"ancestor_record_offset":12240,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00010.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":10,"record_offset":12360,"parent_record_offset":12336,"ancestor_record_offset":12240,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00010.jsonl.gz"}
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
// 完整结构及原始类型记录: server.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::crash_pickup::server::Handler
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "minidumper::ServerHandler::message_alloc<tiny_glade::crash_pickup::server::Handler>", "constructor": "minidumper::ServerHandler::message_alloc", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "minidumper::ServerHandler::on_client_connected<tiny_glade::crash_pickup::server::Handler>", "constructor": "minidumper::ServerHandler::on_client_connected", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tiny_glade::crash_pickup::server::Handler>", "constructor": "core::ptr::drop_in_place", "arguments": ["tiny_glade::crash_pickup::server::Handler"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: server.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: server.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::Handler::new
// 信息 ID: 16ac2ac58863da280ae0a5c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=10, inline_record=12652; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::Handler::on_client_message
// 信息 ID: 591eb9ad30a823c3b60c9197
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_message; module=10, inline_record=7224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$0
// 信息 ID: c4cc364263e2fe4e7fe853fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0 <- sentry_core::hub::Hub::with<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] sentry_core::hub::Hub::with<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >,tuple$<> >,tuple$<> >; module=201, inline_record=13944; 不是独立调用边。
// [PDB 内联宿主] sentry_core::hub::Hub::with<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$0,tuple$<> >,tuple$<> >,tuple$<> >; module=201, inline_record=14916; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$1
// 信息 ID: 71ae14912d4b422247e28f1a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- sentry_core::hub::impl$0::with_active::closure$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1 <- sentry_core::hub::Hub::with<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] sentry_core::hub::impl$0::with_active::closure$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$1,tuple$<> >,tuple$<> >; module=201, inline_record=7088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$0::on_client_message::closure$2
// 信息 ID: d0bd4ae494c56d3acbc6d79c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::crash_pickup::server::impl$0::on_client_message", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- sentry_core::hub::impl$0::with_active::closure$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- sentry_core::hub::Hub::with<sentry_core::hub::impl$0::with_active::closure_env$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2 <- sentry_core::hub_impl::HubImpl::with_mut<sentry_core::hub_impl::impl$3::with_current_scope_mut::closure_env$0<sentry_core::hub::impl$0::configure_scope::closure_env$1<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] sentry_core::hub::impl$0::with_active::closure$0<sentry_core::api::configure_scope::closure_env$0<tiny_glade::crash_pickup::server::impl$0::on_client_message::closure_env$2,tuple$<> >,tuple$<> >; module=201, inline_record=5792; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::create_minidump_file
// 信息 ID: ee64bdc978eb0df2f9aa90b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_client_disconnected
// 信息 ID: 6c04e1e082be69433766c044
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80be3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::walls::wall_attachment::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x80c7a -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::impl$0::add_decorator @ 0x503eab -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a39f -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a3aa -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0ac86 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9641 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d9665 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_roof::create_roof::create_roof @ 0x1988b07 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::set_duck_velocities_towards_goals @ 0x1b34118 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x1c43cc3 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de5b -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x1c6de63 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorators_on_glade_unload @ 0x1c6ee51 -> 0x3390 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part::closure$3 @ 0x1e6ab84 -> 0x3390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_message
// 信息 ID: bf9d8cef1d449ee54be11e88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created
// 信息 ID: 15fb84373c562b765f8e6f72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0
// 信息 ID: 0f9e1a224e4bdb2feedef042
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=10, inline_record=3992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::impl$1::on_minidump_created::closure$0::closure$0
// 信息 ID: f3c117f73654db1d615a2368
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::impl$1::on_minidump_created", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::impl$1::on_minidump_created; module=10, inline_record=4064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run
// 信息 ID: c3b091465c9765b65cc6cf17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::main @ 0x399622 -> 0x16f10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::crash_pickup::server::run::closure$0
// 信息 ID: f453e92b9513e39ee0fe633c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::crash_pickup::server::run", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::crash_pickup::server::run; module=10, inline_record=12240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
