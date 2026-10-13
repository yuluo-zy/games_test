// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/managed/texture.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3d904dbd2cf2bebc2fa520ca5cabe4e21d4b980bb9e8cec626af9ee4d76ddd1e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 684, 1920, 1962
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.7.1"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use backtrace; // namespace candidate
//   package identities: [{"name":"backtrace","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.71"}]
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::impl$0::dispose
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":684,"record_offset":2024,"rva":12435008,"va":5381144128}
// 编译实例 RVAs: 0xbdbe40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12169069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8935046ff","size":5,"soft_fail":false,"target_va":5368976080,"va":5381145144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e884330000","size":5,"soft_fail":false,"target_va":5381158416,"va":5381145223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30702723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8837cd401","size":5,"soft_fail":false,"target_va":5411847200,"va":5381144472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-6788877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8f36898ff","size":5,"soft_fail":false,"target_va":5374355568,"va":5381144440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30340199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e867f4ce01","size":5,"soft_fail":false,"target_va":5411485168,"va":5381144964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30342600","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8c8fdce01","size":5,"soft_fail":false,"target_va":5411487584,"va":5381144979,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t679400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8e85d0a00","size":5,"soft_fail":false,"target_va":5381823600,"va":5381144195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t679461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381144128,"raw_hex":"e8255e0a00","size":5,"soft_fail":false,"target_va":5381824016,"va":5381144550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::Handle::is_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":684,"record_offset":5112,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":39255,"source_line":63,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":684,"record_offset":4476,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":3844,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":2180,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":3596,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":39247,"source_line":143,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":2392,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":10288,"source_line":635,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$76::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":2892,"parent_record_offset":2024,"ancestor_record_offset":2024,"inlinee":11902,"source_line":2712,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":2024,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":2024,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":684,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":2024,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3692,"parent_record_offset":3652,"ancestor_record_offset":2024,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":684,"record_offset":3652,"parent_record_offset":3624,"ancestor_record_offset":2024,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":684,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":2024,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":684,"record_offset":2720,"parent_record_offset":2688,"ancestor_record_offset":2024,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":684,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2024,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":684,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":2024,"inlinee":6028,"source_line":4607,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":684,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2024,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":684,"record_offset":3200,"parent_record_offset":3168,"ancestor_record_offset":2024,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":684,"record_offset":3168,"parent_record_offset":3136,"ancestor_record_offset":2024,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":684,"record_offset":2756,"parent_record_offset":2600,"ancestor_record_offset":2024,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":2024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":4156,"parent_record_offset":3916,"ancestor_record_offset":2024,"inlinee":9631,"source_line":1475,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":3460,"parent_record_offset":3436,"ancestor_record_offset":2024,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":2024,"inlinee":6027,"source_line":1524,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":4540,"parent_record_offset":4504,"ancestor_record_offset":2024,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":2844,"parent_record_offset":2820,"ancestor_record_offset":2024,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":4408,"parent_record_offset":4284,"ancestor_record_offset":2024,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":2244,"parent_record_offset":2208,"ancestor_record_offset":2024,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":4432,"parent_record_offset":4408,"ancestor_record_offset":2024,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":2024,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":3096,"parent_record_offset":3056,"ancestor_record_offset":2024,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":3136,"parent_record_offset":3096,"ancestor_record_offset":2024,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":3244,"parent_record_offset":3056,"ancestor_record_offset":2024,"inlinee":4830,"source_line":37,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":3056,"parent_record_offset":3008,"ancestor_record_offset":2024,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":2632,"parent_record_offset":2600,"ancestor_record_offset":2024,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":2688,"parent_record_offset":2632,"ancestor_record_offset":2024,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":3336,"parent_record_offset":3008,"ancestor_record_offset":2024,"inlinee":4817,"source_line":117,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::impl$2::fill_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":684,"record_offset":4036,"parent_record_offset":3976,"ancestor_record_offset":2024,"inlinee":6225,"source_line":79,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":2024,"inlinee":10289,"source_line":888,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$70::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":2920,"parent_record_offset":2892,"ancestor_record_offset":2024,"inlinee":11904,"source_line":3437,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":3436,"parent_record_offset":3008,"ancestor_record_offset":2024,"inlinee":4833,"source_line":439,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2600,"parent_record_offset":2508,"ancestor_record_offset":2024,"inlinee":4816,"source_line":3397,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":3008,"parent_record_offset":2976,"ancestor_record_offset":2024,"inlinee":4829,"source_line":3464,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2024,"inlinee":6290,"source_line":1313,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2796,"parent_record_offset":2448,"ancestor_record_offset":2024,"inlinee":6279,"source_line":1328,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2480,"parent_record_offset":2448,"ancestor_record_offset":2024,"inlinee":4813,"source_line":1288,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2572,"parent_record_offset":2508,"ancestor_record_offset":2024,"inlinee":4952,"source_line":2488,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":3944,"parent_record_offset":3916,"ancestor_record_offset":2024,"inlinee":10626,"source_line":3020,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::ctrl_slice
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":3976,"parent_record_offset":3944,"ancestor_record_offset":2024,"inlinee":10627,"source_line":2482,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2544,"parent_record_offset":2508,"ancestor_record_offset":2024,"inlinee":4951,"source_line":2291,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2508,"parent_record_offset":2480,"ancestor_record_offset":2024,"inlinee":4814,"source_line":1991,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":4008,"parent_record_offset":3976,"ancestor_record_offset":2024,"inlinee":6148,"source_line":2504,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":4124,"parent_record_offset":3944,"ancestor_record_offset":2024,"inlinee":6127,"source_line":128,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2024,"inlinee":4828,"source_line":3661,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$47::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":2024,"inlinee":11906,"source_line":3937,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":2948,"parent_record_offset":2920,"ancestor_record_offset":2024,"inlinee":6284,"source_line":3958,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":4256,"parent_record_offset":4228,"ancestor_record_offset":2024,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":2300,"parent_record_offset":2208,"ancestor_record_offset":2024,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":4320,"parent_record_offset":4284,"ancestor_record_offset":2024,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":2024,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":2024,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":684,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":2024,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":684,"record_offset":4348,"parent_record_offset":4320,"ancestor_record_offset":2024,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$13::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":4876,"parent_record_offset":4848,"ancestor_record_offset":2024,"inlinee":39253,"source_line":346,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$4::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":2024,"inlinee":39251,"source_line":292,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$5::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":684,"record_offset":3624,"parent_record_offset":3596,"ancestor_record_offset":2024,"inlinee":39249,"source_line":298,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::impl$0::debug_name
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15117
// 首条位置: {"module":1920,"inlinee":86295}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::ManagedTexture::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":684,"record_offset":5764,"rva":12436176,"va":5381145296}
// 编译实例 RVAs: 0xbdc2d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdcba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e831080000","size":5,"soft_fail":false,"target_va":5381147552,"va":5381145450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e895300000","size":5,"soft_fail":false,"target_va":5381158416,"va":5381145974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t236228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e8c49a0300","size":5,"soft_fail":false,"target_va":5381381952,"va":5381145719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13501305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e87903ce00","size":5,"soft_fail":false,"target_va":5394646992,"va":5381145682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1512336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e890131700","size":5,"soft_fail":false,"target_va":5382657856,"va":5381145515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1608058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381145296,"raw_hex":"e87a891800","size":5,"soft_fail":false,"target_va":5382753792,"va":5381145729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6756,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":684,"record_offset":5940,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":684,"record_offset":6728,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":5976,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":12821,"source_line":1151,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":684,"record_offset":5916,"parent_record_offset":5764,"ancestor_record_offset":5764,"inlinee":26711,"source_line":251,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":684,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":5764,"inlinee":7989,"source_line":1446,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":684,"record_offset":7016,"parent_record_offset":6992,"ancestor_record_offset":5764,"inlinee":7990,"source_line":1684,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":684,"record_offset":7184,"parent_record_offset":7160,"ancestor_record_offset":5764,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":684,"record_offset":6884,"parent_record_offset":6860,"ancestor_record_offset":5764,"inlinee":7948,"source_line":81,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::impl$7::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":684,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":5764,"inlinee":16049,"source_line":327,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":684,"record_offset":6916,"parent_record_offset":6828,"ancestor_record_offset":5764,"inlinee":7987,"source_line":86,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":684,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":5764,"inlinee":7988,"source_line":121,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":684,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":5764,"inlinee":7998,"source_line":169,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":684,"record_offset":6860,"parent_record_offset":6828,"ancestor_record_offset":5764,"inlinee":8000,"source_line":196,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":684,"record_offset":6552,"parent_record_offset":6520,"ancestor_record_offset":5764,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":684,"record_offset":6612,"parent_record_offset":6580,"ancestor_record_offset":5764,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":684,"record_offset":7112,"parent_record_offset":6916,"ancestor_record_offset":5764,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":684,"record_offset":6408,"parent_record_offset":6384,"ancestor_record_offset":5764,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":6580,"parent_record_offset":6520,"ancestor_record_offset":5764,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":684,"record_offset":6156,"parent_record_offset":6116,"ancestor_record_offset":5764,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":684,"record_offset":6272,"parent_record_offset":6244,"ancestor_record_offset":5764,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":684,"record_offset":7040,"parent_record_offset":7016,"ancestor_record_offset":5764,"inlinee":7991,"source_line":1097,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":684,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":5764,"inlinee":7992,"source_line":1039,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":5764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":6360,"parent_record_offset":6244,"ancestor_record_offset":5764,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":5764,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5764,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":6464,"parent_record_offset":6408,"ancestor_record_offset":5764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7220,"parent_record_offset":7184,"ancestor_record_offset":5764,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":5764,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7276,"parent_record_offset":7184,"ancestor_record_offset":5764,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":6004,"parent_record_offset":5976,"ancestor_record_offset":5764,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":684,"record_offset":6060,"parent_record_offset":6032,"ancestor_record_offset":5764,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":684,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":5764,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":684,"record_offset":6184,"parent_record_offset":6156,"ancestor_record_offset":5764,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":684,"record_offset":6520,"parent_record_offset":6116,"ancestor_record_offset":5764,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":684,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":5764,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":684,"record_offset":6212,"parent_record_offset":6184,"ancestor_record_offset":5764,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":684,"record_offset":6244,"parent_record_offset":6212,"ancestor_record_offset":5764,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":684,"record_offset":6116,"parent_record_offset":6088,"ancestor_record_offset":5764,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":684,"record_offset":6088,"parent_record_offset":6060,"ancestor_record_offset":5764,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::ManagedTexture::view
// 记录形式: compiled_procedure; instances=1
// 源码行: 73
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":684,"record_offset":7400,"rva":12436944,"va":5381146064}
// 编译实例 RVAs: 0xbdc5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140041280
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12171515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e8054746ff","size":5,"soft_fail":false,"target_va":5368976000,"va":5381147510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400412d0
//     owner=unresolved_namespace; count=1
//     target_aliases=39 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12171339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e8b54746ff","size":5,"soft_fail":false,"target_va":5368976080,"va":5381147414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdcba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e89b030000","size":5,"soft_fail":false,"target_va":5381147552,"va":5381146624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ad1d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13437271","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e85709cd00","size":5,"soft_fail":false,"target_va":5394584016,"va":5381146740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bd030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13502836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e87409ce00","size":5,"soft_fail":false,"target_va":5394649136,"va":5381146295,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bd600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13504205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e8cd0ece00","size":5,"soft_fail":false,"target_va":5394650624,"va":5381146414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30394644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e814c9cf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381147255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30337894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e866ebce01","size":5,"soft_fail":false,"target_va":5411485168,"va":5381147269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30342750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e85efece01","size":5,"soft_fail":false,"target_va":5411488944,"va":5381146189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30340295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e8c7f4ce01","size":5,"soft_fail":false,"target_va":5411487584,"va":5381147284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30344807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e86706cf01","size":5,"soft_fail":false,"target_va":5411491664,"va":5381146852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t677356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e8ec550a00","size":5,"soft_fail":false,"target_va":5381823600,"va":5381146239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t677504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381146064,"raw_hex":"e880560a00","size":5,"soft_fail":false,"target_va":5381824016,"va":5381146507,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::Handle::is_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":684,"record_offset":7916,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":39255,"source_line":63,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":684,"record_offset":11172,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":8892,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":684,"record_offset":9140,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":684,"record_offset":10896,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":7576,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":5820,"source_line":447,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":9176,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":9464,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":24873,"source_line":2798,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":9396,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":7716,"source_line":872,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":7944,"parent_record_offset":7400,"ancestor_record_offset":7400,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":684,"record_offset":10128,"parent_record_offset":10104,"ancestor_record_offset":7400,"inlinee":4832,"source_line":1214,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":684,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":7400,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":684,"record_offset":8320,"parent_record_offset":8296,"ancestor_record_offset":7400,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":684,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":7400,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":684,"record_offset":8716,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":7732,"parent_record_offset":7640,"ancestor_record_offset":7400,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":684,"record_offset":10396,"parent_record_offset":10360,"ancestor_record_offset":7400,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":684,"record_offset":8472,"parent_record_offset":8444,"ancestor_record_offset":7400,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":684,"record_offset":8444,"parent_record_offset":8416,"ancestor_record_offset":7400,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":7400,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":684,"record_offset":10584,"parent_record_offset":10560,"ancestor_record_offset":7400,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":684,"record_offset":10608,"parent_record_offset":10584,"ancestor_record_offset":7400,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":11104,"parent_record_offset":10980,"ancestor_record_offset":7400,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7760,"parent_record_offset":7640,"ancestor_record_offset":7400,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":9072,"parent_record_offset":8948,"ancestor_record_offset":7400,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7676,"parent_record_offset":7640,"ancestor_record_offset":7400,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":7400,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":7400,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":7700,"parent_record_offset":7676,"ancestor_record_offset":7400,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":684,"record_offset":9096,"parent_record_offset":9072,"ancestor_record_offset":7400,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":8380,"parent_record_offset":8352,"ancestor_record_offset":7400,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":8416,"parent_record_offset":8380,"ancestor_record_offset":7400,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":684,"record_offset":8352,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":8212,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":10104,"parent_record_offset":10032,"ancestor_record_offset":7400,"inlinee":4831,"source_line":59,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":8744,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty_or_deleted
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":9876,"parent_record_offset":9836,"ancestor_record_offset":7400,"inlinee":4818,"source_line":101,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":684,"record_offset":8296,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":684,"record_offset":8184,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::special_is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":684,"record_offset":10532,"parent_record_offset":9564,"ancestor_record_offset":7400,"inlinee":5667,"source_line":28,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":7968,"parent_record_offset":7944,"ancestor_record_offset":7400,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":7992,"parent_record_offset":7968,"ancestor_record_offset":7400,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":8684,"parent_record_offset":8516,"ancestor_record_offset":7400,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":684,"record_offset":8024,"parent_record_offset":7992,"ancestor_record_offset":7400,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8624,"parent_record_offset":8540,"ancestor_record_offset":7400,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8568,"parent_record_offset":8540,"ancestor_record_offset":7400,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":10560,"parent_record_offset":9564,"ancestor_record_offset":7400,"inlinee":5669,"source_line":510,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8836,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":7400,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":7400,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8052,"parent_record_offset":7992,"ancestor_record_offset":7400,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":9564,"parent_record_offset":9532,"ancestor_record_offset":7400,"inlinee":6287,"source_line":1062,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":10436,"parent_record_offset":9564,"ancestor_record_offset":7400,"inlinee":6232,"source_line":2195,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8100,"parent_record_offset":8076,"ancestor_record_offset":7400,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":9652,"parent_record_offset":9612,"ancestor_record_offset":7400,"inlinee":6231,"source_line":1805,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":9836,"parent_record_offset":9652,"ancestor_record_offset":7400,"inlinee":5658,"source_line":1600,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":10032,"parent_record_offset":9652,"ancestor_record_offset":7400,"inlinee":5663,"source_line":1563,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":10068,"parent_record_offset":10032,"ancestor_record_offset":7400,"inlinee":5664,"source_line":2498,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":9612,"parent_record_offset":9564,"ancestor_record_offset":7400,"inlinee":6230,"source_line":1762,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8156,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":10336,"parent_record_offset":9612,"ancestor_record_offset":7400,"inlinee":5668,"source_line":2353,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":684,"record_offset":8516,"parent_record_offset":8100,"ancestor_record_offset":7400,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\rustc_entry.rs
//     evidence={"module":684,"record_offset":9532,"parent_record_offset":9500,"ancestor_record_offset":7400,"inlinee":7725,"source_line":512,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":7400,"inlinee":11655,"source_line":1332,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":684,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":7400,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":7820,"parent_record_offset":7604,"ancestor_record_offset":7400,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":8984,"parent_record_offset":8948,"ancestor_record_offset":7400,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":7640,"parent_record_offset":7604,"ancestor_record_offset":7400,"inlinee":5822,"source_line":508,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":9204,"parent_record_offset":9176,"ancestor_record_offset":7400,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":7604,"parent_record_offset":7576,"ancestor_record_offset":7400,"inlinee":5821,"source_line":107,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":10980,"parent_record_offset":10952,"ancestor_record_offset":7400,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":684,"record_offset":8948,"parent_record_offset":8920,"ancestor_record_offset":7400,"inlinee":5810,"source_line":129,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":684,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":7400,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":684,"record_offset":9012,"parent_record_offset":8984,"ancestor_record_offset":7400,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":9500,"parent_record_offset":9464,"ancestor_record_offset":7400,"inlinee":7723,"source_line":3192,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":684,"record_offset":9424,"parent_record_offset":9396,"ancestor_record_offset":7400,"inlinee":17354,"source_line":3285,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::create_vk_image_view
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":684,"record_offset":11204,"rva":12438432,"va":5381147552}
// 编译实例 RVAs: 0xbdcba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"backtrace","version":"0.3.71","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t684281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381147552,"raw_hex":"e8f9700a00","size":5,"soft_fail":false,"target_va":5381832720,"va":5381148434,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30394115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381147552,"raw_hex":"e803c7cf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381147784,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::format::vk_format_valid_aspect_flags
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t11912155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381147552,"raw_hex":"e8dbc3b500","size":5,"soft_fail":false,"target_va":5393059808,"va":5381147648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r13","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5381147552,"raw_hex":"41ffd5","size":3,"soft_fail":false,"va":5381148421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_image_view
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":684,"record_offset":11872,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39270,"source_line":2524,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":684,"record_offset":11404,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39259,"source_line":27,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":684,"record_offset":11376,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39257,"source_line":32,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$618::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":684,"record_offset":11488,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39262,"source_line":389,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ImageViewCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":684,"record_offset":11668,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39269,"source_line":2861,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":684,"record_offset":12052,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":684,"record_offset":11460,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":684,"record_offset":11992,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::texture::view_desc_impl
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11516,"parent_record_offset":11204,"ancestor_record_offset":11204,"inlinee":39264,"source_line":165,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":684,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":11204,"inlinee":39260,"source_line":23,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":684,"record_offset":11904,"parent_record_offset":11872,"ancestor_record_offset":11204,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":684,"record_offset":11928,"parent_record_offset":11904,"ancestor_record_offset":11204,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":684,"record_offset":11952,"parent_record_offset":11928,"ancestor_record_offset":11204,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":684,"record_offset":11816,"parent_record_offset":11792,"ancestor_record_offset":11204,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":684,"record_offset":11720,"parent_record_offset":11696,"ancestor_record_offset":11204,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":684,"record_offset":11744,"parent_record_offset":11720,"ancestor_record_offset":11204,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":684,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":11204,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":684,"record_offset":11696,"parent_record_offset":11668,"ancestor_record_offset":11204,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":684,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":11204,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":684,"record_offset":11568,"parent_record_offset":11516,"ancestor_record_offset":11204,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":684,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11204,"inlinee":39272,"source_line":21,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":11204,"inlinee":39267,"source_line":202,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::view_desc_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11600,"parent_record_offset":11568,"ancestor_record_offset":11204,"inlinee":39266,"source_line":181,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::view_desc_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9960
// 首条位置: {"module":684,"inlinee":39264}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":684,"record_offset":11568,"parent_record_offset":11516,"ancestor_record_offset":11516,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":11516,"inlinee":39267,"source_line":202,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::view_desc_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11600,"parent_record_offset":11568,"ancestor_record_offset":11516,"inlinee":39266,"source_line":181,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::view_desc_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9962
// 首条位置: {"module":684,"inlinee":39266}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":684,"record_offset":11624,"parent_record_offset":11600,"ancestor_record_offset":11600,"inlinee":39267,"source_line":202,"archive":"_pdb/B/modules/00684.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::convert_image_type_to_view_type
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9963
// 首条位置: {"module":684,"inlinee":39267}
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
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::managed::texture::ManagedTexture
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::impl$15::fmt<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::handle::impl$15::fmt", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::ManagedTexture::new
// 信息 ID: 648927532c3ff867077e1d99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x5b9f9c -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x5baa8b -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xc07dd8 -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xfaf03c -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x16a6b9c -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x16a768b -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x17faf1b -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::vec::into_iter::impl$5::fold<ash::vk::definitions::Image,alloc::alloc::Global,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<ash::vk::definitions::Image,rhapsody::raw::texture::Texture,tuple$<>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$20::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > > > @ 0x2135d1e -> 0xbdc2d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::ManagedTexture::view
// 信息 ID: 0ca6530af6e596cdc92e8c52
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::image_view @ 0xbe7ae6 -> 0xbdc5d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::get_vk_image_view @ 0xbfeee7 -> 0xbdc5d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::convert_image_type_to_view_type
// 信息 ID: e862c91a85bcbf0833dc26ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=684, inline_record=11624; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::create_vk_image_view
// 信息 ID: 374ba6c3fe5c4f0842a0f982
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::texture::ManagedTexture::new @ 0xbdc36a -> 0xbdcba0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::managed::texture::ManagedTexture::view @ 0xbdc800 -> 0xbdcba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::impl$0::debug_name
// 信息 ID: 22c208953df55a597bbbe79b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=1962, inline_record=6244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::impl$0::dispose
// 信息 ID: 8920f6d539af2a6dcc4f4887
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture> @ 0x18af0f9 -> 0xbdbe40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::view_desc_impl
// 信息 ID: da024641030c459a2d6b6240
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=684, inline_record=11516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::view_desc_impl::closure$0
// 信息 ID: 6064babd3474bdb7cdb09ab2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::texture::view_desc_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=684, inline_record=11600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
