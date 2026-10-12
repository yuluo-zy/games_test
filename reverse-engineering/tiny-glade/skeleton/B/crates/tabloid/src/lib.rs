// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tabloid/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ddca6c5ca6fcb266c938eda8a3c33923911fdb23ff5e3408b5cdf9a1e9f27e56
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 308, 411, 437, 485, 491, 496, 509, 511, 559, 587, 596, 597, 1066, 1081, 1411, 1475, 1494, 1545, 1585, 1605, 2440, 2611, 2625, 3799, 3811, 4429
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use std; // namespace candidate
//   package identities: []
// use tabloid; // namespace candidate
//   package identities: [{"name":"tabloid","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tabloid::impl$0::default
// 记录形式: inline_instance, inlinee_declaration; instances=36
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x47C3
// 首条位置: {"module":308,"inlinee":18371}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>
// 记录形式: compiled_procedure; instances=2
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1545,"record_offset":656,"rva":19474144,"va":5388183264}
// 编译实例 RVAs: 0x12926e0, 0x1454410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13101870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388183264,"raw_hex":"e8d21438ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388183353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5c250
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7562710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388183264,"raw_hex":"e82a9a8cff","size":5,"soft_fail":false,"target_va":5380620880,"va":5388183585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cc90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7560027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388183264,"raw_hex":"e8a5a48cff","size":5,"soft_fail":false,"target_va":5380623504,"va":5388183526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23357517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388183264,"raw_hex":"e84d686401","size":5,"soft_fail":false,"target_va":5411540992,"va":5388183470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-13827862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388183264,"raw_hex":"e8ea002dff","size":5,"soft_fail":false,"target_va":5374355568,"va":5388183425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":868,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":1384,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1545,"record_offset":1472,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":1812,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1545,"record_offset":840,"parent_record_offset":656,"ancestor_record_offset":656,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":656,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":656,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":656,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1545,"record_offset":960,"parent_record_offset":932,"ancestor_record_offset":656,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1545,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":656,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1545,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":656,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1545,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":656,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1545,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":656,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":1872,"parent_record_offset":1836,"ancestor_record_offset":656,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":656,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1545,"record_offset":1516,"parent_record_offset":1472,"ancestor_record_offset":656,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":1076,"parent_record_offset":896,"ancestor_record_offset":656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1545,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":656,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":656,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":1628,"parent_record_offset":1472,"ancestor_record_offset":656,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":656,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":1264,"parent_record_offset":1228,"ancestor_record_offset":656,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":656,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":656,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":1320,"parent_record_offset":1228,"ancestor_record_offset":656,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=4
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1475,"record_offset":664,"rva":19168016,"va":5387877136}
// 编译实例 RVAs: 0x1247b10, 0x1292860, 0x1454290, 0x1cf68e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12795742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877136,"raw_hex":"e8a2c03cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5387877225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cb30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7254251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877136,"raw_hex":"e8154f91ff","size":5,"soft_fail":false,"target_va":5380623152,"va":5387877398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d040
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7253014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877136,"raw_hex":"e8ea5391ff","size":5,"soft_fail":false,"target_va":5380624448,"va":5387877457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23663645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877136,"raw_hex":"e81d146901","size":5,"soft_fail":false,"target_va":5411540992,"va":5387877342,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-13521734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877136,"raw_hex":"e8baac31ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5387877297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":868,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":1384,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1475,"record_offset":1472,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":1812,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::new
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1475,"record_offset":840,"parent_record_offset":664,"ancestor_record_offset":664,"inlinee":10634,"source_line":154,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1475,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":664,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1475,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":664,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1475,"record_offset":932,"parent_record_offset":896,"ancestor_record_offset":664,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1475,"record_offset":960,"parent_record_offset":932,"ancestor_record_offset":664,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1475,"record_offset":896,"parent_record_offset":868,"ancestor_record_offset":664,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1475,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":664,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1475,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":664,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1475,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":664,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":1872,"parent_record_offset":1836,"ancestor_record_offset":664,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":664,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1475,"record_offset":1516,"parent_record_offset":1472,"ancestor_record_offset":664,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":1076,"parent_record_offset":896,"ancestor_record_offset":664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1475,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":664,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":664,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":1628,"parent_record_offset":1472,"ancestor_record_offset":664,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":664,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":1264,"parent_record_offset":1228,"ancestor_record_offset":664,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":664,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":664,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":1320,"parent_record_offset":1228,"ancestor_record_offset":664,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1605,"record_offset":7104,"rva":21314560,"va":5390023680}
// 编译实例 RVAs: 0x1453c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=2
//     target_aliases=35 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20423147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8155ec8fe","size":5,"soft_fail":false,"target_va":5369600800,"va":5390023942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b650
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19958334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8c275cffe","size":5,"soft_fail":false,"target_va":5370066512,"va":5390024841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19867106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e81edad0fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5390025229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14942828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e894fd1bff","size":5,"soft_fail":false,"target_va":5375081488,"va":5390024311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140882620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12393157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e83be542ff","size":5,"soft_fail":false,"target_va":5377631776,"va":5390024928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088f2c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12340872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e878b143ff","size":5,"soft_fail":false,"target_va":5377684160,"va":5390025027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cc90
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9400945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e88f8d70ff","size":5,"soft_fail":false,"target_va":5380623504,"va":5390024444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b78920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9286821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e85b4b72ff","size":5,"soft_fail":false,"target_va":5380737312,"va":5390024128,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141256ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2085128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8f82ee0ff","size":5,"soft_fail":false,"target_va":5387938976,"va":5390024099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141265a10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2024418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e81e1ce1ff","size":5,"soft_fail":false,"target_va":5387999760,"va":5390024173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4d70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1437416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e81811eaff","size":5,"soft_fail":false,"target_va":5388586352,"va":5390023763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21516321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e821504801","size":5,"soft_fail":false,"target_va":5411540992,"va":5390024666,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15669032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8d8e810ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5390024595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t21467820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8ac924701","size":5,"soft_fail":false,"target_va":5411492368,"va":5390024543,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t21470271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e83f9c4701","size":5,"soft_fail":false,"target_va":5411494800,"va":5390024524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8200289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e89fdf82ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5390023884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8200479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8e1de82ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5390024490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21512419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390023680,"raw_hex":"e8e3404801","size":5,"soft_fail":false,"target_va":5411537104,"va":5390024680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":9564,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":7700,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10012,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":7368,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":7940,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":7884,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9652,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":7628,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1605,"record_offset":7728,"parent_record_offset":7104,"ancestor_record_offset":7104,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9452,"parent_record_offset":9428,"ancestor_record_offset":7104,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":7104,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9396,"parent_record_offset":9360,"ancestor_record_offset":7104,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":9428,"parent_record_offset":9396,"ancestor_record_offset":7104,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":9360,"parent_record_offset":9324,"ancestor_record_offset":7104,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":7472,"parent_record_offset":7448,"ancestor_record_offset":7104,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":7520,"parent_record_offset":7496,"ancestor_record_offset":7104,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":7496,"parent_record_offset":7472,"ancestor_record_offset":7104,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":9324,"parent_record_offset":7980,"ancestor_record_offset":7104,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":9700,"parent_record_offset":9676,"ancestor_record_offset":7104,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":10052,"parent_record_offset":10012,"ancestor_record_offset":7104,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":7448,"parent_record_offset":7424,"ancestor_record_offset":7104,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":7424,"parent_record_offset":7392,"ancestor_record_offset":7104,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":7392,"parent_record_offset":7368,"ancestor_record_offset":7104,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8380,"parent_record_offset":8352,"ancestor_record_offset":7104,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":7104,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8408,"parent_record_offset":8352,"ancestor_record_offset":7104,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1605,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":7104,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":7104,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1605,"record_offset":8324,"parent_record_offset":8296,"ancestor_record_offset":7104,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":7104,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":7104,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":7664,"parent_record_offset":7628,"ancestor_record_offset":7104,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":9504,"parent_record_offset":9476,"ancestor_record_offset":7104,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":7104,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":7564,"parent_record_offset":7392,"ancestor_record_offset":7104,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9944,"parent_record_offset":9880,"ancestor_record_offset":7104,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":7792,"parent_record_offset":7756,"ancestor_record_offset":7104,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9592,"parent_record_offset":9564,"ancestor_record_offset":7104,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9736,"parent_record_offset":9700,"ancestor_record_offset":7104,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9616,"parent_record_offset":9592,"ancestor_record_offset":7104,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":7104,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":7816,"parent_record_offset":7792,"ancestor_record_offset":7104,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9760,"parent_record_offset":9736,"ancestor_record_offset":7104,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":9792,"parent_record_offset":9700,"ancestor_record_offset":7104,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$59::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":8076,"parent_record_offset":8040,"ancestor_record_offset":7104,"inlinee":27610,"source_line":215,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$6::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":8040,"parent_record_offset":7980,"ancestor_record_offset":7104,"inlinee":27805,"source_line":222,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":9016,"parent_record_offset":7980,"ancestor_record_offset":7104,"inlinee":27807,"source_line":233,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1605,"record_offset":9112,"parent_record_offset":9088,"ancestor_record_offset":7104,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1605,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":7104,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":8540,"parent_record_offset":8180,"ancestor_record_offset":7104,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":8180,"parent_record_offset":8152,"ancestor_record_offset":7104,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":7104,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":7104,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":7104,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":7104,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1605,"record_offset":9856,"parent_record_offset":9832,"ancestor_record_offset":7104,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1605,"record_offset":7756,"parent_record_offset":7728,"ancestor_record_offset":7104,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1605,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":7104,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1605,"record_offset":7848,"parent_record_offset":7756,"ancestor_record_offset":7104,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1605,"record_offset":9916,"parent_record_offset":9880,"ancestor_record_offset":7104,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":9208,"parent_record_offset":9016,"ancestor_record_offset":7104,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":9064,"parent_record_offset":9016,"ancestor_record_offset":7104,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":8124,"parent_record_offset":8076,"ancestor_record_offset":7104,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":7104,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8352,"parent_record_offset":8296,"ancestor_record_offset":7104,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":7104,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1605,"record_offset":8296,"parent_record_offset":8264,"ancestor_record_offset":7104,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1605,"record_offset":8264,"parent_record_offset":8236,"ancestor_record_offset":7104,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$1::publish::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":1605,"record_offset":7980,"parent_record_offset":7940,"ancestor_record_offset":7104,"inlinee":73672,"source_line":111,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=3
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1605,"record_offset":5052,"rva":21312336,"va":5390021456}
// 编译实例 RVAs: 0x1453350, 0x1cf6500, 0x25db920
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=3
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19865247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e861e1d0fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5390023370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14940602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e846061cff","size":5,"soft_fail":false,"target_va":5375081488,"va":5390022085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140960ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20536141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399078144,"raw_hex":"e8b3a4c6fe","size":5,"soft_fail":false,"target_va":5378542272,"va":5399078408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140960ef0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20535101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399078144,"raw_hex":"e8c3a8c6fe","size":5,"soft_fail":false,"target_va":5378543344,"va":5399078440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cb30
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9399361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e8bf9370ff","size":5,"soft_fail":false,"target_va":5380623152,"va":5390022508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f5580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1433190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e89a21eaff","size":5,"soft_fail":false,"target_va":5388588416,"va":5390021601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141bb0250
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1336058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399078144,"raw_hex":"e8069debff","size":5,"soft_fail":false,"target_va":5397742160,"va":5399078213,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12768929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399078144,"raw_hex":"e8a1d6c200","size":5,"soft_fail":false,"target_va":5411847168,"va":5399078234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t21518198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e876574801","size":5,"soft_fail":false,"target_va":5411540992,"va":5390022789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-15667093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e86bf010ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5390022656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t21469762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e8429a4701","size":5,"soft_fail":false,"target_va":5411492368,"va":5390022601,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t21472213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e8d5a34701","size":5,"soft_fail":false,"target_va":5411494800,"va":5390022582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8198417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e8efe682ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5390022012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-8198536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5390021456,"raw_hex":"e878e682ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5390022547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":6160,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":5620,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":6608,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":5288,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":5860,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":5804,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":6248,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1605,"record_offset":5548,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1605,"record_offset":5648,"parent_record_offset":5052,"ancestor_record_offset":5052,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5052,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5052,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":5052,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6024,"parent_record_offset":5992,"ancestor_record_offset":5052,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":5960,"parent_record_offset":5928,"ancestor_record_offset":5052,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":5392,"parent_record_offset":5368,"ancestor_record_offset":5052,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":5052,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1605,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":5052,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":5928,"parent_record_offset":5896,"ancestor_record_offset":5052,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":5052,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":6652,"parent_record_offset":6608,"ancestor_record_offset":5052,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":5368,"parent_record_offset":5344,"ancestor_record_offset":5052,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":5344,"parent_record_offset":5312,"ancestor_record_offset":5052,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1605,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":5052,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":2625,"record_offset":1536,"parent_record_offset":1496,"ancestor_record_offset":652,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/02625.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1605,"record_offset":5828,"parent_record_offset":5804,"ancestor_record_offset":5052,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":6272,"parent_record_offset":6248,"ancestor_record_offset":5052,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1605,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":5052,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1605,"record_offset":5584,"parent_record_offset":5548,"ancestor_record_offset":5052,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2625,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":652,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5052,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":6764,"parent_record_offset":6608,"ancestor_record_offset":5052,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1605,"record_offset":5484,"parent_record_offset":5312,"ancestor_record_offset":5052,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6540,"parent_record_offset":6476,"ancestor_record_offset":5052,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":5712,"parent_record_offset":5676,"ancestor_record_offset":5052,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6188,"parent_record_offset":6160,"ancestor_record_offset":5052,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6332,"parent_record_offset":6296,"ancestor_record_offset":5052,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":5052,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":5052,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":5052,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":5052,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1605,"record_offset":6388,"parent_record_offset":6296,"ancestor_record_offset":5052,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1605,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":5052,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1605,"record_offset":5676,"parent_record_offset":5648,"ancestor_record_offset":5052,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1605,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":5052,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1605,"record_offset":5768,"parent_record_offset":5676,"ancestor_record_offset":5052,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1605,"record_offset":6512,"parent_record_offset":6476,"ancestor_record_offset":5052,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] tabloid::impl$1::publish::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tabloid","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tabloid/src/lib.rs
//     evidence={"module":1605,"record_offset":5896,"parent_record_offset":5860,"ancestor_record_offset":5052,"inlinee":73672,"source_line":111,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 89
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11A76
// 首条位置: {"module":1585,"inlinee":72310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::strong_count
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":94588,"parent_record_offset":94536,"ancestor_record_offset":94536,"inlinee":11385,"source_line":1799,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":94708,"parent_record_offset":94536,"ancestor_record_offset":94536,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1585,"record_offset":95008,"parent_record_offset":94536,"ancestor_record_offset":94536,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1585,"record_offset":108096,"parent_record_offset":107624,"ancestor_record_offset":107624,"inlinee":27807,"source_line":233,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::dirty_tiles::impl$2::apply_to
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1585,"record_offset":94952,"parent_record_offset":94536,"ancestor_record_offset":94536,"inlinee":29581,"source_line":310,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1585,"record_offset":94736,"parent_record_offset":94536,"ancestor_record_offset":94536,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$25::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1585,"record_offset":108040,"parent_record_offset":107624,"ancestor_record_offset":107624,"inlinee":12544,"source_line":1660,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1585,"record_offset":94612,"parent_record_offset":94588,"ancestor_record_offset":94536,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1585,"record_offset":108064,"parent_record_offset":108040,"ancestor_record_offset":107624,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1585,"record_offset":94644,"parent_record_offset":94612,"ancestor_record_offset":94536,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":95188,"parent_record_offset":95064,"ancestor_record_offset":94536,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":94800,"parent_record_offset":94764,"ancestor_record_offset":94536,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":95212,"parent_record_offset":95188,"ancestor_record_offset":94536,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":94824,"parent_record_offset":94800,"ancestor_record_offset":94536,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1585,"record_offset":94672,"parent_record_offset":94612,"ancestor_record_offset":94536,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::dirty_tiles::DirtyTiles::extend
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/dirty_tiles.rs
//     evidence={"module":1585,"record_offset":94976,"parent_record_offset":94952,"ancestor_record_offset":94536,"inlinee":29556,"source_line":189,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":108184,"parent_record_offset":108160,"ancestor_record_offset":107624,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":108208,"parent_record_offset":108184,"ancestor_record_offset":107624,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1585,"record_offset":108232,"parent_record_offset":108208,"ancestor_record_offset":107624,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1585,"record_offset":108300,"parent_record_offset":108276,"ancestor_record_offset":107624,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1585,"record_offset":108160,"parent_record_offset":108136,"ancestor_record_offset":107624,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1585,"record_offset":95036,"parent_record_offset":95008,"ancestor_record_offset":94536,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1585,"record_offset":94856,"parent_record_offset":94764,"ancestor_record_offset":94536,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1585,"record_offset":95100,"parent_record_offset":95064,"ancestor_record_offset":94536,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1585,"record_offset":94764,"parent_record_offset":94736,"ancestor_record_offset":94536,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1585,"record_offset":95064,"parent_record_offset":95036,"ancestor_record_offset":94536,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1585,"record_offset":94884,"parent_record_offset":94856,"ancestor_record_offset":94536,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1585,"record_offset":95128,"parent_record_offset":95100,"ancestor_record_offset":94536,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1585,"record_offset":108276,"parent_record_offset":108096,"ancestor_record_offset":107624,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1585,"record_offset":108136,"parent_record_offset":108096,"ancestor_record_offset":107624,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/01585.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$1::publish::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11FC8
// 首条位置: {"module":1605,"inlinee":73672}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1605,"record_offset":5928,"parent_record_offset":5896,"ancestor_record_offset":5896,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":2625,"record_offset":1536,"parent_record_offset":1496,"ancestor_record_offset":1496,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/02625.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$6::empty
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":8040,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":27805,"source_line":222,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::collision::world::impl$7::apply_to
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":9016,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":27807,"source_line":233,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5896,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5896,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":5992,"parent_record_offset":5960,"ancestor_record_offset":5896,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1605,"record_offset":6024,"parent_record_offset":5992,"ancestor_record_offset":5896,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1605,"record_offset":5960,"parent_record_offset":5928,"ancestor_record_offset":5896,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8380,"parent_record_offset":8352,"ancestor_record_offset":7980,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":7980,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1605,"record_offset":8408,"parent_record_offset":8352,"ancestor_record_offset":7980,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1605,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":7980,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1605,"record_offset":8324,"parent_record_offset":8296,"ancestor_record_offset":7980,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2625,"record_offset":1560,"parent_record_offset":1536,"ancestor_record_offset":1496,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02625.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":5896,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1605,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":7980,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::collision::world::impl$59::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/collision/world.rs
//     evidence={"module":1605,"record_offset":8076,"parent_record_offset":8040,"ancestor_record_offset":7980,"inlinee":27610,"source_line":215,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1605,"record_offset":9112,"parent_record_offset":9088,"ancestor_record_offset":7980,"inlinee":5625,"source_line":1954,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":1605,"record_offset":9136,"parent_record_offset":9112,"ancestor_record_offset":7980,"inlinee":5626,"source_line":1993,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::with_hasher_in
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":8540,"parent_record_offset":8180,"ancestor_record_offset":7980,"inlinee":4799,"source_line":543,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$7::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":8180,"parent_record_offset":8152,"ancestor_record_offset":7980,"inlinee":4790,"source_line":2070,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1605,"record_offset":9164,"parent_record_offset":9136,"ancestor_record_offset":7980,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":7980,"inlinee":6057,"source_line":1092,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::remove
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":9088,"parent_record_offset":9064,"ancestor_record_offset":7980,"inlinee":15887,"source_line":1175,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$12::default
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\set.rs
//     evidence={"module":1605,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":7980,"inlinee":4788,"source_line":1355,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":9208,"parent_record_offset":9016,"ancestor_record_offset":7980,"inlinee":6056,"source_line":917,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":9064,"parent_record_offset":9016,"ancestor_record_offset":7980,"inlinee":15886,"source_line":964,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\set.rs
//     evidence={"module":1605,"record_offset":8124,"parent_record_offset":8076,"ancestor_record_offset":7980,"inlinee":4786,"source_line":1161,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":7980,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8352,"parent_record_offset":8296,"ancestor_record_offset":7980,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1605,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":7980,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1605,"record_offset":8296,"parent_record_offset":8264,"ancestor_record_offset":7980,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":1605,"record_offset":8264,"parent_record_offset":8236,"ancestor_record_offset":7980,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/01605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$2::drop
// 记录形式: inline_instance, inlinee_declaration; instances=27
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60B6
// 首条位置: {"module":411,"inlinee":24758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":411,"record_offset":12916,"parent_record_offset":12508,"ancestor_record_offset":12508,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] core::mem::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":411,"record_offset":12548,"parent_record_offset":12508,"ancestor_record_offset":12508,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] core::mem::replace
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":411,"record_offset":12828,"parent_record_offset":12508,"ancestor_record_offset":12508,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":411,"record_offset":13028,"parent_record_offset":12508,"ancestor_record_offset":12508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":411,"record_offset":12940,"parent_record_offset":12916,"ancestor_record_offset":12508,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":411,"record_offset":13052,"parent_record_offset":13028,"ancestor_record_offset":12508,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":411,"record_offset":12964,"parent_record_offset":12940,"ancestor_record_offset":12508,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":411,"record_offset":12988,"parent_record_offset":12964,"ancestor_record_offset":12508,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":411,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":12508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":411,"record_offset":12856,"parent_record_offset":12828,"ancestor_record_offset":12508,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":411,"record_offset":12884,"parent_record_offset":12828,"ancestor_record_offset":12508,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":411,"record_offset":12756,"parent_record_offset":12632,"ancestor_record_offset":12508,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":411,"record_offset":12780,"parent_record_offset":12756,"ancestor_record_offset":12508,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":411,"record_offset":13144,"parent_record_offset":13052,"ancestor_record_offset":12508,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$43::drop
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":411,"record_offset":12604,"parent_record_offset":12576,"ancestor_record_offset":12508,"inlinee":5808,"source_line":2431,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":411,"record_offset":12668,"parent_record_offset":12632,"ancestor_record_offset":12508,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":411,"record_offset":12632,"parent_record_offset":12604,"ancestor_record_offset":12508,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":411,"record_offset":12696,"parent_record_offset":12668,"ancestor_record_offset":12508,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00411.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::impl$3::deref
// 记录形式: inline_instance, inlinee_declaration; instances=12
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x60B4
// 首条位置: {"module":411,"inlinee":24756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>
// 记录形式: compiled_procedure; instances=2
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1545,"record_offset":3344,"rva":19474912,"va":5388184032}
// 编译实例 RVAs: 0x12929e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=2
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18026462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e822f0ecfe","size":5,"soft_fail":false,"target_va":5370158128,"va":5388184585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cc90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7561022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e8c2a08cff","size":5,"soft_fail":false,"target_va":5380623504,"va":5388184521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23307932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e89ca66301","size":5,"soft_fail":false,"target_va":5411492368,"va":5388184431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23310349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e80db06301","size":5,"soft_fail":false,"target_va":5411494800,"va":5388184446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23304781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e84d9a6301","size":5,"soft_fail":false,"target_va":5411488944,"va":5388184158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23307199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e8bfa36301","size":5,"soft_fail":false,"target_va":5411491664,"va":5388184460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6360491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e855f29eff","size":5,"soft_fail":false,"target_va":5381823600,"va":5388184086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6360223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388184032,"raw_hex":"e861f39eff","size":5,"soft_fail":false,"target_va":5381824016,"va":5388184234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":4444,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":3532,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1545,"record_offset":3716,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":4196,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1545,"record_offset":3560,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1545,"record_offset":3856,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1545,"record_offset":4836,"parent_record_offset":3344,"ancestor_record_offset":3344,"inlinee":5837,"source_line":1206,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":4468,"parent_record_offset":4444,"ancestor_record_offset":3344,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":5140,"parent_record_offset":5104,"ancestor_record_offset":3344,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1545,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":3344,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1545,"record_offset":4860,"parent_record_offset":4836,"ancestor_record_offset":3344,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1545,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":3344,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1545,"record_offset":4012,"parent_record_offset":3920,"ancestor_record_offset":3344,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1545,"record_offset":5172,"parent_record_offset":5140,"ancestor_record_offset":3344,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":3344,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1545,"record_offset":3792,"parent_record_offset":3740,"ancestor_record_offset":3344,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":5012,"parent_record_offset":4948,"ancestor_record_offset":3344,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":3624,"parent_record_offset":3588,"ancestor_record_offset":3344,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":4040,"parent_record_offset":3920,"ancestor_record_offset":3344,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":4376,"parent_record_offset":4252,"ancestor_record_offset":3344,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":3956,"parent_record_offset":3920,"ancestor_record_offset":3344,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3344,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":3648,"parent_record_offset":3624,"ancestor_record_offset":3344,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":3980,"parent_record_offset":3956,"ancestor_record_offset":3344,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":4400,"parent_record_offset":4376,"ancestor_record_offset":3344,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1545,"record_offset":5232,"parent_record_offset":5104,"ancestor_record_offset":3344,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1545,"record_offset":4920,"parent_record_offset":4892,"ancestor_record_offset":3344,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1545,"record_offset":4224,"parent_record_offset":4196,"ancestor_record_offset":3344,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1545,"record_offset":3588,"parent_record_offset":3560,"ancestor_record_offset":3344,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1545,"record_offset":4948,"parent_record_offset":4920,"ancestor_record_offset":3344,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1545,"record_offset":4100,"parent_record_offset":3884,"ancestor_record_offset":3344,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1545,"record_offset":4288,"parent_record_offset":4252,"ancestor_record_offset":3344,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1545,"record_offset":3920,"parent_record_offset":3884,"ancestor_record_offset":3344,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1545,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":3344,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1545,"record_offset":4252,"parent_record_offset":4224,"ancestor_record_offset":3344,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1545,"record_offset":3680,"parent_record_offset":3588,"ancestor_record_offset":3344,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1545,"record_offset":4316,"parent_record_offset":4288,"ancestor_record_offset":3344,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 记录形式: compiled_procedure; instances=4
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1475,"record_offset":2004,"rva":19168400,"va":5387877520}
// 编译实例 RVAs: 0x1247c90, 0x1454590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=4
//     target_aliases=47 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17719950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e8729df1fe","size":5,"soft_fail":false,"target_va":5370158128,"va":5387878073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5cb30
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7254862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e8b24c91ff","size":5,"soft_fail":false,"target_va":5380623152,"va":5387878009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23614441","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e8e9536801","size":5,"soft_fail":false,"target_va":5411492368,"va":5387877922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23616858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e85a5d6801","size":5,"soft_fail":false,"target_va":5411494800,"va":5387877937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23611293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e89d476801","size":5,"soft_fail":false,"target_va":5411488944,"va":5387877646,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t23613708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e80c516801","size":5,"soft_fail":false,"target_va":5411491664,"va":5387877951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6053979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e8a59fa3ff","size":5,"soft_fail":false,"target_va":5381823600,"va":5387877574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-6053714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387877520,"raw_hex":"e8aea0a3ff","size":5,"soft_fail":false,"target_va":5381824016,"va":5387877725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t21468242","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5390026128,"raw_hex":"e952944701","size":5,"soft_fail":false,"target_va":5411494800,"va":5390026553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::Arc::into_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":3096,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":12275,"source_line":1554,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":2184,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1475,"record_offset":2368,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":2848,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1475,"record_offset":2212,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1475,"record_offset":2508,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLockReadGuard::map
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1475,"record_offset":3488,"parent_record_offset":2004,"ancestor_record_offset":2004,"inlinee":5837,"source_line":1206,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":3120,"parent_record_offset":3096,"ancestor_record_offset":2004,"inlinee":7985,"source_line":1606,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":3792,"parent_record_offset":3756,"ancestor_record_offset":2004,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1475,"record_offset":3756,"parent_record_offset":3732,"ancestor_record_offset":2004,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1475,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":2004,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1475,"record_offset":2392,"parent_record_offset":2368,"ancestor_record_offset":2004,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1475,"record_offset":2664,"parent_record_offset":2572,"ancestor_record_offset":2004,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1475,"record_offset":3824,"parent_record_offset":3792,"ancestor_record_offset":2004,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":2004,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1475,"record_offset":2444,"parent_record_offset":2392,"ancestor_record_offset":2004,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":3664,"parent_record_offset":3600,"ancestor_record_offset":2004,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":2276,"parent_record_offset":2240,"ancestor_record_offset":2004,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":2692,"parent_record_offset":2572,"ancestor_record_offset":2004,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":3028,"parent_record_offset":2904,"ancestor_record_offset":2004,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":2608,"parent_record_offset":2572,"ancestor_record_offset":2004,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":2004,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":2300,"parent_record_offset":2276,"ancestor_record_offset":2004,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":2004,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":3052,"parent_record_offset":3028,"ancestor_record_offset":2004,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1475,"record_offset":3884,"parent_record_offset":3756,"ancestor_record_offset":2004,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1475,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":2004,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":1475,"record_offset":2876,"parent_record_offset":2848,"ancestor_record_offset":2004,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1475,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":2004,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":1475,"record_offset":3600,"parent_record_offset":3572,"ancestor_record_offset":2004,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1475,"record_offset":2752,"parent_record_offset":2536,"ancestor_record_offset":2004,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1475,"record_offset":2940,"parent_record_offset":2904,"ancestor_record_offset":2004,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1475,"record_offset":2572,"parent_record_offset":2536,"ancestor_record_offset":2004,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1475,"record_offset":2536,"parent_record_offset":2508,"ancestor_record_offset":2004,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":1475,"record_offset":2904,"parent_record_offset":2876,"ancestor_record_offset":2004,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1475,"record_offset":2332,"parent_record_offset":2240,"ancestor_record_offset":2004,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/01475.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":1475,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2004,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/01475.jsonl.gz"}
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
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publication
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publication", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > >", "constructor": "alloc::sync::ArcInner", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publication", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::rwlock::RwLockWriteGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::Vec::retain", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "alloc::vec::impl$1::retain::closure$0", "arguments": ["alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global", "tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::PublicationReader
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::PublicationReader", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Publisher
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publisher", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::systems::collision::world::ColliderDiff> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "alloc::sync::ArcInner", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Publisher<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Publisher::subscribe", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >", "constructor": "tabloid::Publisher::publish", "arguments": ["country_core::systems::collision::world::ColliderDiff", "enum2$<country_core::systems::collision::world::ColliderDelta>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscriber
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Subscriber::take", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tabloid::Subscription
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>", "constructor": "tabloid::Subscription", "arguments": ["country_core::utils::dirty_tiles::DirtyTiles"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>", "constructor": "tabloid::Subscription", "arguments": ["country_core::systems::collision::world::ColliderDiff"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::utils::dirty_tiles::DirtyTiles> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "tabloid::Subscription<country_core::systems::collision::world::ColliderDiff>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,tabloid::Subscription<country_core::systems::collision::world::ColliderDiff> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 166; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)
// 候选参数类型: ["country_core::resources::walls::wall_path_segment::WallPathSegment"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>,bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>,bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>)
// 候选参数类型: ["bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>", "bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>", "bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::add_birdnests::BirdNestLazySubscriber>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::BirdNestCandidateLocations>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<system_clutter::autoclutter::add_birdnests::BirdNestCollisionCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui_clutter::DeletedAutoClutter>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::stitch_bricks::StitchBrickAlloc>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::construct_inter_roof_merlons::InterRoofMerlonBrickCount>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>", "bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::read_bush_positions_from_gpu::BushGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::system::commands::Commands,bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::system::commands::Commands", "bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::AutoClutter>,enum2$<core::option::Option<ref_mut$<system_clutter::autoclutter::WindowShutter> > >,ref_mut$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::resources::clutter_meshes::ClutterMarkedForDeletion> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::AutoClutter>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<utils::components::transform::Transform>,tuple$<bevy_ecs::query::filter::With<country_core::resources::critters::sheep::Sheep>,bevy_ecs::query::filter::Without<country_core::components::AutoClutter> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsHole,alloc::alloc::Global> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsHole,alloc::alloc::Global> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlons>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::InterRoofMerlonsHole,alloc::alloc::Global> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>,bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "system_clutter::autoclutter::populate_autoclutter_regions::DecoratorRejectionRes", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>", "bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::clutter_meshes::ClutterMeshes>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::AutoClutterState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::autoclutter::populate_autoclutter_regions::AutoClutterDirtySubscriptions>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges,bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >,bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>,bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "system_clutter::inter_shape_stitches::detect_intra_shape_corners::WallAndRoofChanges", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >", "bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>", "bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<alloc::vec::Vec<system_clutter::inter_shape_stitches::detect_intra_shape_corners::ShapeIntersectionHole,alloc::alloc::Global> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<system_clutter::inter_shape_stitches::detect_intra_shape_corners::IntraShapeCorners>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::shape_intersections::ShapeIntersections>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >
// 信息 ID: 7c716b14daa6ac8b7ef7c687
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["country_core::systems::collision::world::ColliderDelta"]; 字段布局 = unknown。
// [类型关系] tabloid::Publisher::publish -> ["country_core::systems::collision::world::ColliderDiff", "enum2$<country_core::systems::collision::world::ColliderDelta>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::Rapier::on_collider_upserted @ 0x8ce487 -> 0x1453c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::dispose_collider_mesh_instance @ 0x8d972e -> 0x1453c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xb40ff2 -> 0x1453c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::upsert_collider_entry<bevy_ecs::entity::Entity,tuple$<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat> > @ 0xb412e6 -> 0x1453c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 0ac3b7aca709b77e41413aec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Publisher::publish -> ["country_core::utils::dirty_tiles::DirtyTiles", "country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::flush_colliders @ 0x8cd8cf -> 0x1453350 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::terrain_height::TerrainHeightsData::copy_from_gpu<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x9ea65a -> 0x1453350 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$2::copy_to_staging<512> @ 0xa1760e -> 0x1453350 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$2::copy_to_staging<512> @ 0xa1765a -> 0x1453350 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu @ 0xeebd08 -> 0x1cf6500 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$2::copy_to_staging<512> @ 0x2261a9e -> 0x25db920 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::impl$2::copy_to_staging<512> @ 0x2261aea -> 0x25db920 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::subscribe<country_core::systems::collision::world::ColliderDiff>
// 信息 ID: 673e9ee12fde8aea00cb4e1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] d1b1f6201f2a09a88654812c: void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)
// [类型关系] tabloid::Publisher::subscribe -> ["country_core::systems::collision::world::ColliderDiff"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::structural_graph_subscribe @ 0x7d49ac -> 0x12926e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> > > @ 0x14c37d6 -> 0x1454410 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::commands::Commands)> > @ 0x17ceb9d -> 0x1454410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Publisher::subscribe<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: a901e69041ab4b8ab0c0020f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc68acc99a848e3e9f4a896d: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// [类型关系] tabloid::Publisher::subscribe -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::overhang_subscribe_to_tabloids @ 0x786707 -> 0x1247b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::stairs_subscribe_lazy @ 0x7cf2ff -> 0x1292860 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::stairs_subscribe_lazy @ 0x7cf351 -> 0x1292860 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > @ 0x8c6322 -> 0x1454290 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::RaycastWorld::subscribe_to_changes @ 0x8cdc34 -> 0x1454290 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0xa15949 -> 0x1454290 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0xa1597d -> 0x1454290 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)> > > @ 0x14d763e -> 0x1454290 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x22d6bac -> 0x1cf68e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::autoclutter_subscribe_to_tabloids @ 0x22d6d07 -> 0x1cf68e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0x2612c49 -> 0x1cf68e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::world_raster::WorldRaster::subscribe<512> @ 0x2612c7d -> 0x1cf68e0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>
// 信息 ID: ad7fd1859db7da615cffad83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Subscriber::take -> ["country_core::systems::collision::world::ColliderDiff"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph @ 0x7d4cb6 -> 0x12929e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections @ 0xa97ed8 -> 0x12929e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::Subscriber::take<country_core::utils::dirty_tiles::DirtyTiles>
// 信息 ID: 579161411fb57c72f0e635be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tabloid::Subscriber::take -> ["country_core::utils::dirty_tiles::DirtyTiles"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments @ 0x786e9f -> 0x1247c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags @ 0x7d1c7f -> 0x1247c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::add_birdnests::add_birdnests @ 0x22c9f02 -> 0x1247c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions @ 0x22d7273 -> 0x1247c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$1> @ 0x1458a6d -> 0x1454590 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::dirty_tiles::impl$3::iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env$0> @ 0x1458b3d -> 0x1454590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$0::default
// 信息 ID: 77e6e45f8a40b9e45bec7838
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 25 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_clutter::impl$0::build; module=308, inline_record=5572; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::world::RaycastWorld::on_level_reload; module=496, inline_record=160464; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::world_raster::DirtyTilePublisher::unique; module=509, inline_record=71936; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::impl$0::insert_terrain_resources; module=511, inline_record=70884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::terrain::impl$0::insert_terrain_resources; module=511, inline_record=71492; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$44::default<country_core::systems::collision::world::RapierSubWorld>; module=587, inline_record=102436; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$44::default<country_core::systems::collision::world::RapierSubWorld>; module=587, inline_record=102612; 不是独立调用边。
// [PDB 内联宿主] core::array::impl$44::default<country_core::systems::collision::world::RapierSubWorld>; module=587, inline_record=102788; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$0
// 信息 ID: fd85015394947b3ddf9a990e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tabloid::impl$1::publish::closure_env$0 <- alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 7 条。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1585, inline_record=94536; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::Vec::retain<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=1585, inline_record=95768; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> > >; module=1585, inline_record=107624; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=2440, inline_record=4512; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::impl$1::retain::closure$0<alloc::sync::Arc<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::utils::dirty_tiles::DirtyTiles> >,alloc::alloc::Global>,alloc::alloc::Global,tabloid::impl$1::publish::closure_env$0<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles> >; module=2611, inline_record=28216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$1::publish::closure$1
// 信息 ID: 9de778c7da6505bbc667ed06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tabloid::impl$1::publish", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=1605, inline_record=5896; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::systems::collision::world::ColliderDiff,enum2$<country_core::systems::collision::world::ColliderDelta> >; module=1605, inline_record=7980; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=2625, inline_record=1496; 不是独立调用边。
// [PDB 内联宿主] tabloid::Publisher::publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>; module=4429, inline_record=1496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$2::drop
// 信息 ID: d6c4318c49e5ae9a70a4810a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=411, inline_record=12508; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags; module=437, inline_record=30564; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph; module=437, inline_record=65516; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::systems::collision::world::ColliderDiff> >; module=485, inline_record=66900; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=485, inline_record=1130144; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=49244; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=1066, inline_record=180268; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<tabloid::PublicationReader<country_core::utils::dirty_tiles::DirtyTiles> >; module=1411, inline_record=65124; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tabloid::impl$3::deref
// 信息 ID: 8bdf4f2533d210be6e989fd0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments; module=411, inline_record=12452; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assembly_propagate_dirty_flags; module=437, inline_record=30512; 不是独立调用边。
// [PDB 内联宿主] system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph; module=437, inline_record=60568; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::collision::intersect_meshes::find_mesh_mesh_intersections; module=559, inline_record=40704; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::add_birdnests::add_birdnests; module=3799, inline_record=4808; 不是独立调用边。
// [PDB 内联宿主] system_clutter::autoclutter::populate_autoclutter_regions::populate_autoclutter_regions; module=3811, inline_record=10940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
