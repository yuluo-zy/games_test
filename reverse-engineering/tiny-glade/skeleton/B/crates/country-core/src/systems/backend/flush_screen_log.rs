// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/systems/backend/flush_screen_log.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 3b2907a6ead8be0ec94d56244a3426a44e8d2c285161bfee4e0fe01a757136c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flush_screen_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 493
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::flush_error_log_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":78904,"rva":9178512,"va":5377887632}
// 编译实例 RVAs: 0x8c0d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8904779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8b51f78ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5377888326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3074066","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8ee17d1ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5377888381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140602960
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2877148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e82419d4ff","size":5,"soft_fail":false,"target_va":5375011168,"va":5377888311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2806426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8662dd5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377887925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33654693","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8a5870102","size":5,"soft_fail":false,"target_va":5411542880,"va":5377888182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::DebugGui::log_to_screen
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t520110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8aeef0700","size":5,"soft_fail":false,"target_va":5378408016,"va":5377887901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33650500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e844770102","size":5,"soft_fail":false,"target_va":5411538608,"va":5377888103,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33650676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377887632,"raw_hex":"e8f4770102","size":5,"soft_fail":false,"target_va":5411538768,"va":5377888087,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t33650743","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377887632,"raw_hex":"e937780102","size":5,"soft_fail":false,"target_va":5411538960,"va":5377888212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":79596,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":5295,"source_line":2606,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":493,"record_offset":79912,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":493,"record_offset":80180,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":493,"record_offset":79884,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":80236,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":493,"record_offset":79552,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":79088,"parent_record_offset":78904,"ancestor_record_offset":78904,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":80412,"parent_record_offset":80388,"ancestor_record_offset":78904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":80388,"parent_record_offset":80332,"ancestor_record_offset":78904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":79708,"parent_record_offset":79684,"ancestor_record_offset":78904,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":80360,"parent_record_offset":80332,"ancestor_record_offset":78904,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":80332,"parent_record_offset":80308,"ancestor_record_offset":78904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":79756,"parent_record_offset":79732,"ancestor_record_offset":78904,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":79732,"parent_record_offset":79708,"ancestor_record_offset":78904,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":80308,"parent_record_offset":80284,"ancestor_record_offset":78904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":80520,"parent_record_offset":80488,"ancestor_record_offset":78904,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":79684,"parent_record_offset":79596,"ancestor_record_offset":78904,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":79628,"parent_record_offset":79596,"ancestor_record_offset":78904,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":79656,"parent_record_offset":79596,"ancestor_record_offset":78904,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":493,"record_offset":80108,"parent_record_offset":80080,"ancestor_record_offset":78904,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":493,"record_offset":80488,"parent_record_offset":80464,"ancestor_record_offset":78904,"inlinee":5300,"source_line":175,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":493,"record_offset":80204,"parent_record_offset":80180,"ancestor_record_offset":78904,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":493,"record_offset":80080,"parent_record_offset":79912,"ancestor_record_offset":78904,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":80260,"parent_record_offset":80236,"ancestor_record_offset":78904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":493,"record_offset":79844,"parent_record_offset":79820,"ancestor_record_offset":78904,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":493,"record_offset":79988,"parent_record_offset":79944,"ancestor_record_offset":78904,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":80136,"parent_record_offset":80108,"ancestor_record_offset":78904,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":493,"record_offset":79796,"parent_record_offset":79596,"ancestor_record_offset":78904,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":493,"record_offset":79820,"parent_record_offset":79796,"ancestor_record_offset":78904,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":80044,"parent_record_offset":80020,"ancestor_record_offset":78904,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":79944,"parent_record_offset":79912,"ancestor_record_offset":78904,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":493,"record_offset":80020,"parent_record_offset":79944,"ancestor_record_offset":78904,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":79472,"parent_record_offset":79444,"ancestor_record_offset":78904,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81204,"parent_record_offset":81016,"ancestor_record_offset":78904,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":79160,"parent_record_offset":79128,"ancestor_record_offset":78904,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81296,"parent_record_offset":81264,"ancestor_record_offset":78904,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":79376,"parent_record_offset":79344,"ancestor_record_offset":78904,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":79184,"parent_record_offset":79160,"ancestor_record_offset":78904,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":79400,"parent_record_offset":79376,"ancestor_record_offset":78904,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81228,"parent_record_offset":81204,"ancestor_record_offset":78904,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81320,"parent_record_offset":81296,"ancestor_record_offset":78904,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":79344,"parent_record_offset":79316,"ancestor_record_offset":78904,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":79316,"parent_record_offset":79288,"ancestor_record_offset":78904,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":81016,"parent_record_offset":80980,"ancestor_record_offset":78904,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":79444,"parent_record_offset":79248,"ancestor_record_offset":78904,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":79248,"parent_record_offset":79220,"ancestor_record_offset":78904,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":79220,"parent_record_offset":79088,"ancestor_record_offset":78904,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":80980,"parent_record_offset":80952,"ancestor_record_offset":78904,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":79128,"parent_record_offset":79088,"ancestor_record_offset":78904,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":81264,"parent_record_offset":80980,"ancestor_record_offset":78904,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":493,"record_offset":79288,"parent_record_offset":79248,"ancestor_record_offset":78904,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::log_error_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":81596,"rva":9179296,"va":5377888416}
// 编译实例 RVAs: 0x8c10a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8905362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e86e1d78ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5377888909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3074700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e87415d1ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5377889015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2807053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e8f32ad5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377888536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2807469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e85329d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377888968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33958590","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e8be2a0602","size":5,"soft_fail":false,"target_va":5411847168,"va":5377888573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3532085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e8cb1acaff","size":5,"soft_fail":false,"target_va":5374356528,"va":5377888608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33652093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e87d7d0102","size":5,"soft_fail":false,"target_va":5411540963,"va":5377888865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33654079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e83f850102","size":5,"soft_fail":false,"target_va":5411542880,"va":5377888796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33649878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e8d6740102","size":5,"soft_fail":false,"target_va":5411538608,"va":5377888725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33650048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377888416,"raw_hex":"e880750102","size":5,"soft_fail":false,"target_va":5411538768,"va":5377888715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t33650135","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377888416,"raw_hex":"e9d7750102","size":5,"soft_fail":false,"target_va":5411538960,"va":5377888820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":493,"record_offset":82272,"parent_record_offset":81596,"ancestor_record_offset":81596,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":82924,"parent_record_offset":81596,"ancestor_record_offset":81596,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":83380,"parent_record_offset":81596,"ancestor_record_offset":81596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":493,"record_offset":82228,"parent_record_offset":81596,"ancestor_record_offset":81596,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":81768,"parent_record_offset":81596,"ancestor_record_offset":81596,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":82764,"parent_record_offset":82740,"ancestor_record_offset":81596,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":82788,"parent_record_offset":82764,"ancestor_record_offset":81596,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":83320,"parent_record_offset":83296,"ancestor_record_offset":81596,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":82740,"parent_record_offset":82564,"ancestor_record_offset":81596,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":83296,"parent_record_offset":83240,"ancestor_record_offset":81596,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83004,"parent_record_offset":82972,"ancestor_record_offset":81596,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":82504,"parent_record_offset":82476,"ancestor_record_offset":81596,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83268,"parent_record_offset":83240,"ancestor_record_offset":81596,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83240,"parent_record_offset":83216,"ancestor_record_offset":81596,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83052,"parent_record_offset":83028,"ancestor_record_offset":81596,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83028,"parent_record_offset":83004,"ancestor_record_offset":81596,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":82564,"parent_record_offset":82532,"ancestor_record_offset":81596,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":82532,"parent_record_offset":82504,"ancestor_record_offset":81596,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":83216,"parent_record_offset":83192,"ancestor_record_offset":81596,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":82600,"parent_record_offset":82564,"ancestor_record_offset":81596,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":82440,"parent_record_offset":82412,"ancestor_record_offset":81596,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":82412,"parent_record_offset":82384,"ancestor_record_offset":81596,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":82356,"parent_record_offset":82328,"ancestor_record_offset":81596,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":82384,"parent_record_offset":82356,"ancestor_record_offset":81596,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":82328,"parent_record_offset":82300,"ancestor_record_offset":81596,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":493,"record_offset":82300,"parent_record_offset":82272,"ancestor_record_offset":81596,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":82972,"parent_record_offset":82924,"ancestor_record_offset":81596,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":82476,"parent_record_offset":82440,"ancestor_record_offset":81596,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":82624,"parent_record_offset":82600,"ancestor_record_offset":81596,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":82648,"parent_record_offset":82624,"ancestor_record_offset":81596,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":493,"record_offset":82864,"parent_record_offset":82840,"ancestor_record_offset":81596,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":493,"record_offset":82696,"parent_record_offset":82672,"ancestor_record_offset":81596,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":82672,"parent_record_offset":82648,"ancestor_record_offset":81596,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":82840,"parent_record_offset":82440,"ancestor_record_offset":81596,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":83120,"parent_record_offset":82924,"ancestor_record_offset":81596,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":83092,"parent_record_offset":82924,"ancestor_record_offset":81596,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":82148,"parent_record_offset":82120,"ancestor_record_offset":81596,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":83632,"parent_record_offset":83444,"ancestor_record_offset":81596,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81840,"parent_record_offset":81808,"ancestor_record_offset":81596,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":83724,"parent_record_offset":83692,"ancestor_record_offset":81596,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":82052,"parent_record_offset":82020,"ancestor_record_offset":81596,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":81864,"parent_record_offset":81840,"ancestor_record_offset":81596,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":82076,"parent_record_offset":82052,"ancestor_record_offset":81596,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":83656,"parent_record_offset":83632,"ancestor_record_offset":81596,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":83748,"parent_record_offset":83724,"ancestor_record_offset":81596,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":82020,"parent_record_offset":81992,"ancestor_record_offset":81596,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":81992,"parent_record_offset":81964,"ancestor_record_offset":81596,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":83444,"parent_record_offset":83408,"ancestor_record_offset":81596,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":82120,"parent_record_offset":81928,"ancestor_record_offset":81596,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":81928,"parent_record_offset":81900,"ancestor_record_offset":81596,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":81900,"parent_record_offset":81768,"ancestor_record_offset":81596,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":83408,"parent_record_offset":83380,"ancestor_record_offset":81596,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":81808,"parent_record_offset":81768,"ancestor_record_offset":81596,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":83692,"parent_record_offset":83408,"ancestor_record_offset":81596,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":493,"record_offset":81964,"parent_record_offset":81928,"ancestor_record_offset":81596,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::backend::flush_screen_log::log_warning_to_screen
// 记录形式: compiled_procedure; instances=1
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":493,"record_offset":83796,"rva":9179920,"va":5377889040}
// 编译实例 RVAs: 0x8c1310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140043000
//     owner=unresolved_namespace; count=1
//     target_aliases=30 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8905986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e8fe1a78ff","size":5,"soft_fail":false,"target_va":5368983552,"va":5377889533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d2870
//     owner=unresolved_namespace; count=1
//     target_aliases=102 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3075324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e80413d1ff","size":5,"soft_fail":false,"target_va":5374814320,"va":5377889639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2807677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e88328d5ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5377889160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2808093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e8e326d5ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5377889592,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t33957966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e84e280602","size":5,"soft_fail":false,"target_va":5411847168,"va":5377889197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3532709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e85b18caff","size":5,"soft_fail":false,"target_va":5374356528,"va":5377889232,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33651469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e80d7b0102","size":5,"soft_fail":false,"target_va":5411540963,"va":5377889489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33653455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e8cf820102","size":5,"soft_fail":false,"target_va":5411542880,"va":5377889420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t33649254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e866720102","size":5,"soft_fail":false,"target_va":5411538608,"va":5377889349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t33649424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5377889040,"raw_hex":"e810730102","size":5,"soft_fail":false,"target_va":5411538768,"va":5377889339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t33649511","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5377889040,"raw_hex":"e967730102","size":5,"soft_fail":false,"target_va":5411538960,"va":5377889444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":493,"record_offset":84472,"parent_record_offset":83796,"ancestor_record_offset":83796,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":85124,"parent_record_offset":83796,"ancestor_record_offset":83796,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":85580,"parent_record_offset":83796,"ancestor_record_offset":83796,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":493,"record_offset":84428,"parent_record_offset":83796,"ancestor_record_offset":83796,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_direct] std::sync::poison::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":83968,"parent_record_offset":83796,"ancestor_record_offset":83796,"inlinee":6007,"source_line":435,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":84964,"parent_record_offset":84940,"ancestor_record_offset":83796,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":84988,"parent_record_offset":84964,"ancestor_record_offset":83796,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":85520,"parent_record_offset":85496,"ancestor_record_offset":83796,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":84940,"parent_record_offset":84764,"ancestor_record_offset":83796,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":493,"record_offset":85496,"parent_record_offset":85440,"ancestor_record_offset":83796,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85204,"parent_record_offset":85172,"ancestor_record_offset":83796,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":84704,"parent_record_offset":84676,"ancestor_record_offset":83796,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85468,"parent_record_offset":85440,"ancestor_record_offset":83796,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85440,"parent_record_offset":85416,"ancestor_record_offset":83796,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85252,"parent_record_offset":85228,"ancestor_record_offset":83796,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85228,"parent_record_offset":85204,"ancestor_record_offset":83796,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":84764,"parent_record_offset":84732,"ancestor_record_offset":83796,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":84732,"parent_record_offset":84704,"ancestor_record_offset":83796,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":85416,"parent_record_offset":85392,"ancestor_record_offset":83796,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":493,"record_offset":84800,"parent_record_offset":84764,"ancestor_record_offset":83796,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":84640,"parent_record_offset":84612,"ancestor_record_offset":83796,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":84612,"parent_record_offset":84584,"ancestor_record_offset":83796,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":84556,"parent_record_offset":84528,"ancestor_record_offset":83796,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":84584,"parent_record_offset":84556,"ancestor_record_offset":83796,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":493,"record_offset":84528,"parent_record_offset":84500,"ancestor_record_offset":83796,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":493,"record_offset":84500,"parent_record_offset":84472,"ancestor_record_offset":83796,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":85172,"parent_record_offset":85124,"ancestor_record_offset":83796,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":493,"record_offset":84676,"parent_record_offset":84640,"ancestor_record_offset":83796,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":84824,"parent_record_offset":84800,"ancestor_record_offset":83796,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":493,"record_offset":84848,"parent_record_offset":84824,"ancestor_record_offset":83796,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":493,"record_offset":85064,"parent_record_offset":85040,"ancestor_record_offset":83796,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":493,"record_offset":84896,"parent_record_offset":84872,"ancestor_record_offset":83796,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":493,"record_offset":84872,"parent_record_offset":84848,"ancestor_record_offset":83796,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":493,"record_offset":85040,"parent_record_offset":84640,"ancestor_record_offset":83796,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":85320,"parent_record_offset":85124,"ancestor_record_offset":83796,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":493,"record_offset":85292,"parent_record_offset":85124,"ancestor_record_offset":83796,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":84348,"parent_record_offset":84320,"ancestor_record_offset":83796,"inlinee":5623,"source_line":623,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":85832,"parent_record_offset":85644,"ancestor_record_offset":83796,"inlinee":5912,"source_line":651,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":84040,"parent_record_offset":84008,"ancestor_record_offset":83796,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":85924,"parent_record_offset":85892,"ancestor_record_offset":83796,"inlinee":5910,"source_line":2753,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":84252,"parent_record_offset":84220,"ancestor_record_offset":83796,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":84064,"parent_record_offset":84040,"ancestor_record_offset":83796,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":84276,"parent_record_offset":84252,"ancestor_record_offset":83796,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":85856,"parent_record_offset":85832,"ancestor_record_offset":83796,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":493,"record_offset":85948,"parent_record_offset":85924,"ancestor_record_offset":83796,"inlinee":5911,"source_line":3743,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":84220,"parent_record_offset":84192,"ancestor_record_offset":83796,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":493,"record_offset":84192,"parent_record_offset":84164,"ancestor_record_offset":83796,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":85644,"parent_record_offset":85608,"ancestor_record_offset":83796,"inlinee":5901,"source_line":132,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":84320,"parent_record_offset":84128,"ancestor_record_offset":83796,"inlinee":5621,"source_line":144,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison.rs
//     evidence={"module":493,"record_offset":84128,"parent_record_offset":84100,"ancestor_record_offset":83796,"inlinee":6011,"source_line":122,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":84100,"parent_record_offset":83968,"ancestor_record_offset":83796,"inlinee":6010,"source_line":647,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sync\poison\mutex.rs
//     evidence={"module":493,"record_offset":85608,"parent_record_offset":85580,"ancestor_record_offset":83796,"inlinee":5900,"source_line":671,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":84008,"parent_record_offset":83968,"ancestor_record_offset":83796,"inlinee":6008,"source_line":27,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":493,"record_offset":85892,"parent_record_offset":85608,"ancestor_record_offset":83796,"inlinee":5909,"source_line":83,"archive":"_pdb/B/modules/00493.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":493,"record_offset":84164,"parent_record_offset":84128,"ancestor_record_offset":83796,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00493.jsonl.gz"}
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
// 完整结构及原始类型记录: flush_screen_log.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::backend::flush_screen_log::LogType
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "constructor": "tuple$", "arguments": ["alloc::string::String", "country_core::systems::backend::flush_screen_log::LogType"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: flush_screen_log.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: flush_screen_log.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::flush_error_log_to_screen
// 信息 ID: d0a269f5ca9944d11aaf7980
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86d2c9c7ede139c54c7d002b: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > > @ 0x44f54e -> 0x8c0d90 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > @ 0x5b66d0 -> 0x8c0d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_error_to_screen
// 信息 ID: f3b7d4e71928bb2371a1494a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x77795c -> 0x8c10a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0xa7ece2 -> 0x8c10a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x146e3cc -> 0x8c10a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_warning_to_screen
// 信息 ID: abbb127998b52acd18a393f4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0xaa0366 -> 0x8c1310 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
