// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/disposal.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 44bd4343b21f3da670e6d1be236d072f02c864b65d8b149d3443926557489d2f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: disposal.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 549, 566, 570, 1591, 1592, 1601, 1622, 1632
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$14::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":86972,"rva":7622432,"va":5376331552}
// 编译实例 RVAs: 0x744f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3706563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376331552,"raw_hex":"e83d71c7ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5376331582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1034659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376331552,"raw_hex":"e8a3c90f00","size":5,"soft_fail":false,"target_va":5377366224,"va":5376331560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":570,"record_offset":87100,"parent_record_offset":86972,"ancestor_record_offset":86972,"inlinee":24142,"source_line":1151,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":87392,"parent_record_offset":87368,"ancestor_record_offset":86972,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":87444,"parent_record_offset":87420,"ancestor_record_offset":86972,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":87420,"parent_record_offset":87368,"ancestor_record_offset":86972,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":570,"record_offset":87468,"parent_record_offset":87444,"ancestor_record_offset":86972,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":87328,"parent_record_offset":87304,"ancestor_record_offset":86972,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":87492,"parent_record_offset":87468,"ancestor_record_offset":86972,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":570,"record_offset":87124,"parent_record_offset":87100,"ancestor_record_offset":86972,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":87172,"parent_record_offset":87148,"ancestor_record_offset":86972,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":87256,"parent_record_offset":87228,"ancestor_record_offset":86972,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":87368,"parent_record_offset":87228,"ancestor_record_offset":86972,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":570,"record_offset":87148,"parent_record_offset":87124,"ancestor_record_offset":86972,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":570,"record_offset":87280,"parent_record_offset":87256,"ancestor_record_offset":86972,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":570,"record_offset":87304,"parent_record_offset":87280,"ancestor_record_offset":86972,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":87228,"parent_record_offset":87200,"ancestor_record_offset":86972,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":570,"record_offset":87200,"parent_record_offset":87172,"ancestor_record_offset":86972,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
// 记录形式: compiled_procedure; instances=1
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1632,"record_offset":248,"rva":18413584,"va":5387122704}
// 编译实例 RVAs: 0x118f810
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17509267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e86dd4f4fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387122782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141180af0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-60820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e86c12ffff","size":5,"soft_fail":false,"target_va":5387062000,"va":5387122815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16470501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e8e551fb00","size":5,"soft_fail":false,"target_va":5403593360,"va":5387122854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15855952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e850f1f100","size":5,"soft_fail":false,"target_va":5402978912,"va":5387122955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15840985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e8d9b6f100","size":5,"soft_fail":false,"target_va":5402963904,"va":5387122914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15842410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387122704,"raw_hex":"e86abcf100","size":5,"soft_fail":false,"target_va":5402965344,"va":5387122929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":1020,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":836,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::into_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1632,"record_offset":780,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":70856,"source_line":173,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":416,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":724,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":9721,"source_line":1660,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1632,"record_offset":596,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1632,"record_offset":748,"parent_record_offset":724,"ancestor_record_offset":248,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::into_inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1632,"record_offset":804,"parent_record_offset":780,"ancestor_record_offset":248,"inlinee":70857,"source_line":2075,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1632,"record_offset":532,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1632,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":924,"parent_record_offset":892,"ancestor_record_offset":248,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":476,"parent_record_offset":444,"ancestor_record_offset":248,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":248,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":248,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1632,"record_offset":624,"parent_record_offset":596,"ancestor_record_offset":248,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1632,"record_offset":656,"parent_record_offset":624,"ancestor_record_offset":248,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1632,"record_offset":684,"parent_record_offset":656,"ancestor_record_offset":248,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":248,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1632,"record_offset":444,"parent_record_offset":416,"ancestor_record_offset":248,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1632,"record_offset":892,"parent_record_offset":864,"ancestor_record_offset":248,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1632,"record_offset":1240,"rva":18413920,"va":5387123040}
// 编译实例 RVAs: 0x118f960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c7c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10892163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387123040,"raw_hex":"e87dcc59ff","size":5,"soft_fail":false,"target_va":5376231360,"va":5387123518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141186c10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387123040,"raw_hex":"e89071ffff","size":5,"soft_fail":false,"target_va":5387086864,"va":5387123323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15855735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387123040,"raw_hex":"e877f0f100","size":5,"soft_fail":false,"target_va":5402979152,"va":5387123412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15840516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387123040,"raw_hex":"e804b5f100","size":5,"soft_fail":false,"target_va":5402963904,"va":5387123383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15841988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387123040,"raw_hex":"e8c4baf100","size":5,"soft_fail":false,"target_va":5402965344,"va":5387123351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":1904,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":4710,"source_line":651,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":1728,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1632,"record_offset":1644,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":9750,"source_line":422,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":1408,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":1588,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":9721,"source_line":1660,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1632,"record_offset":1700,"parent_record_offset":1240,"ancestor_record_offset":1240,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1632,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":1240,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1632,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":1240,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1632,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":1240,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1632,"record_offset":1524,"parent_record_offset":1436,"ancestor_record_offset":1240,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1632,"record_offset":1548,"parent_record_offset":1524,"ancestor_record_offset":1240,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":1240,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1468,"parent_record_offset":1436,"ancestor_record_offset":1240,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1832,"parent_record_offset":1808,"ancestor_record_offset":1240,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1632,"record_offset":1492,"parent_record_offset":1468,"ancestor_record_offset":1240,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1632,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":1240,"inlinee":9703,"source_line":154,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1632,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1240,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1632,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":1240,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1632,"record_offset":1776,"parent_record_offset":1752,"ancestor_record_offset":1240,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/01632.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":10748,"rva":18321104,"va":5387030224}
// 编译实例 RVAs: 0x1178ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17416814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e8923df6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387030329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177950
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e81ce9ffff","size":5,"soft_fail":false,"target_va":5387024720,"va":5387030575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15938449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e89133f300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387030538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15940199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e8673af300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387030596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15936117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e8752af300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387030614,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15937539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030224,"raw_hex":"e80330f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387030808,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15937629","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387030224,"raw_hex":"e95d30f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387030718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":12660,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":12404,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":12576,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":11060,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":12632,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":11344,"parent_record_offset":10748,"ancestor_record_offset":10748,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":10748,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":10748,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":10748,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":10748,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":10748,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":11208,"parent_record_offset":11120,"ancestor_record_offset":10748,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":11240,"parent_record_offset":11208,"ancestor_record_offset":10748,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":11820,"parent_record_offset":11796,"ancestor_record_offset":10748,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":10748,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":10748,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":12512,"parent_record_offset":12428,"ancestor_record_offset":10748,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":12536,"parent_record_offset":12512,"ancestor_record_offset":10748,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":12740,"parent_record_offset":12708,"ancestor_record_offset":10748,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":12456,"parent_record_offset":12428,"ancestor_record_offset":10748,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":11276,"parent_record_offset":11120,"ancestor_record_offset":10748,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":12956,"parent_record_offset":12920,"ancestor_record_offset":10748,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":11152,"parent_record_offset":11120,"ancestor_record_offset":10748,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":10748,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":10748,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":11176,"parent_record_offset":11152,"ancestor_record_offset":10748,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":10748,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":10748,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":11372,"parent_record_offset":11344,"ancestor_record_offset":10748,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":11400,"parent_record_offset":11372,"ancestor_record_offset":10748,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":12028,"parent_record_offset":11888,"ancestor_record_offset":10748,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":11436,"parent_record_offset":11400,"ancestor_record_offset":10748,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":10748,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11916,"parent_record_offset":11888,"ancestor_record_offset":10748,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":10748,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":12236,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":12272,"parent_record_offset":11488,"ancestor_record_offset":10748,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11488,"parent_record_offset":11464,"ancestor_record_offset":10748,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11464,"parent_record_offset":11400,"ancestor_record_offset":10748,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11516,"parent_record_offset":11488,"ancestor_record_offset":10748,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":11768,"parent_record_offset":11736,"ancestor_record_offset":10748,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":11796,"parent_record_offset":11768,"ancestor_record_offset":10748,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":11856,"parent_record_offset":11736,"ancestor_record_offset":10748,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":11736,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11568,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":11888,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":11596,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":11680,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":12144,"parent_record_offset":11516,"ancestor_record_offset":10748,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":12684,"parent_record_offset":12660,"ancestor_record_offset":10748,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":12892,"parent_record_offset":12864,"ancestor_record_offset":10748,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":10748,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":12708,"parent_record_offset":12684,"ancestor_record_offset":10748,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":11120,"parent_record_offset":11088,"ancestor_record_offset":10748,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":11088,"parent_record_offset":11060,"ancestor_record_offset":10748,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":12920,"parent_record_offset":12892,"ancestor_record_offset":10748,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":12100,"parent_record_offset":12076,"ancestor_record_offset":10748,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":2868,"rva":18319088,"va":5387028208}
// 编译实例 RVAs: 0x11786f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17414798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e87245f6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387028313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177e50
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e8fcf5ffff","size":5,"soft_fail":false,"target_va":5387026000,"va":5387028559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15940465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e8713bf300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387028522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15942215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e84742f300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387028580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15938133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e85532f300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387028598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15939555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028208,"raw_hex":"e8e337f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387028792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15939645","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387028208,"raw_hex":"e93d38f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387028702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":4808,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":4552,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":4724,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":3208,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":4780,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":3492,"parent_record_offset":2868,"ancestor_record_offset":2868,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":4748,"parent_record_offset":4724,"ancestor_record_offset":2868,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":2868,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":2868,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":2868,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":2868,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":3356,"parent_record_offset":3268,"ancestor_record_offset":2868,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":3388,"parent_record_offset":3356,"ancestor_record_offset":2868,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":3968,"parent_record_offset":3944,"ancestor_record_offset":2868,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":4136,"parent_record_offset":4112,"ancestor_record_offset":2868,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":4112,"parent_record_offset":4088,"ancestor_record_offset":2868,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":4660,"parent_record_offset":4576,"ancestor_record_offset":2868,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":2868,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":4888,"parent_record_offset":4856,"ancestor_record_offset":2868,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":4604,"parent_record_offset":4576,"ancestor_record_offset":2868,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":3424,"parent_record_offset":3268,"ancestor_record_offset":2868,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":5104,"parent_record_offset":5068,"ancestor_record_offset":2868,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":3300,"parent_record_offset":3268,"ancestor_record_offset":2868,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":2868,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":2868,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":3324,"parent_record_offset":3300,"ancestor_record_offset":2868,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":2868,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":4200,"parent_record_offset":4176,"ancestor_record_offset":2868,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":3520,"parent_record_offset":3492,"ancestor_record_offset":2868,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":3548,"parent_record_offset":3520,"ancestor_record_offset":2868,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":4176,"parent_record_offset":4036,"ancestor_record_offset":2868,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":3584,"parent_record_offset":3548,"ancestor_record_offset":2868,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2868,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2868,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4444,"parent_record_offset":4420,"ancestor_record_offset":2868,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4384,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4420,"parent_record_offset":3636,"ancestor_record_offset":2868,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":2868,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":3612,"parent_record_offset":3548,"ancestor_record_offset":2868,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":3664,"parent_record_offset":3636,"ancestor_record_offset":2868,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":3916,"parent_record_offset":3884,"ancestor_record_offset":2868,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":2868,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":4004,"parent_record_offset":3884,"ancestor_record_offset":2868,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":3884,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":3716,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":4036,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":3744,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":3828,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":4292,"parent_record_offset":3664,"ancestor_record_offset":2868,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":2868,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":5040,"parent_record_offset":5012,"ancestor_record_offset":2868,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":4576,"parent_record_offset":4552,"ancestor_record_offset":2868,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":2868,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":3268,"parent_record_offset":3236,"ancestor_record_offset":2868,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":3236,"parent_record_offset":3208,"ancestor_record_offset":2868,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":2868,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":4248,"parent_record_offset":4224,"ancestor_record_offset":2868,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":244,"rva":18318416,"va":5387027536}
// 编译实例 RVAs: 0x1178450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17414126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e81248f6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387027641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141178150
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e89cfbffff","size":5,"soft_fail":false,"target_va":5387026768,"va":5387027887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15941137","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e8113ef300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387027850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15942887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e8e744f300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387027908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15938805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e8f534f300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387027926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15940227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387027536,"raw_hex":"e8833af300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387028120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15940317","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387027536,"raw_hex":"e9dd3af300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387028030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":2168,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":1912,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":2084,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":568,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":2140,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":852,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":244,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":1584,"parent_record_offset":1560,"ancestor_record_offset":244,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":244,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":1212,"parent_record_offset":1188,"ancestor_record_offset":244,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":244,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":716,"parent_record_offset":628,"ancestor_record_offset":244,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":748,"parent_record_offset":716,"ancestor_record_offset":244,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":244,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":244,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":244,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":2020,"parent_record_offset":1936,"ancestor_record_offset":244,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":2044,"parent_record_offset":2020,"ancestor_record_offset":244,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":2248,"parent_record_offset":2216,"ancestor_record_offset":244,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":244,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":784,"parent_record_offset":628,"ancestor_record_offset":244,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":2464,"parent_record_offset":2428,"ancestor_record_offset":244,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":660,"parent_record_offset":628,"ancestor_record_offset":244,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":2272,"parent_record_offset":2248,"ancestor_record_offset":244,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":244,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":684,"parent_record_offset":660,"ancestor_record_offset":244,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":244,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":244,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":244,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":244,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":1536,"parent_record_offset":1396,"ancestor_record_offset":244,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":944,"parent_record_offset":908,"ancestor_record_offset":244,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":244,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":244,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":244,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1744,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1780,"parent_record_offset":996,"ancestor_record_offset":244,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":996,"parent_record_offset":972,"ancestor_record_offset":244,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":972,"parent_record_offset":908,"ancestor_record_offset":244,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1024,"parent_record_offset":996,"ancestor_record_offset":244,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":1276,"parent_record_offset":1244,"ancestor_record_offset":244,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":1304,"parent_record_offset":1276,"ancestor_record_offset":244,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":1364,"parent_record_offset":1244,"ancestor_record_offset":244,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":1244,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1076,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":1396,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":1104,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":1188,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":1652,"parent_record_offset":1024,"ancestor_record_offset":244,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":244,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":2400,"parent_record_offset":2372,"ancestor_record_offset":244,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":244,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":244,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":244,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":596,"parent_record_offset":568,"ancestor_record_offset":244,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":244,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":1608,"parent_record_offset":1584,"ancestor_record_offset":244,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":8124,"rva":18320432,"va":5387029552}
// 编译实例 RVAs: 0x1178c30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17416142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e83240f6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387029657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e8dcedffff","size":5,"soft_fail":false,"target_va":5387025264,"va":5387029903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15939121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e83136f300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387029866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15940871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e8073df300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387029924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15936789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e8152df300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387029942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15938211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387029552,"raw_hex":"e8a332f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387030136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15938301","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387029552,"raw_hex":"e9fd32f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387030046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":10048,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":9792,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":9964,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":8448,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":10020,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":8732,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":9988,"parent_record_offset":9964,"ancestor_record_offset":8124,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":9464,"parent_record_offset":9440,"ancestor_record_offset":8124,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":9008,"parent_record_offset":8984,"ancestor_record_offset":8124,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":9092,"parent_record_offset":9068,"ancestor_record_offset":8124,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":9032,"parent_record_offset":9008,"ancestor_record_offset":8124,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":8596,"parent_record_offset":8508,"ancestor_record_offset":8124,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":8628,"parent_record_offset":8596,"ancestor_record_offset":8124,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":8124,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":8124,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":8124,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":9900,"parent_record_offset":9816,"ancestor_record_offset":8124,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":9924,"parent_record_offset":9900,"ancestor_record_offset":8124,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":10128,"parent_record_offset":10096,"ancestor_record_offset":8124,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":9844,"parent_record_offset":9816,"ancestor_record_offset":8124,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":8664,"parent_record_offset":8508,"ancestor_record_offset":8124,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":10344,"parent_record_offset":10308,"ancestor_record_offset":8124,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":8540,"parent_record_offset":8508,"ancestor_record_offset":8124,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":10152,"parent_record_offset":10128,"ancestor_record_offset":8124,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":8124,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":8564,"parent_record_offset":8540,"ancestor_record_offset":8124,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":8124,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":9440,"parent_record_offset":9416,"ancestor_record_offset":8124,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":8124,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":8788,"parent_record_offset":8760,"ancestor_record_offset":8124,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":9416,"parent_record_offset":9276,"ancestor_record_offset":8124,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":8824,"parent_record_offset":8788,"ancestor_record_offset":8124,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9328,"parent_record_offset":9304,"ancestor_record_offset":8124,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9304,"parent_record_offset":9276,"ancestor_record_offset":8124,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9684,"parent_record_offset":9660,"ancestor_record_offset":8124,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9624,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9660,"parent_record_offset":8876,"ancestor_record_offset":8124,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":8876,"parent_record_offset":8852,"ancestor_record_offset":8124,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":8852,"parent_record_offset":8788,"ancestor_record_offset":8124,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":8904,"parent_record_offset":8876,"ancestor_record_offset":8124,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":9156,"parent_record_offset":9124,"ancestor_record_offset":8124,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":9184,"parent_record_offset":9156,"ancestor_record_offset":8124,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":9244,"parent_record_offset":9124,"ancestor_record_offset":8124,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":9124,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":8956,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":9276,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":8984,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":9068,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":9532,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":10072,"parent_record_offset":10048,"ancestor_record_offset":8124,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":8124,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":8124,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":10096,"parent_record_offset":10072,"ancestor_record_offset":8124,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":8508,"parent_record_offset":8476,"ancestor_record_offset":8124,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":8476,"parent_record_offset":8448,"ancestor_record_offset":8124,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":10308,"parent_record_offset":10280,"ancestor_record_offset":8124,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":9488,"parent_record_offset":9464,"ancestor_record_offset":8124,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":5508,"rva":18319760,"va":5387028880}
// 编译实例 RVAs: 0x1178990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17415470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e8d242f6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387028985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177ce0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e8ecf1ffff","size":5,"soft_fail":false,"target_va":5387025632,"va":5387029231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15939793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e8d138f300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387029194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15941543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e8a73ff300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387029252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15937461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e8b52ff300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387029270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15938883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387028880,"raw_hex":"e84335f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387029464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15938973","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387028880,"raw_hex":"e99d35f300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387029374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":7424,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":7168,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":7340,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":5824,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":7396,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":6108,"parent_record_offset":5508,"ancestor_record_offset":5508,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":7364,"parent_record_offset":7340,"ancestor_record_offset":5508,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":5508,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5508,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":6468,"parent_record_offset":6444,"ancestor_record_offset":5508,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":5508,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":5972,"parent_record_offset":5884,"ancestor_record_offset":5508,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":6004,"parent_record_offset":5972,"ancestor_record_offset":5508,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":5508,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5508,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5508,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":7276,"parent_record_offset":7192,"ancestor_record_offset":5508,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":5508,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":7504,"parent_record_offset":7472,"ancestor_record_offset":5508,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":7220,"parent_record_offset":7192,"ancestor_record_offset":5508,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":6040,"parent_record_offset":5884,"ancestor_record_offset":5508,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":7720,"parent_record_offset":7684,"ancestor_record_offset":5508,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":5916,"parent_record_offset":5884,"ancestor_record_offset":5508,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":5508,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":5508,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":5940,"parent_record_offset":5916,"ancestor_record_offset":5508,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":5508,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":5508,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":5508,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":6164,"parent_record_offset":6136,"ancestor_record_offset":5508,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":6792,"parent_record_offset":6652,"ancestor_record_offset":5508,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":6200,"parent_record_offset":6164,"ancestor_record_offset":5508,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":5508,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6680,"parent_record_offset":6652,"ancestor_record_offset":5508,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":7060,"parent_record_offset":7036,"ancestor_record_offset":5508,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":7000,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":7036,"parent_record_offset":6252,"ancestor_record_offset":5508,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6252,"parent_record_offset":6228,"ancestor_record_offset":5508,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6228,"parent_record_offset":6164,"ancestor_record_offset":5508,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":5508,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":6532,"parent_record_offset":6500,"ancestor_record_offset":5508,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":5508,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":6620,"parent_record_offset":6500,"ancestor_record_offset":5508,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":6500,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6332,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":6652,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":6360,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":6444,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":6908,"parent_record_offset":6280,"ancestor_record_offset":5508,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":5508,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":7656,"parent_record_offset":7628,"ancestor_record_offset":5508,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":5508,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":5508,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":5884,"parent_record_offset":5852,"ancestor_record_offset":5508,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":5852,"parent_record_offset":5824,"ancestor_record_offset":5508,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":5508,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":6864,"parent_record_offset":6840,"ancestor_record_offset":5508,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":15976,"rva":18322448,"va":5387031568}
// 编译实例 RVAs: 0x1179410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17418158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e85238f6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387031673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411782c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e84cedffff","size":5,"soft_fail":false,"target_va":5387027136,"va":5387031919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15937105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e8512ef300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387031882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15938855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e82735f300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387031940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15934773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e83525f300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387031958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15936195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387031568,"raw_hex":"e8c32af300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387032152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15936285","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387031568,"raw_hex":"e91d2bf300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387032062,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":17896,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":17640,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":17812,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":16296,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":17868,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":16580,"parent_record_offset":15976,"ancestor_record_offset":15976,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":17836,"parent_record_offset":17812,"ancestor_record_offset":15976,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":15976,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":15976,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":16940,"parent_record_offset":16916,"ancestor_record_offset":15976,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":15976,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":16444,"parent_record_offset":16356,"ancestor_record_offset":15976,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":16476,"parent_record_offset":16444,"ancestor_record_offset":15976,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":17056,"parent_record_offset":17032,"ancestor_record_offset":15976,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":17224,"parent_record_offset":17200,"ancestor_record_offset":15976,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":17200,"parent_record_offset":17176,"ancestor_record_offset":15976,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":17748,"parent_record_offset":17664,"ancestor_record_offset":15976,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":17772,"parent_record_offset":17748,"ancestor_record_offset":15976,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":17976,"parent_record_offset":17944,"ancestor_record_offset":15976,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":17692,"parent_record_offset":17664,"ancestor_record_offset":15976,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":16512,"parent_record_offset":16356,"ancestor_record_offset":15976,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":18192,"parent_record_offset":18156,"ancestor_record_offset":15976,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":15976,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":18000,"parent_record_offset":17976,"ancestor_record_offset":15976,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":15976,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":15976,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":18216,"parent_record_offset":18192,"ancestor_record_offset":15976,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":15976,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":16608,"parent_record_offset":16580,"ancestor_record_offset":15976,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":16636,"parent_record_offset":16608,"ancestor_record_offset":15976,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":17264,"parent_record_offset":17124,"ancestor_record_offset":15976,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":16672,"parent_record_offset":16636,"ancestor_record_offset":15976,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":15976,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":15976,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":15976,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17472,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17508,"parent_record_offset":16724,"ancestor_record_offset":15976,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":16724,"parent_record_offset":16700,"ancestor_record_offset":15976,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":16700,"parent_record_offset":16636,"ancestor_record_offset":15976,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":16752,"parent_record_offset":16724,"ancestor_record_offset":15976,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":17004,"parent_record_offset":16972,"ancestor_record_offset":15976,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":17032,"parent_record_offset":17004,"ancestor_record_offset":15976,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":17092,"parent_record_offset":16972,"ancestor_record_offset":15976,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":16972,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":16804,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":17124,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":16832,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":16916,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":17380,"parent_record_offset":16752,"ancestor_record_offset":15976,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":17920,"parent_record_offset":17896,"ancestor_record_offset":15976,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":18128,"parent_record_offset":18100,"ancestor_record_offset":15976,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":17664,"parent_record_offset":17640,"ancestor_record_offset":15976,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":17944,"parent_record_offset":17920,"ancestor_record_offset":15976,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":16356,"parent_record_offset":16324,"ancestor_record_offset":15976,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":16324,"parent_record_offset":16296,"ancestor_record_offset":15976,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":18156,"parent_record_offset":18128,"ancestor_record_offset":15976,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":17336,"parent_record_offset":17312,"ancestor_record_offset":15976,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":13360,"rva":18321776,"va":5387030896}
// 编译实例 RVAs: 0x1179170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17417486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8f23af6fe","size":5,"soft_fail":false,"target_va":5369613520,"va":5387031001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177fc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8ececffff","size":5,"soft_fail":false,"target_va":5387026368,"va":5387031247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15937777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8f130f300","size":5,"soft_fail":false,"target_va":5402968992,"va":5387031210,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15939527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8c737f300","size":5,"soft_fail":false,"target_va":5402970800,"va":5387031268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15935445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8d527f300","size":5,"soft_fail":false,"target_va":5402966736,"va":5387031286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t15936867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387030896,"raw_hex":"e8632df300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387031480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t15936957","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5387030896,"raw_hex":"e9bd2df300","size":5,"soft_fail":false,"target_va":5402968352,"va":5387031390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":15276,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":15020,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":15192,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":13676,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1592,"record_offset":15248,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":70054,"source_line":201,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1592,"record_offset":13960,"parent_record_offset":13360,"ancestor_record_offset":13360,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1592,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":13360,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1592,"record_offset":14692,"parent_record_offset":14668,"ancestor_record_offset":13360,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":13360,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1592,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":13360,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1592,"record_offset":14260,"parent_record_offset":14236,"ancestor_record_offset":13360,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":13824,"parent_record_offset":13736,"ancestor_record_offset":13360,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":13856,"parent_record_offset":13824,"ancestor_record_offset":13360,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1592,"record_offset":14436,"parent_record_offset":14412,"ancestor_record_offset":13360,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":13360,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1592,"record_offset":14580,"parent_record_offset":14556,"ancestor_record_offset":13360,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":15128,"parent_record_offset":15044,"ancestor_record_offset":13360,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1592,"record_offset":15152,"parent_record_offset":15128,"ancestor_record_offset":13360,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":15356,"parent_record_offset":15324,"ancestor_record_offset":13360,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":15072,"parent_record_offset":15044,"ancestor_record_offset":13360,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":13892,"parent_record_offset":13736,"ancestor_record_offset":13360,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":15572,"parent_record_offset":15536,"ancestor_record_offset":13360,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":13768,"parent_record_offset":13736,"ancestor_record_offset":13360,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":13360,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":13916,"parent_record_offset":13892,"ancestor_record_offset":13360,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":13792,"parent_record_offset":13768,"ancestor_record_offset":13360,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1592,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":13360,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1592,"record_offset":14668,"parent_record_offset":14644,"ancestor_record_offset":13360,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":13988,"parent_record_offset":13960,"ancestor_record_offset":13360,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":14016,"parent_record_offset":13988,"ancestor_record_offset":13360,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":14644,"parent_record_offset":14504,"ancestor_record_offset":13360,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1592,"record_offset":14052,"parent_record_offset":14016,"ancestor_record_offset":13360,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14556,"parent_record_offset":14532,"ancestor_record_offset":13360,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14532,"parent_record_offset":14504,"ancestor_record_offset":13360,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14912,"parent_record_offset":14888,"ancestor_record_offset":13360,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14852,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14888,"parent_record_offset":14104,"ancestor_record_offset":13360,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":13360,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14080,"parent_record_offset":14016,"ancestor_record_offset":13360,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":13360,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":14384,"parent_record_offset":14352,"ancestor_record_offset":13360,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":14412,"parent_record_offset":14384,"ancestor_record_offset":13360,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":14472,"parent_record_offset":14352,"ancestor_record_offset":13360,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1592,"record_offset":14352,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14184,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1592,"record_offset":14504,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":14212,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":14296,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1592,"record_offset":14760,"parent_record_offset":14132,"ancestor_record_offset":13360,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1592,"record_offset":15300,"parent_record_offset":15276,"ancestor_record_offset":13360,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1592,"record_offset":15508,"parent_record_offset":15480,"ancestor_record_offset":13360,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":13360,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1592,"record_offset":15324,"parent_record_offset":15300,"ancestor_record_offset":13360,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":13736,"parent_record_offset":13704,"ancestor_record_offset":13360,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":13704,"parent_record_offset":13676,"ancestor_record_offset":13360,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1592,"record_offset":15536,"parent_record_offset":15508,"ancestor_record_offset":13360,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1592,"record_offset":14716,"parent_record_offset":14692,"ancestor_record_offset":13360,"inlinee":70052,"source_line":31,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$15::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7907
// 首条位置: {"module":570,"inlinee":30983}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1592,"record_offset":18596,"rva":18323120,"va":5387032240}
// 编译实例 RVAs: 0x11796b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c9460
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15401647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e851fd14ff","size":5,"soft_fail":false,"target_va":5371630688,"va":5387032330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c7c0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10801075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e84d305bff","size":5,"soft_fail":false,"target_va":5376231360,"va":5387032430,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140741d00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10713706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e896855cff","size":5,"soft_fail":false,"target_va":5376318720,"va":5387032421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::collections::vec_deque::VecDeque$LT$T$C$A$GT$::resize_with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7990149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e885eb7900","size":5,"soft_fail":false,"target_va":5395022464,"va":5387032310,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16193532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e8fc17f700","size":5,"soft_fail":false,"target_va":5403226224,"va":5387032687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15946436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e8c452f300","size":5,"soft_fail":false,"target_va":5402979152,"va":5387032711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t90289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387032240,"raw_hex":"e8b1600100","size":5,"soft_fail":false,"target_va":5387122704,"va":5387032410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::front
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19348,"parent_record_offset":18596,"ancestor_record_offset":18596,"inlinee":22347,"source_line":1520,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::pop_front
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19184,"parent_record_offset":18596,"ancestor_record_offset":18596,"inlinee":10716,"source_line":1610,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::vec_deque::VecDeque::push_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":18836,"parent_record_offset":18596,"ancestor_record_offset":18596,"inlinee":10291,"source_line":1685,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1592,"record_offset":19144,"parent_record_offset":18596,"ancestor_record_offset":18596,"inlinee":70056,"source_line":167,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19696,"parent_record_offset":19568,"ancestor_record_offset":18596,"inlinee":10717,"source_line":165,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::buffer_write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19048,"parent_record_offset":18836,"ancestor_record_offset":18596,"inlinee":6132,"source_line":171,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":18892,"parent_record_offset":18868,"ancestor_record_offset":18596,"inlinee":7711,"source_line":751,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19380,"parent_record_offset":19348,"ancestor_record_offset":18596,"inlinee":7687,"source_line":667,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19220,"parent_record_offset":19184,"ancestor_record_offset":18596,"inlinee":11193,"source_line":1254,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::is_full
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":18868,"parent_record_offset":18836,"ancestor_record_offset":18596,"inlinee":7710,"source_line":188,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19072,"parent_record_offset":19048,"ancestor_record_offset":18596,"inlinee":7712,"source_line":159,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::to_physical_idx
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":18952,"parent_record_offset":18836,"ancestor_record_offset":18596,"inlinee":7688,"source_line":200,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::VecDeque::wrap_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":18596,"inlinee":7689,"source_line":195,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::vec_deque::wrap_index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\vec_deque\mod.rs
//     evidence={"module":1592,"record_offset":19524,"parent_record_offset":19500,"ancestor_record_offset":18596,"inlinee":11291,"source_line":2679,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1592,"record_offset":18916,"parent_record_offset":18892,"ancestor_record_offset":18596,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1592,"record_offset":19104,"parent_record_offset":19072,"ancestor_record_offset":18596,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1592,"record_offset":19000,"parent_record_offset":18976,"ancestor_record_offset":18596,"inlinee":7690,"source_line":1639,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1592,"record_offset":19720,"parent_record_offset":19696,"ancestor_record_offset":18596,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01592.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$17::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76E4
// 首条位置: {"module":549,"inlinee":30436}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::DisposalBucket::dispose
// 记录形式: inline_instance, inlinee_declaration; instances=23
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7851
// 首条位置: {"module":566,"inlinee":30801}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$10::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":6696,"inlinee":70032,"source_line":245,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$11::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":7504,"parent_record_offset":7480,"ancestor_record_offset":7480,"inlinee":70037,"source_line":251,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$12::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":8884,"parent_record_offset":8860,"ancestor_record_offset":8860,"inlinee":70042,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$13::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":3592,"parent_record_offset":3564,"ancestor_record_offset":3564,"inlinee":30807,"source_line":263,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$4::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":3388,"inlinee":30805,"source_line":209,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$5::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":2580,"inlinee":30803,"source_line":215,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$6::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1601,"record_offset":520,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":70060,"source_line":221,"archive":"_pdb/A/modules/01601.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$7::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":23724,"parent_record_offset":23700,"ancestor_record_offset":23700,"inlinee":70058,"source_line":227,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$8::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1622,"record_offset":24344,"parent_record_offset":24312,"ancestor_record_offset":24312,"inlinee":70062,"source_line":233,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::impl$9::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":1591,"record_offset":5960,"parent_record_offset":5936,"ancestor_record_offset":5936,"inlinee":70027,"source_line":239,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":2580,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2672,"parent_record_offset":2636,"ancestor_record_offset":2580,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":2580,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":2728,"parent_record_offset":2636,"ancestor_record_offset":2580,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket
// 记录形式: compiled_procedure; instances=1
// 源码行: 151
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":56208,"rva":7609600,"va":5376318720}
// 编译实例 RVAs: 0x741d00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3570994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e8ce82c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376320701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-89774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e852a1feff","size":5,"soft_fail":false,"target_va":5376231008,"va":5376320777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072de40
//     owner=unresolved_namespace; count=9
//     target_aliases=10 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e8cab8feff","size":5,"soft_fail":false,"target_va":5376237120,"va":5376320881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072dea0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: disposal.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-83216","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e8f0bafeff","size":5,"soft_fail":false,"target_va":5376237216,"va":5376320427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26659610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e81acb9601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376320513,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::free
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t81879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e8d73f0100","size":5,"soft_fail":false,"target_va":5376402256,"va":5376320372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1034541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e82dc90f00","size":5,"soft_fail":false,"target_va":5377353776,"va":5376319230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26648695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e877a09601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376320292,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26650382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e80ea79601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376320413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376318720,"raw_hex":"e855f5ffff","size":5,"soft_fail":false,"target_va":5376316512,"va":5376319238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t*440(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376318720,"raw_hex":"ff96b8010000","size":6,"soft_fail":false,"va":5376318872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":61720,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56440,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":57984,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":570,"record_offset":56692,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":57956,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31001,"source_line":1242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":61260,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31007,"source_line":1298,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_framebuffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":59700,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31004,"source_line":1200,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_image
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":57440,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31000,"source_line":1148,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_image_view
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":56924,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":30999,"source_line":1172,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_render_pass
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":60220,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31005,"source_line":1186,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_sampler
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":60740,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31006,"source_line":1000,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::destroy_semaphore
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":59184,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":30845,"source_line":1284,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::swapchain::Device::destroy_swapchain
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/khr/swapchain.rs
//     evidence={"module":570,"record_offset":58668,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":31003,"source_line":31,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":58096,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":58124,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":62276,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":62004,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":61748,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":61920,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":58152,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":58012,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::allocator::GpuAllocator::free
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/allocator.rs
//     evidence={"module":570,"record_offset":61976,"parent_record_offset":56208,"ancestor_record_offset":56208,"inlinee":30567,"source_line":72,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":62220,"parent_record_offset":62196,"ancestor_record_offset":56208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":62196,"parent_record_offset":62140,"ancestor_record_offset":56208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":62168,"parent_record_offset":62140,"ancestor_record_offset":56208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":56504,"parent_record_offset":56480,"ancestor_record_offset":56208,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":62140,"parent_record_offset":62112,"ancestor_record_offset":56208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56480,"parent_record_offset":56440,"ancestor_record_offset":56208,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56536,"parent_record_offset":56440,"ancestor_record_offset":56208,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":56564,"parent_record_offset":56440,"ancestor_record_offset":56208,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":570,"record_offset":56864,"parent_record_offset":56840,"ancestor_record_offset":56208,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":61944,"parent_record_offset":61920,"ancestor_record_offset":56208,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":56840,"parent_record_offset":56692,"ancestor_record_offset":56208,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":62040,"parent_record_offset":62004,"ancestor_record_offset":56208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":56648,"parent_record_offset":56620,"ancestor_record_offset":56208,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":56804,"parent_record_offset":56780,"ancestor_record_offset":56208,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":56752,"parent_record_offset":56716,"ancestor_record_offset":56208,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":56884,"parent_record_offset":56864,"ancestor_record_offset":56208,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":61856,"parent_record_offset":61772,"ancestor_record_offset":56208,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":61880,"parent_record_offset":61856,"ancestor_record_offset":56208,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":56592,"parent_record_offset":56440,"ancestor_record_offset":56208,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":56620,"parent_record_offset":56592,"ancestor_record_offset":56208,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":56780,"parent_record_offset":56716,"ancestor_record_offset":56208,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":56716,"parent_record_offset":56692,"ancestor_record_offset":56208,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":62356,"parent_record_offset":62324,"ancestor_record_offset":56208,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":61800,"parent_record_offset":61772,"ancestor_record_offset":56208,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":58036,"parent_record_offset":58012,"ancestor_record_offset":56208,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":62380,"parent_record_offset":62356,"ancestor_record_offset":56208,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":61824,"parent_record_offset":61800,"ancestor_record_offset":56208,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":58060,"parent_record_offset":58036,"ancestor_record_offset":56208,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":62300,"parent_record_offset":62276,"ancestor_record_offset":56208,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":61772,"parent_record_offset":61748,"ancestor_record_offset":56208,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":62324,"parent_record_offset":62300,"ancestor_record_offset":56208,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$4::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 209
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7855
// 首条位置: {"module":566,"inlinee":30805}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":3436,"parent_record_offset":3412,"ancestor_record_offset":3412,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3412,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":3468,"parent_record_offset":3436,"ancestor_record_offset":3412,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":3524,"parent_record_offset":3436,"ancestor_record_offset":3412,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$5::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 215
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7853
// 首条位置: {"module":566,"inlinee":30803}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":2608,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":2608,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2672,"parent_record_offset":2636,"ancestor_record_offset":2608,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":2728,"parent_record_offset":2636,"ancestor_record_offset":2608,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$6::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111AC
// 首条位置: {"module":1592,"inlinee":70060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1601,"record_offset":544,"parent_record_offset":520,"ancestor_record_offset":520,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01601.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1601,"record_offset":600,"parent_record_offset":576,"ancestor_record_offset":520,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01601.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1601,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":520,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01601.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1601,"record_offset":632,"parent_record_offset":544,"ancestor_record_offset":520,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01601.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$7::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111AA
// 首条位置: {"module":1592,"inlinee":70058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23748,"parent_record_offset":23724,"ancestor_record_offset":23724,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":23724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":23784,"parent_record_offset":23748,"ancestor_record_offset":23724,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":23840,"parent_record_offset":23748,"ancestor_record_offset":23724,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$8::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111AE
// 首条位置: {"module":1592,"inlinee":70062}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":24376,"parent_record_offset":24344,"ancestor_record_offset":24344,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1622,"record_offset":24444,"parent_record_offset":24420,"ancestor_record_offset":24344,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1622,"record_offset":24420,"parent_record_offset":24376,"ancestor_record_offset":24344,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1622,"record_offset":24476,"parent_record_offset":24376,"ancestor_record_offset":24344,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01622.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$9::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1118B
// 首条位置: {"module":1591,"inlinee":70027}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":5984,"parent_record_offset":5960,"ancestor_record_offset":5960,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":5960,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6016,"parent_record_offset":5984,"ancestor_record_offset":5960,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":6072,"parent_record_offset":5984,"ancestor_record_offset":5960,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$10::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 245
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11190
// 首条位置: {"module":1591,"inlinee":70032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6744,"parent_record_offset":6720,"ancestor_record_offset":6720,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6720,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":6776,"parent_record_offset":6744,"ancestor_record_offset":6720,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":6832,"parent_record_offset":6744,"ancestor_record_offset":6720,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$11::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 251
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11195
// 首条位置: {"module":1591,"inlinee":70037}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":7528,"parent_record_offset":7504,"ancestor_record_offset":7504,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":7504,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":7560,"parent_record_offset":7528,"ancestor_record_offset":7504,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":7616,"parent_record_offset":7528,"ancestor_record_offset":7504,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$12::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 257
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1119A
// 首条位置: {"module":1591,"inlinee":70042}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":8884,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1591,"record_offset":8964,"parent_record_offset":8940,"ancestor_record_offset":8884,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1591,"record_offset":8940,"parent_record_offset":8908,"ancestor_record_offset":8884,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1591,"record_offset":8996,"parent_record_offset":8908,"ancestor_record_offset":8884,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01591.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::disposal::impl$13::dispose_to_bucket
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 263
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7857
// 首条位置: {"module":566,"inlinee":30807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":3620,"parent_record_offset":3592,"ancestor_record_offset":3592,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":3700,"parent_record_offset":3668,"ancestor_record_offset":3592,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":3668,"parent_record_offset":3620,"ancestor_record_offset":3592,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":3732,"parent_record_offset":3620,"ancestor_record_offset":3592,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
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
// 完整结构及原始类型记录: disposal.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::CommandBufferDisposalBuckets
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::DisposalBucket
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 18
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::disposal::DisposalBucket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::disposal::DisposalBucket>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::disposal::DisposalBucket"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::SequentialDisposal
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>", "constructor": "rhapsody::raw::disposal::SequentialDisposal::dispose_in_order", "arguments": ["rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::disposal::SequentialDisposalItem
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::resize_with<rhapsody::raw::disposal::SequentialDisposalItem,alloc::alloc::Global,rhapsody::raw::disposal::SequentialDisposalItem (*)()>", "constructor": "alloc::collections::vec_deque::VecDeque::resize_with", "arguments": ["rhapsody::raw::disposal::SequentialDisposalItem", "alloc::alloc::Global", "rhapsody::raw::disposal::SequentialDisposalItem (*)()"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::disposal::SequentialDisposalItem,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::raw::disposal::SequentialDisposalItem", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: disposal.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: disposal.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::immediate_empty_bucket
// 信息 ID: 15872f2cc2e9b512574cafc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste @ 0x728e1d -> 0x741d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x1179765 -> 0x741d00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x1179828 -> 0x741d00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >
// 信息 ID: 205d251c64ca71577b7ba339
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::buffer::ManagedBuffer"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >
// 信息 ID: 7f2c2b7fed25b2c235f411fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >
// 信息 ID: 69d187cc17404be4fb9192f3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::framebuffer::ManagedFramebuffer"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >
// 信息 ID: 19bfca570336a2aea26f5052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::render_pass::ManagedRenderPass"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >
// 信息 ID: 74341776f9043343a0a73d9f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::sampler::ManagedSampler"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >
// 信息 ID: a9dc97b222027443e4f98afc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::swapchain::ManagedSwapchain"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >
// 信息 ID: aa3d3ffe262a0911ea6245b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0 -> ["rhapsody::raw::managed::texture::ManagedTexture"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource -> ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste @ 0x728d16 -> 0x1179170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert
// 信息 ID: 1f826be7dec37198c896d4bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::command_buffer::CommandBuffer::begin @ 0x745587 -> 0x118f960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::CommandBufferDisposalBuckets::remove
// 信息 ID: bfb1b80e512b93a162bf8333
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x117975a -> 0x118f810 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0> @ 0x117981d -> 0x118f810 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::DisposalBucket::dispose
// 信息 ID: c63a794665e6d9220ea54878
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=2580; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3388; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3564; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3800; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1591, inline_record=5936; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1591, inline_record=6696; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1591, inline_record=7480; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1591, inline_record=8860; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::SequentialDisposal::dispose_in_order<rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0>
// 信息 ID: 0807f0c18486db868a6f6168
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::raw::disposal::SequentialDisposal::dispose_in_order -> ["rhapsody::raw::device::impl$1::reclaim_command_buffer_resources::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::Queue::reclaim_command_buffer_resources @ 0x73b791 -> 0x11796b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$10::dispose_to_bucket
// 信息 ID: a74538030ce6f9bb81e1ae76
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::sampler::ManagedSampler>; module=1591, inline_record=6720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$11::dispose_to_bucket
// 信息 ID: 888c6e4b24a1d580e74ce744
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1591, inline_record=7504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$12::dispose_to_bucket
// 信息 ID: 21a49e8388a5d4a5b94caadb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::framebuffer::ManagedFramebuffer>; module=1591, inline_record=8884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$13::dispose_to_bucket
// 信息 ID: d47a690b992f5c79fc74446b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3592; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::buffer::impl$0::dispose; module=1601, inline_record=700; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$14::default
// 信息 ID: 6a740b3015029b2dae06e82b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73f085 -> 0x744f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$15::default
// 信息 ID: 380e65749140725d0c1ed753
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=43516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$17::default
// 信息 ID: 37998b97e2c13cb764619d3f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$4::dispose_to_bucket
// 信息 ID: dd5b3bfbeddd7cfc86fd43d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$5::dispose_to_bucket
// 信息 ID: f382c53616104cf08e6fe624
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=2608; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::texture::impl$0::dispose; module=566, inline_record=3824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$6::dispose_to_bucket
// 信息 ID: 088e439571906fde588ede88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::buffer::impl$0::dispose; module=1601, inline_record=520; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$7::dispose_to_bucket
// 信息 ID: 2d839b85d2f77c06256b3160
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1622, inline_record=23724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$8::dispose_to_bucket
// 信息 ID: f12e0c4d13e4a9f526efc80a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::swapchain::impl$1::dispose; module=1622, inline_record=24344; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::disposal::impl$9::dispose_to_bucket
// 信息 ID: 0a6901c3ff62095ba06695b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1591, inline_record=5960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
