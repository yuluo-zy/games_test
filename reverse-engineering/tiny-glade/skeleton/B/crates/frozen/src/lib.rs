// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/frozen/src/lib.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b52bc5c23ce3ca5b311aa21ad6bb6e4f35c79ab775332d0b4251eefc23e307c4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 250, 506, 508, 509, 511, 517, 1422, 1423, 1492, 1579, 2280, 2627, 3471, 3481, 3697, 3873, 3975, 4424, 4441, 4464
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
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":22152,"rva":9873040,"va":5378582160}
// 编译实例 RVAs: 0x96a690
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3905960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e85866c4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378582291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0df0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3775621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e87b63c6ff","size":5,"soft_fail":false,"target_va":5374807536,"va":5378583152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3501113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e8c793caff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378582612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-924949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e8ebe2f1ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5378582592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e854e7ffff","size":5,"soft_fail":false,"target_va":5378576928,"va":5378583239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428aa250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32766110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e89ef8f301","size":5,"soft_fail":false,"target_va":5411349072,"va":5378582957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32959958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e8d6edf601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378582917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32955753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e869ddf601","size":5,"soft_fail":false,"target_va":5411538608,"va":5378582850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32957464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e818e4f601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378582819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32957143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378582160,"raw_hex":"e8d7e2f601","size":5,"soft_fail":false,"target_va":5411539984,"va":5378582836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":24016,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":22920,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":23016,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":24640,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":22460,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":13083,"source_line":457,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::impl$26::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":22960,"parent_record_offset":22152,"ancestor_record_offset":22152,"inlinee":29629,"source_line":769,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":22152,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":22152,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":517,"record_offset":24468,"parent_record_offset":24440,"ancestor_record_offset":22152,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":517,"record_offset":22984,"parent_record_offset":22960,"ancestor_record_offset":22152,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":25276,"parent_record_offset":25252,"ancestor_record_offset":22152,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":23280,"parent_record_offset":23256,"ancestor_record_offset":22152,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":23364,"parent_record_offset":23340,"ancestor_record_offset":22152,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":23304,"parent_record_offset":23280,"ancestor_record_offset":22152,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":23752,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":23972,"parent_record_offset":23940,"ancestor_record_offset":22152,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":25564,"parent_record_offset":25536,"ancestor_record_offset":22152,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":23512,"parent_record_offset":23484,"ancestor_record_offset":22152,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":23484,"parent_record_offset":23456,"ancestor_record_offset":22152,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":23628,"parent_record_offset":23604,"ancestor_record_offset":22152,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":25744,"parent_record_offset":25716,"ancestor_record_offset":22152,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":25772,"parent_record_offset":25744,"ancestor_record_offset":22152,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":22840,"parent_record_offset":22812,"ancestor_record_offset":22152,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":24276,"parent_record_offset":24088,"ancestor_record_offset":22152,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":22528,"parent_record_offset":22496,"ancestor_record_offset":22152,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":24372,"parent_record_offset":24336,"ancestor_record_offset":22152,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":22744,"parent_record_offset":22712,"ancestor_record_offset":22152,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":22552,"parent_record_offset":22528,"ancestor_record_offset":22152,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":22768,"parent_record_offset":22744,"ancestor_record_offset":22152,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":22152,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":22152,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":22152,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":23688,"parent_record_offset":23664,"ancestor_record_offset":22152,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":23424,"parent_record_offset":23396,"ancestor_record_offset":22152,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":23456,"parent_record_offset":23424,"ancestor_record_offset":22152,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":23396,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":23256,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":25252,"parent_record_offset":25180,"ancestor_record_offset":22152,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":23780,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":25000,"parent_record_offset":24956,"ancestor_record_offset":22152,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":23340,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":23200,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":25688,"parent_record_offset":24696,"ancestor_record_offset":22152,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":23912,"parent_record_offset":23048,"ancestor_record_offset":22152,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":23048,"parent_record_offset":23016,"ancestor_record_offset":22152,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":23088,"parent_record_offset":23048,"ancestor_record_offset":22152,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23604,"parent_record_offset":23580,"ancestor_record_offset":22152,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23580,"parent_record_offset":23556,"ancestor_record_offset":22152,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25628,"parent_record_offset":25604,"ancestor_record_offset":22152,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25716,"parent_record_offset":24696,"ancestor_record_offset":22152,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23872,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23116,"parent_record_offset":23048,"ancestor_record_offset":22152,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":24696,"parent_record_offset":24668,"ancestor_record_offset":22152,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23940,"parent_record_offset":23912,"ancestor_record_offset":22152,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25604,"parent_record_offset":24696,"ancestor_record_offset":22152,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23144,"parent_record_offset":23116,"ancestor_record_offset":22152,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":24780,"parent_record_offset":24740,"ancestor_record_offset":22152,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":24956,"parent_record_offset":24780,"ancestor_record_offset":22152,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25180,"parent_record_offset":24780,"ancestor_record_offset":22152,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25216,"parent_record_offset":25180,"ancestor_record_offset":22152,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":24740,"parent_record_offset":24696,"ancestor_record_offset":22152,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23228,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25536,"parent_record_offset":25512,"ancestor_record_offset":22152,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":25512,"parent_record_offset":24740,"ancestor_record_offset":22152,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":23556,"parent_record_offset":23144,"ancestor_record_offset":22152,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":24668,"parent_record_offset":24640,"ancestor_record_offset":22152,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":23664,"parent_record_offset":23556,"ancestor_record_offset":22152,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":22712,"parent_record_offset":22684,"ancestor_record_offset":22152,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":22684,"parent_record_offset":22656,"ancestor_record_offset":22152,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":24088,"parent_record_offset":24048,"ancestor_record_offset":22152,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":22812,"parent_record_offset":22616,"ancestor_record_offset":22152,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":22616,"parent_record_offset":22588,"ancestor_record_offset":22152,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":22588,"parent_record_offset":22460,"ancestor_record_offset":22152,"inlinee":13086,"source_line":694,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":24048,"parent_record_offset":24016,"ancestor_record_offset":22152,"inlinee":13088,"source_line":826,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":22496,"parent_record_offset":22460,"ancestor_record_offset":22152,"inlinee":13084,"source_line":172,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":24336,"parent_record_offset":24048,"ancestor_record_offset":22152,"inlinee":13089,"source_line":182,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":517,"record_offset":22656,"parent_record_offset":22616,"ancestor_record_offset":22152,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":18016,"rva":9871808,"va":5378580928}
// 编译实例 RVAs: 0x96a1c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3904728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e8286bc4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378581059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0df0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3774389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e84b68c6ff","size":5,"soft_fail":false,"target_va":5374807536,"va":5378581920,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3589845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e82b39c9ff","size":5,"soft_fail":false,"target_va":5374991520,"va":5378581360,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3499881","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e89798caff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378581380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969300
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e804edffff","size":5,"soft_fail":false,"target_va":5378577152,"va":5378582007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428aa250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32767342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e86efdf301","size":5,"soft_fail":false,"target_va":5411349072,"va":5378581725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32961190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e8a6f2f601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378581685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32956985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e839e2f601","size":5,"soft_fail":false,"target_va":5411538608,"va":5378581618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32958696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e8e8e8f601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378581587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32958375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378580928,"raw_hex":"e8a7e7f601","size":5,"soft_fail":false,"target_va":5411539984,"va":5378581604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":19916,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":18820,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":18916,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":20540,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":18360,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":13083,"source_line":457,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::impl$26::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":18860,"parent_record_offset":18016,"ancestor_record_offset":18016,"inlinee":29629,"source_line":769,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":20416,"parent_record_offset":20392,"ancestor_record_offset":18016,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":18016,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":517,"record_offset":20368,"parent_record_offset":20340,"ancestor_record_offset":18016,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":517,"record_offset":18884,"parent_record_offset":18860,"ancestor_record_offset":18016,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":18016,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":19180,"parent_record_offset":19156,"ancestor_record_offset":18016,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":19264,"parent_record_offset":19240,"ancestor_record_offset":18016,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":19204,"parent_record_offset":19180,"ancestor_record_offset":18016,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":19652,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":19872,"parent_record_offset":19840,"ancestor_record_offset":18016,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":21464,"parent_record_offset":21436,"ancestor_record_offset":18016,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":19412,"parent_record_offset":19384,"ancestor_record_offset":18016,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":19384,"parent_record_offset":19356,"ancestor_record_offset":18016,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":19528,"parent_record_offset":19504,"ancestor_record_offset":18016,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":21644,"parent_record_offset":21616,"ancestor_record_offset":18016,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":21672,"parent_record_offset":21644,"ancestor_record_offset":18016,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":18740,"parent_record_offset":18712,"ancestor_record_offset":18016,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":20176,"parent_record_offset":19988,"ancestor_record_offset":18016,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":18428,"parent_record_offset":18396,"ancestor_record_offset":18016,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":20272,"parent_record_offset":20236,"ancestor_record_offset":18016,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":18644,"parent_record_offset":18612,"ancestor_record_offset":18016,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":18016,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":18668,"parent_record_offset":18644,"ancestor_record_offset":18016,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":20200,"parent_record_offset":20176,"ancestor_record_offset":18016,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":20296,"parent_record_offset":20272,"ancestor_record_offset":18016,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":19612,"parent_record_offset":19588,"ancestor_record_offset":18016,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":18016,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":19324,"parent_record_offset":19296,"ancestor_record_offset":18016,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":19356,"parent_record_offset":19324,"ancestor_record_offset":18016,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":19296,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":19156,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":21152,"parent_record_offset":21080,"ancestor_record_offset":18016,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":19680,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":20900,"parent_record_offset":20856,"ancestor_record_offset":18016,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":19240,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":19100,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":21588,"parent_record_offset":20596,"ancestor_record_offset":18016,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":19812,"parent_record_offset":18948,"ancestor_record_offset":18016,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":18948,"parent_record_offset":18916,"ancestor_record_offset":18016,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":18988,"parent_record_offset":18948,"ancestor_record_offset":18016,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19504,"parent_record_offset":19480,"ancestor_record_offset":18016,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19480,"parent_record_offset":19456,"ancestor_record_offset":18016,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21528,"parent_record_offset":21504,"ancestor_record_offset":18016,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21616,"parent_record_offset":20596,"ancestor_record_offset":18016,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19772,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19016,"parent_record_offset":18948,"ancestor_record_offset":18016,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":20596,"parent_record_offset":20568,"ancestor_record_offset":18016,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19840,"parent_record_offset":19812,"ancestor_record_offset":18016,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21504,"parent_record_offset":20596,"ancestor_record_offset":18016,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19044,"parent_record_offset":19016,"ancestor_record_offset":18016,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":20680,"parent_record_offset":20640,"ancestor_record_offset":18016,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":20856,"parent_record_offset":20680,"ancestor_record_offset":18016,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21080,"parent_record_offset":20680,"ancestor_record_offset":18016,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21116,"parent_record_offset":21080,"ancestor_record_offset":18016,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":20640,"parent_record_offset":20596,"ancestor_record_offset":18016,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19128,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21436,"parent_record_offset":21412,"ancestor_record_offset":18016,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":21412,"parent_record_offset":20640,"ancestor_record_offset":18016,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":19456,"parent_record_offset":19044,"ancestor_record_offset":18016,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":20568,"parent_record_offset":20540,"ancestor_record_offset":18016,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":19564,"parent_record_offset":19456,"ancestor_record_offset":18016,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":18612,"parent_record_offset":18584,"ancestor_record_offset":18016,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":18584,"parent_record_offset":18556,"ancestor_record_offset":18016,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":19988,"parent_record_offset":19948,"ancestor_record_offset":18016,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":18712,"parent_record_offset":18516,"ancestor_record_offset":18016,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":18516,"parent_record_offset":18488,"ancestor_record_offset":18016,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":18488,"parent_record_offset":18360,"ancestor_record_offset":18016,"inlinee":13086,"source_line":694,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":19948,"parent_record_offset":19916,"ancestor_record_offset":18016,"inlinee":13088,"source_line":826,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":18396,"parent_record_offset":18360,"ancestor_record_offset":18016,"inlinee":13084,"source_line":172,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":20236,"parent_record_offset":19948,"ancestor_record_offset":18016,"inlinee":13089,"source_line":182,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":517,"record_offset":18556,"parent_record_offset":18516,"ancestor_record_offset":18016,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":26252,"rva":9874272,"va":5378583392}
// 编译实例 RVAs: 0x96ab60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3907192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e88861c4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378583523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0df0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3776853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e8ab5ec6ff","size":5,"soft_fail":false,"target_va":5374807536,"va":5378584384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fc950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3597253","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e83b1cc9ff","size":5,"soft_fail":false,"target_va":5374986576,"va":5378583824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3502345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e8f78ecaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378583844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e8f4e2ffff","size":5,"soft_fail":false,"target_va":5378577040,"va":5378584471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428aa250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32764878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e8cef3f301","size":5,"soft_fail":false,"target_va":5411349072,"va":5378584189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32958726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e806e9f601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378584149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t32954521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e899d8f601","size":5,"soft_fail":false,"target_va":5411538608,"va":5378584082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32956232","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e848dff601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378584051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32955911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378583392,"raw_hex":"e807def601","size":5,"soft_fail":false,"target_va":5411539984,"va":5378584068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":28148,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":27052,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":27148,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":28772,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":26592,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":13083,"source_line":457,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::impl$26::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":27092,"parent_record_offset":26252,"ancestor_record_offset":26252,"inlinee":29629,"source_line":769,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":28648,"parent_record_offset":28624,"ancestor_record_offset":26252,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":28624,"parent_record_offset":28600,"ancestor_record_offset":26252,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":517,"record_offset":28600,"parent_record_offset":28572,"ancestor_record_offset":26252,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":517,"record_offset":27116,"parent_record_offset":27092,"ancestor_record_offset":26252,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":29408,"parent_record_offset":29384,"ancestor_record_offset":26252,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":27412,"parent_record_offset":27388,"ancestor_record_offset":26252,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":26252,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":27436,"parent_record_offset":27412,"ancestor_record_offset":26252,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":27884,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":28104,"parent_record_offset":28072,"ancestor_record_offset":26252,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":29696,"parent_record_offset":29668,"ancestor_record_offset":26252,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":27644,"parent_record_offset":27616,"ancestor_record_offset":26252,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":27616,"parent_record_offset":27588,"ancestor_record_offset":26252,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":27760,"parent_record_offset":27736,"ancestor_record_offset":26252,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":29876,"parent_record_offset":29848,"ancestor_record_offset":26252,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":29904,"parent_record_offset":29876,"ancestor_record_offset":26252,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":26972,"parent_record_offset":26944,"ancestor_record_offset":26252,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":28408,"parent_record_offset":28220,"ancestor_record_offset":26252,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":26660,"parent_record_offset":26628,"ancestor_record_offset":26252,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":28504,"parent_record_offset":28468,"ancestor_record_offset":26252,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":26876,"parent_record_offset":26844,"ancestor_record_offset":26252,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":26252,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":26900,"parent_record_offset":26876,"ancestor_record_offset":26252,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":28432,"parent_record_offset":28408,"ancestor_record_offset":26252,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":28528,"parent_record_offset":28504,"ancestor_record_offset":26252,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":27844,"parent_record_offset":27820,"ancestor_record_offset":26252,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":27820,"parent_record_offset":27796,"ancestor_record_offset":26252,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":27556,"parent_record_offset":27528,"ancestor_record_offset":26252,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":27588,"parent_record_offset":27556,"ancestor_record_offset":26252,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":27528,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":27388,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":29384,"parent_record_offset":29312,"ancestor_record_offset":26252,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":27912,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":29132,"parent_record_offset":29088,"ancestor_record_offset":26252,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":27472,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":27332,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":29820,"parent_record_offset":28828,"ancestor_record_offset":26252,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":28044,"parent_record_offset":27180,"ancestor_record_offset":26252,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":27180,"parent_record_offset":27148,"ancestor_record_offset":26252,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":27220,"parent_record_offset":27180,"ancestor_record_offset":26252,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27736,"parent_record_offset":27712,"ancestor_record_offset":26252,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27712,"parent_record_offset":27688,"ancestor_record_offset":26252,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29760,"parent_record_offset":29736,"ancestor_record_offset":26252,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29848,"parent_record_offset":28828,"ancestor_record_offset":26252,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":28004,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27248,"parent_record_offset":27180,"ancestor_record_offset":26252,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":28828,"parent_record_offset":28800,"ancestor_record_offset":26252,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":28072,"parent_record_offset":28044,"ancestor_record_offset":26252,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29736,"parent_record_offset":28828,"ancestor_record_offset":26252,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27276,"parent_record_offset":27248,"ancestor_record_offset":26252,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":28912,"parent_record_offset":28872,"ancestor_record_offset":26252,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29088,"parent_record_offset":28912,"ancestor_record_offset":26252,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29312,"parent_record_offset":28912,"ancestor_record_offset":26252,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29348,"parent_record_offset":29312,"ancestor_record_offset":26252,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":28872,"parent_record_offset":28828,"ancestor_record_offset":26252,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27360,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29668,"parent_record_offset":29644,"ancestor_record_offset":26252,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":29644,"parent_record_offset":28872,"ancestor_record_offset":26252,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":27688,"parent_record_offset":27276,"ancestor_record_offset":26252,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":28800,"parent_record_offset":28772,"ancestor_record_offset":26252,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":27796,"parent_record_offset":27688,"ancestor_record_offset":26252,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":26844,"parent_record_offset":26816,"ancestor_record_offset":26252,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":26252,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":28220,"parent_record_offset":28180,"ancestor_record_offset":26252,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":26944,"parent_record_offset":26748,"ancestor_record_offset":26252,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":26748,"parent_record_offset":26720,"ancestor_record_offset":26252,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":26720,"parent_record_offset":26592,"ancestor_record_offset":26252,"inlinee":13086,"source_line":694,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":28180,"parent_record_offset":28148,"ancestor_record_offset":26252,"inlinee":13088,"source_line":826,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":26628,"parent_record_offset":26592,"ancestor_record_offset":26252,"inlinee":13084,"source_line":172,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":28468,"parent_record_offset":28180,"ancestor_record_offset":26252,"inlinee":13089,"source_line":182,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":517,"record_offset":26788,"parent_record_offset":26748,"ancestor_record_offset":26252,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 37
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":13552,"rva":9870320,"va":5378579440}
// 编译实例 RVAs: 0x969bf0, 0x25f1bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d0df0
//     owner=unresolved_namespace; count=4
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3772984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e8c86dc6ff","size":5,"soft_fail":false,"target_va":5374807536,"va":5378580515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3498403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e85d9ecaff","size":5,"soft_fail":false,"target_va":5375081504,"va":5378579902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140969160
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e821f0ffff","size":5,"soft_fail":false,"target_va":5378576736,"va":5378580794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cd70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3354741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e875303300","size":5,"soft_fail":false,"target_va":5381934448,"va":5378579702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428accb0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32779548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e81c2df401","size":5,"soft_fail":false,"target_va":5411359920,"va":5378580367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32962588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e81cf8f601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378580287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t32958380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e8ace7f601","size":5,"soft_fail":false,"target_va":5411538608,"va":5378580223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t32960100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e864eef601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378580183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32959776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378579440,"raw_hex":"e820edf601","size":5,"soft_fail":false,"target_va":5411539984,"va":5378580203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5378579440,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5378579879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":15452,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":14408,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":14504,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":16428,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":13948,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":13083,"source_line":457,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::impl$26::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":14448,"parent_record_offset":13552,"ancestor_record_offset":13552,"inlinee":29629,"source_line":769,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":16068,"parent_record_offset":16044,"ancestor_record_offset":13552,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":517,"record_offset":16044,"parent_record_offset":15960,"ancestor_record_offset":13552,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":517,"record_offset":15960,"parent_record_offset":15932,"ancestor_record_offset":13552,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":517,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":13552,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":517,"record_offset":14472,"parent_record_offset":14448,"ancestor_record_offset":13552,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":13552,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":14744,"parent_record_offset":14720,"ancestor_record_offset":13552,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":14828,"parent_record_offset":14804,"ancestor_record_offset":13552,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":14768,"parent_record_offset":14744,"ancestor_record_offset":13552,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":15188,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":15408,"parent_record_offset":15376,"ancestor_record_offset":13552,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":517,"record_offset":16012,"parent_record_offset":15988,"ancestor_record_offset":13552,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":517,"record_offset":17328,"parent_record_offset":17300,"ancestor_record_offset":13552,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":14976,"parent_record_offset":14948,"ancestor_record_offset":13552,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":14948,"parent_record_offset":14920,"ancestor_record_offset":13552,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":15904,"parent_record_offset":15876,"ancestor_record_offset":13552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":13552,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":17508,"parent_record_offset":17480,"ancestor_record_offset":13552,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":17536,"parent_record_offset":17508,"ancestor_record_offset":13552,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":14328,"parent_record_offset":14300,"ancestor_record_offset":13552,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":15712,"parent_record_offset":15524,"ancestor_record_offset":13552,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":14016,"parent_record_offset":13984,"ancestor_record_offset":13552,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":15808,"parent_record_offset":15772,"ancestor_record_offset":13552,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":14232,"parent_record_offset":14200,"ancestor_record_offset":13552,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":14040,"parent_record_offset":14016,"ancestor_record_offset":13552,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":14256,"parent_record_offset":14232,"ancestor_record_offset":13552,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":15736,"parent_record_offset":15712,"ancestor_record_offset":13552,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":15832,"parent_record_offset":15808,"ancestor_record_offset":13552,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$12::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":517,"record_offset":15152,"parent_record_offset":15128,"ancestor_record_offset":13552,"inlinee":11666,"source_line":124,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":14888,"parent_record_offset":14860,"ancestor_record_offset":13552,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":14920,"parent_record_offset":14888,"ancestor_record_offset":13552,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":14860,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":14720,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":17044,"parent_record_offset":16972,"ancestor_record_offset":13552,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":15216,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":16816,"parent_record_offset":16776,"ancestor_record_offset":13552,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":14804,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":14664,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":17452,"parent_record_offset":16500,"ancestor_record_offset":13552,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":15348,"parent_record_offset":14540,"ancestor_record_offset":13552,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":14540,"parent_record_offset":14504,"ancestor_record_offset":13552,"inlinee":7717,"source_line":34,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":15068,"parent_record_offset":15044,"ancestor_record_offset":13552,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":15044,"parent_record_offset":15020,"ancestor_record_offset":13552,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17392,"parent_record_offset":17368,"ancestor_record_offset":13552,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17480,"parent_record_offset":16500,"ancestor_record_offset":13552,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":15308,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":14580,"parent_record_offset":14540,"ancestor_record_offset":13552,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":16500,"parent_record_offset":16464,"ancestor_record_offset":13552,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":15376,"parent_record_offset":15348,"ancestor_record_offset":13552,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17368,"parent_record_offset":16500,"ancestor_record_offset":13552,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":14608,"parent_record_offset":14580,"ancestor_record_offset":13552,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":16592,"parent_record_offset":16548,"ancestor_record_offset":13552,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":16776,"parent_record_offset":16592,"ancestor_record_offset":13552,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":16972,"parent_record_offset":16592,"ancestor_record_offset":13552,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17008,"parent_record_offset":16972,"ancestor_record_offset":13552,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":16548,"parent_record_offset":16500,"ancestor_record_offset":13552,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":14692,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17300,"parent_record_offset":17276,"ancestor_record_offset":13552,"inlinee":5641,"source_line":2419,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":17276,"parent_record_offset":16548,"ancestor_record_offset":13552,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":15020,"parent_record_offset":14608,"ancestor_record_offset":13552,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":16464,"parent_record_offset":16428,"ancestor_record_offset":13552,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":517,"record_offset":15128,"parent_record_offset":15020,"ancestor_record_offset":13552,"inlinee":7719,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":14200,"parent_record_offset":14172,"ancestor_record_offset":13552,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":517,"record_offset":14172,"parent_record_offset":14144,"ancestor_record_offset":13552,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":15524,"parent_record_offset":15484,"ancestor_record_offset":13552,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":14300,"parent_record_offset":14104,"ancestor_record_offset":13552,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":14104,"parent_record_offset":14076,"ancestor_record_offset":13552,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":14076,"parent_record_offset":13948,"ancestor_record_offset":13552,"inlinee":13086,"source_line":694,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$32::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":15484,"parent_record_offset":15452,"ancestor_record_offset":13552,"inlinee":13088,"source_line":826,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":13984,"parent_record_offset":13948,"ancestor_record_offset":13552,"inlinee":13084,"source_line":172,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":15772,"parent_record_offset":15484,"ancestor_record_offset":13552,"inlinee":13089,"source_line":182,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":517,"record_offset":14144,"parent_record_offset":14104,"ancestor_record_offset":13552,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":6656,"rva":9868624,"va":5378577744}
// 编译实例 RVAs: 0x969550
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3901497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577744,"raw_hex":"e8c777c4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378577828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32964770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577744,"raw_hex":"e8a200f701","size":5,"soft_fail":false,"target_va":5411542880,"va":5378578105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32961461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577744,"raw_hex":"e8b5f3f601","size":5,"soft_fail":false,"target_va":5411539504,"va":5378578038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32962165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577744,"raw_hex":"e875f6f601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378578118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":517,"record_offset":8640,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":8668,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":7408,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":7644,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":7028,"parent_record_offset":6656,"ancestor_record_offset":6656,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":6656,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":6656,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":6656,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":6656,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":8464,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":8168,"parent_record_offset":8140,"ancestor_record_offset":6656,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":6656,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":7452,"parent_record_offset":7408,"ancestor_record_offset":6656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":8284,"parent_record_offset":8260,"ancestor_record_offset":6656,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7328,"parent_record_offset":7300,"ancestor_record_offset":6656,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7184,"parent_record_offset":7064,"ancestor_record_offset":6656,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7568,"parent_record_offset":7536,"ancestor_record_offset":6656,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7100,"parent_record_offset":7064,"ancestor_record_offset":6656,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7208,"parent_record_offset":7184,"ancestor_record_offset":6656,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6656,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":7592,"parent_record_offset":7568,"ancestor_record_offset":6656,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":8416,"parent_record_offset":8392,"ancestor_record_offset":6656,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":6656,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":8080,"parent_record_offset":8052,"ancestor_record_offset":6656,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":8112,"parent_record_offset":8080,"ancestor_record_offset":6656,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":8052,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":7912,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":8492,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":7996,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":7884,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":517,"record_offset":8344,"parent_record_offset":8320,"ancestor_record_offset":6656,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":6656,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":6656,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":8320,"parent_record_offset":8212,"ancestor_record_offset":6656,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":7724,"parent_record_offset":7692,"ancestor_record_offset":6656,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":6656,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":6656,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":8584,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":6656,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":7752,"parent_record_offset":7692,"ancestor_record_offset":6656,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":7804,"parent_record_offset":7776,"ancestor_record_offset":6656,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":7856,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":8212,"parent_record_offset":7804,"ancestor_record_offset":6656,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":7272,"parent_record_offset":7244,"ancestor_record_offset":6656,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":7300,"parent_record_offset":7272,"ancestor_record_offset":6656,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":7244,"parent_record_offset":7028,"ancestor_record_offset":6656,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":6656,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":7064,"parent_record_offset":7028,"ancestor_record_offset":6656,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":7536,"parent_record_offset":7508,"ancestor_record_offset":6656,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":7156,"parent_record_offset":7064,"ancestor_record_offset":6656,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":11320,"rva":9869856,"va":5378578976}
// 编译实例 RVAs: 0x969a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3902729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378578976,"raw_hex":"e8f772c4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378579060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32963538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378578976,"raw_hex":"e8d2fbf601","size":5,"soft_fail":false,"target_va":5411542880,"va":5378579337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32960229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378578976,"raw_hex":"e8e5eef601","size":5,"soft_fail":false,"target_va":5411539504,"va":5378579270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32960933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378578976,"raw_hex":"e8a5f1f601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378579350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":517,"record_offset":13360,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":13388,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":12128,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":12364,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":11748,"parent_record_offset":11320,"ancestor_record_offset":11320,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":13088,"parent_record_offset":13064,"ancestor_record_offset":11320,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":12656,"parent_record_offset":12632,"ancestor_record_offset":11320,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":12740,"parent_record_offset":12716,"ancestor_record_offset":11320,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":12680,"parent_record_offset":12656,"ancestor_record_offset":11320,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":13184,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":12888,"parent_record_offset":12860,"ancestor_record_offset":11320,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":12860,"parent_record_offset":12832,"ancestor_record_offset":11320,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":12172,"parent_record_offset":12128,"ancestor_record_offset":11320,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":11320,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":12048,"parent_record_offset":12020,"ancestor_record_offset":11320,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":11904,"parent_record_offset":11784,"ancestor_record_offset":11320,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":12288,"parent_record_offset":12256,"ancestor_record_offset":11320,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":11820,"parent_record_offset":11784,"ancestor_record_offset":11320,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":11320,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":11844,"parent_record_offset":11820,"ancestor_record_offset":11320,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":11320,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":13136,"parent_record_offset":13112,"ancestor_record_offset":11320,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":13112,"parent_record_offset":13088,"ancestor_record_offset":11320,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":12800,"parent_record_offset":12772,"ancestor_record_offset":11320,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":12832,"parent_record_offset":12800,"ancestor_record_offset":11320,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":12772,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":12632,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":13212,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":12716,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":12604,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":517,"record_offset":13064,"parent_record_offset":13040,"ancestor_record_offset":11320,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":12388,"parent_record_offset":12364,"ancestor_record_offset":11320,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":12412,"parent_record_offset":12388,"ancestor_record_offset":11320,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":13040,"parent_record_offset":12932,"ancestor_record_offset":11320,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":12444,"parent_record_offset":12412,"ancestor_record_offset":11320,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":11320,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12956,"parent_record_offset":12932,"ancestor_record_offset":11320,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":13304,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12496,"parent_record_offset":12472,"ancestor_record_offset":11320,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12472,"parent_record_offset":12412,"ancestor_record_offset":11320,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":11320,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12576,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":12932,"parent_record_offset":12524,"ancestor_record_offset":11320,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":11992,"parent_record_offset":11964,"ancestor_record_offset":11320,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":12020,"parent_record_offset":11992,"ancestor_record_offset":11320,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":11964,"parent_record_offset":11748,"ancestor_record_offset":11320,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":12228,"parent_record_offset":12200,"ancestor_record_offset":11320,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":11784,"parent_record_offset":11748,"ancestor_record_offset":11320,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":12256,"parent_record_offset":12228,"ancestor_record_offset":11320,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":11876,"parent_record_offset":11784,"ancestor_record_offset":11320,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":517,"record_offset":4428,"rva":9868160,"va":5378577280}
// 编译实例 RVAs: 0x969380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3901033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577280,"raw_hex":"e89779c4ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5378577364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32965234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577280,"raw_hex":"e87202f701","size":5,"soft_fail":false,"target_va":5411542880,"va":5378577641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32961925","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577280,"raw_hex":"e885f5f601","size":5,"soft_fail":false,"target_va":5411539504,"va":5378577574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t32962629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5378577280,"raw_hex":"e845f8f601","size":5,"soft_fail":false,"target_va":5411540288,"va":5378577654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":517,"record_offset":6464,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":6492,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":517,"record_offset":5232,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":517,"record_offset":5468,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":4852,"parent_record_offset":4428,"ancestor_record_offset":4428,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":517,"record_offset":6192,"parent_record_offset":6168,"ancestor_record_offset":4428,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":4428,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":517,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":4428,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4428,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":517,"record_offset":6288,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":5992,"parent_record_offset":5964,"ancestor_record_offset":4428,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":517,"record_offset":5964,"parent_record_offset":5936,"ancestor_record_offset":4428,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":517,"record_offset":5276,"parent_record_offset":5232,"ancestor_record_offset":4428,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":517,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":4428,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":5152,"parent_record_offset":5124,"ancestor_record_offset":4428,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":5008,"parent_record_offset":4888,"ancestor_record_offset":4428,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":5392,"parent_record_offset":5360,"ancestor_record_offset":4428,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":4428,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":5032,"parent_record_offset":5008,"ancestor_record_offset":4428,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":4948,"parent_record_offset":4924,"ancestor_record_offset":4428,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":517,"record_offset":5416,"parent_record_offset":5392,"ancestor_record_offset":4428,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$17::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":6240,"parent_record_offset":6216,"ancestor_record_offset":4428,"inlinee":26662,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$8::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":517,"record_offset":6216,"parent_record_offset":6192,"ancestor_record_offset":4428,"inlinee":26660,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":4428,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":4428,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":517,"record_offset":5876,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":5736,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":6316,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":517,"record_offset":5820,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":517,"record_offset":5708,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":517,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":4428,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":5492,"parent_record_offset":5468,"ancestor_record_offset":4428,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":5516,"parent_record_offset":5492,"ancestor_record_offset":4428,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":6144,"parent_record_offset":6036,"ancestor_record_offset":4428,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":517,"record_offset":5548,"parent_record_offset":5516,"ancestor_record_offset":4428,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":6084,"parent_record_offset":6060,"ancestor_record_offset":4428,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":4428,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":6408,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":5600,"parent_record_offset":5576,"ancestor_record_offset":4428,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":5576,"parent_record_offset":5516,"ancestor_record_offset":4428,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":5628,"parent_record_offset":5600,"ancestor_record_offset":4428,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":5680,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":517,"record_offset":6036,"parent_record_offset":5628,"ancestor_record_offset":4428,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":4428,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":517,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":4428,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":5068,"parent_record_offset":4852,"ancestor_record_offset":4428,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":517,"record_offset":5332,"parent_record_offset":5304,"ancestor_record_offset":4428,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":4428,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":5360,"parent_record_offset":5332,"ancestor_record_offset":4428,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":517,"record_offset":4980,"parent_record_offset":4888,"ancestor_record_offset":4428,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00517.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
// 记录形式: compiled_procedure; instances=13
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":250,"record_offset":34108,"rva":6072432,"va":5374781552}
// 编译实例 RVAs: 0x5ca870, 0x969720, 0x1209c40, 0x1258610, 0x1cf6a60, 0x21aeb40, 0x21b5d00, 0x22782d0, 0x230e780, 0x23a3280, 0x25dad90, 0x25df9c0, 0x25f18b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cd70
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7152662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374781552,"raw_hex":"e816246d00","size":5,"soft_fail":false,"target_va":5381934448,"va":5374781781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t36760851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374781552,"raw_hex":"e813ed3002","size":5,"soft_fail":false,"target_va":5411542880,"va":5374782024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t36757543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374781552,"raw_hex":"e827e03002","size":5,"soft_fail":false,"target_va":5411539504,"va":5374781956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=39
//     evidence={"assembly":"\tcallq\t36758246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374781552,"raw_hex":"e8e6e23002","size":5,"soft_fail":false,"target_va":5411540288,"va":5374782037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":250,"record_offset":36244,"parent_record_offset":34108,"ancestor_record_offset":34108,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":250,"record_offset":36272,"parent_record_offset":34108,"ancestor_record_offset":34108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":250,"record_offset":34928,"parent_record_offset":34108,"ancestor_record_offset":34108,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":250,"record_offset":35168,"parent_record_offset":34108,"ancestor_record_offset":34108,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::read
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":250,"record_offset":34548,"parent_record_offset":34108,"ancestor_record_offset":34108,"inlinee":8101,"source_line":367,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":250,"record_offset":36000,"parent_record_offset":35976,"ancestor_record_offset":34108,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":250,"record_offset":35568,"parent_record_offset":35544,"ancestor_record_offset":34108,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":250,"record_offset":35652,"parent_record_offset":35628,"ancestor_record_offset":34108,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":250,"record_offset":35268,"parent_record_offset":35244,"ancestor_record_offset":34108,"inlinee":12248,"source_line":694,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":250,"record_offset":35292,"parent_record_offset":35268,"ancestor_record_offset":34108,"inlinee":6316,"source_line":946,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":250,"record_offset":35592,"parent_record_offset":35568,"ancestor_record_offset":34108,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":250,"record_offset":36068,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":250,"record_offset":35800,"parent_record_offset":35772,"ancestor_record_offset":34108,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":250,"record_offset":35772,"parent_record_offset":35744,"ancestor_record_offset":34108,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":250,"record_offset":34972,"parent_record_offset":34928,"ancestor_record_offset":34108,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":250,"record_offset":35916,"parent_record_offset":35892,"ancestor_record_offset":34108,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":34848,"parent_record_offset":34820,"ancestor_record_offset":34108,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":34704,"parent_record_offset":34584,"ancestor_record_offset":34108,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":35092,"parent_record_offset":35056,"ancestor_record_offset":34108,"inlinee":7030,"source_line":2986,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":34620,"parent_record_offset":34584,"ancestor_record_offset":34108,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":34728,"parent_record_offset":34704,"ancestor_record_offset":34108,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":34644,"parent_record_offset":34620,"ancestor_record_offset":34108,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":250,"record_offset":35116,"parent_record_offset":35092,"ancestor_record_offset":34108,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$10::hash
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":250,"record_offset":35316,"parent_record_offset":35292,"ancestor_record_offset":34108,"inlinee":15951,"source_line":124,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$12::eq
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":250,"record_offset":36024,"parent_record_offset":36000,"ancestor_record_offset":34108,"inlinee":11666,"source_line":124,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":250,"record_offset":35712,"parent_record_offset":35684,"ancestor_record_offset":34108,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":250,"record_offset":35744,"parent_record_offset":35712,"ancestor_record_offset":34108,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":250,"record_offset":35684,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":250,"record_offset":35544,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":250,"record_offset":36096,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=26
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":250,"record_offset":35628,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":250,"record_offset":35516,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":250,"record_offset":35976,"parent_record_offset":35952,"ancestor_record_offset":34108,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":250,"record_offset":35192,"parent_record_offset":35168,"ancestor_record_offset":34108,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":250,"record_offset":35216,"parent_record_offset":35192,"ancestor_record_offset":34108,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":250,"record_offset":35952,"parent_record_offset":35844,"ancestor_record_offset":34108,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":250,"record_offset":35244,"parent_record_offset":35216,"ancestor_record_offset":34108,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35892,"parent_record_offset":35868,"ancestor_record_offset":34108,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35868,"parent_record_offset":35844,"ancestor_record_offset":34108,"inlinee":4721,"source_line":522,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":36188,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35408,"parent_record_offset":35384,"ancestor_record_offset":34108,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35384,"parent_record_offset":35216,"ancestor_record_offset":34108,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35436,"parent_record_offset":35408,"ancestor_record_offset":34108,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35488,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=13
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":250,"record_offset":35844,"parent_record_offset":35436,"ancestor_record_offset":34108,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":250,"record_offset":34792,"parent_record_offset":34764,"ancestor_record_offset":34108,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":250,"record_offset":34820,"parent_record_offset":34792,"ancestor_record_offset":34108,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":250,"record_offset":34764,"parent_record_offset":34548,"ancestor_record_offset":34108,"inlinee":7022,"source_line":682,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::rwlock::impl$31::drop
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":250,"record_offset":35028,"parent_record_offset":35000,"ancestor_record_offset":34108,"inlinee":7028,"source_line":816,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":250,"record_offset":34584,"parent_record_offset":34548,"ancestor_record_offset":34108,"inlinee":7017,"source_line":94,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":250,"record_offset":35056,"parent_record_offset":35028,"ancestor_record_offset":34108,"inlinee":7029,"source_line":110,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":250,"record_offset":34676,"parent_record_offset":34584,"ancestor_record_offset":34108,"inlinee":7019,"source_line":47,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   [pdb_inline_transitive] utils::name_hash::impl$0::hash
//     owner=private_source_ownership_candidate; count=13
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/name_hash.rs
//     evidence={"module":250,"record_offset":35340,"parent_record_offset":35316,"ancestor_record_offset":34108,"inlinee":8234,"source_line":11,"archive":"_pdb/B/modules/00250.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::as_mut
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 114
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x704D
// 首条位置: {"module":506,"inlinee":28749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":506,"record_offset":61112,"parent_record_offset":60908,"ancestor_record_offset":60908,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":506,"record_offset":60936,"parent_record_offset":60908,"ancestor_record_offset":60908,"inlinee":5618,"source_line":631,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":506,"record_offset":61080,"parent_record_offset":60936,"ancestor_record_offset":60908,"inlinee":15035,"source_line":2219,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":506,"record_offset":61016,"parent_record_offset":60992,"ancestor_record_offset":60908,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":506,"record_offset":61040,"parent_record_offset":61016,"ancestor_record_offset":60908,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":506,"record_offset":60964,"parent_record_offset":60936,"ancestor_record_offset":60908,"inlinee":5620,"source_line":116,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":506,"record_offset":60992,"parent_record_offset":60964,"ancestor_record_offset":60908,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::impl$7::default
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 148
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x704C
// 首条位置: {"module":506,"inlinee":28748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":508,"record_offset":52276,"parent_record_offset":52240,"ancestor_record_offset":52240,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::rwlock::RwLock::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\rwlock.rs
//     evidence={"module":506,"record_offset":60684,"parent_record_offset":60660,"ancestor_record_offset":60660,"inlinee":15036,"source_line":221,"archive":"_pdb/B/modules/00506.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":508,"record_offset":52472,"parent_record_offset":52448,"ancestor_record_offset":52240,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":508,"record_offset":52524,"parent_record_offset":52500,"ancestor_record_offset":52240,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":508,"record_offset":52500,"parent_record_offset":52448,"ancestor_record_offset":52240,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":508,"record_offset":52548,"parent_record_offset":52524,"ancestor_record_offset":52240,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":508,"record_offset":52420,"parent_record_offset":52392,"ancestor_record_offset":52240,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":508,"record_offset":52572,"parent_record_offset":52548,"ancestor_record_offset":52240,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":508,"record_offset":52332,"parent_record_offset":52304,"ancestor_record_offset":52240,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":508,"record_offset":52448,"parent_record_offset":52392,"ancestor_record_offset":52240,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":508,"record_offset":52304,"parent_record_offset":52276,"ancestor_record_offset":52240,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":508,"record_offset":52392,"parent_record_offset":52360,"ancestor_record_offset":52240,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00508.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":508,"record_offset":52360,"parent_record_offset":52332,"ancestor_record_offset":52240,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00508.jsonl.gz"}
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
// [PDB 命名实体；类型/枚举变体身份待确认] frozen::FrozenMap
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 21
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >", "country_core::asset_libraries::ssbo_library::SsboId"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "frozen::FrozenMap", "status": "evidence_association; original field ownership unknown"}]
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "frozen::FrozenMap", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 19; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::terrain_stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>,bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>", "bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::trigger_audio::TriggerAudioState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::as_mut
// 信息 ID: 1914945145f80d38be6ef7c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg; module=506, inline_record=60908; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut; module=508, inline_record=59124; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::remove; module=508, inline_record=62960; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::sparkle::SparkleSsbo>; module=1422, inline_record=520; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1422, inline_record=2072; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1579, inline_record=84316; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>; module=1579, inline_record=85892; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=2280, inline_record=516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >
// 信息 ID: 3a0c17f2272081d447e7ff1e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::mesh::Mesh", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::backend::vao_update::build_missing_vaos @ 0x86492d -> 0x969550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::vao_library::AssetVAOLibrary::add @ 0x92427c -> 0x969550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::vao_library::AssetVAOLibrary::rebuild_vao @ 0x92442a -> 0x969550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get @ 0x92d2f2 -> 0x969550 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name @ 0x92d47c -> 0x969550 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$57,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x9b0ccd -> 0x969550 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >
// 信息 ID: 3f6e651581ee5bfa7b8a53bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::ComputeShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name @ 0x937a6b -> 0x969a20 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x169b01a -> 0x969a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >
// 信息 ID: 05fbc250c43c421404fb02ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::RasterShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$0::drop @ 0x89a1db -> 0x969380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x905c41 -> 0x969380 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster @ 0x9371c4 -> 0x969380 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697da8 -> 0x969380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
// 信息 ID: ebd3e6402dc0f03aee67a6e0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 63e698043556bbae6bbed0d8: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsArchSupportRequests>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::visual::StairsVisualState>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>,bevy_ecs::change_detection::Res<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls> >)
// [类型关系] alloc::boxed::Box -> ["country_core::asset_libraries::ssbo_library::BufferState", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] nohash_hasher::NoHashHasher -> ["country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// [类型关系] core::hash::BuildHasherDefault -> ["nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >", "country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 402 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x9021 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x90a3 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x9125 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x183d1 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x18453 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x184d5 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b22f4 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b24f8 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b26e5 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b31d2 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b3274 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x2b32e8 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b3502 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b35a4 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x2b3618 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x2b3832 -> 0x5ca870 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: 32cfae1baedd95a7a988f172
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::mesh::Mesh", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add @ 0x92cdba -> 0x96a690 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: f3e5d29739331542db00f8dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::ComputeShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_compute @ 0x9376f7 -> 0x96a1c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: b514a7cdd4ca648df2d9b057
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::RasterShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster @ 0x93713d -> 0x96ab60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >
// 信息 ID: 9fa6c18e08e0a56b3a234977
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::boxed::Box -> ["country_core::asset_libraries::ssbo_library::BufferState", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] nohash_hasher::NoHashHasher -> ["country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// [类型关系] core::hash::BuildHasherDefault -> ["nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 82 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> @ 0x12baed3 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> @ 0x12bb153 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniDrawListSsbo> @ 0x12bb3d7 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x12bb653 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo> @ 0x12bb8d6 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO> @ 0x12bbb53 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x12bbdd6 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniListDrawCmdOffsets> @ 0x12bc053 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO> @ 0x12bc2d7 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::resources::critters::FireflyVisibilitySsbo> @ 0x12bc55a -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::geometry::instanced_wall::BrickPrevVisibilityBitmaskSsbo> @ 0x12bc7d2 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo> @ 0x12bca5a -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo> @ 0x12bccda -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> @ 0x12bcf52 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListLengthSsbo> @ 0x12bd1d2 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0x12bd452 -> 0x969bf0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::impl$7::default
// 信息 ID: c82608afbf3f8eb867a1998f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::impl$6::default; module=506, inline_record=60660; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::new; module=508, inline_record=52240; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::new; module=509, inline_record=48384; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::new; module=509, inline_record=49568; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::impl$0::insert_rendering_resources; module=511, inline_record=21260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
