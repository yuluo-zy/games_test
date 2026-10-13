// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/swapchain.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c846134606a3f648abc265f37eccaed5f4afc574d4efdfd1eddbd2429a112409
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: swapchain.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 61, 690, 703, 710, 1987, 2311, 3316
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use append_only_vec; // namespace candidate
//   package identities: [{"name":"append-only-vec","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8","version":"0.1.2"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$8::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=9
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25B4
// 首条位置: {"module":61,"inlinee":9652}
// 编译实例 RVAs: 0x1575f0, 0x18cd0f0, 0x1a496c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t12777449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5370115568,"raw_hex":"e8e9f7c200","size":5,"soft_fail":false,"target_va":5382893232,"va":5370115778,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$9::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x25B6
// 首条位置: {"module":61,"inlinee":9654}
// 编译实例 RVAs: 0x18cd1d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11822695","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715088,"raw_hex":"e899994bff","size":5,"soft_fail":false,"target_va":5382892528,"va":5394715218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11822699","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715088,"raw_hex":"e895994bff","size":5,"soft_fail":false,"target_va":5382892704,"va":5394715398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-11820654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394715088,"raw_hex":"e892a14bff","size":5,"soft_fail":false,"target_va":5382894640,"va":5394715289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140d86b90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11822726","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5394715088,"raw_hex":"e97a994bff","size":5,"soft_fail":false,"target_va":5382892432,"va":5394715153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::enumerate_surface_formats
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":648,"rva":12641872,"va":5381350992}
// 编译实例 RVAs: 0xc0e650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] ash::prelude::read_into_uninitialized_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t13245931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381350992,"raw_hex":"e8eb1dca00","size":5,"soft_fail":false,"target_va":5394597008,"va":5381351072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":710,"record_offset":800,"parent_record_offset":648,"ancestor_record_offset":648,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::khr::surface::Instance::get_physical_device_surface_formats
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/khr/surface.rs
//     evidence={"module":710,"record_offset":828,"parent_record_offset":648,"ancestor_record_offset":648,"inlinee":40050,"source_line":64,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::on_focus
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":860,"rva":12641968,"va":5381351088}
// 编译实例 RVAs: 0xc0e6b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30190391","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351088,"raw_hex":"e837abcc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381351380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30130334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351088,"raw_hex":"e89ec0cb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381351283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30133819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351088,"raw_hex":"e83bcecb01","size":5,"soft_fail":false,"target_va":5411485168,"va":5381351344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t472422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351088,"raw_hex":"e866350700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381351173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t472808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351088,"raw_hex":"e8e8360700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381351203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t30136211","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381351088,"raw_hex":"e993d7cb01","size":5,"soft_fail":false,"target_va":5411487584,"va":5381351368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":1124,"parent_record_offset":860,"ancestor_record_offset":860,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":1000,"parent_record_offset":860,"ancestor_record_offset":860,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::ManagedSwapchain::on_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":1160,"parent_record_offset":860,"ancestor_record_offset":860,"inlinee":40052,"source_line":64,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":1432,"parent_record_offset":1160,"ancestor_record_offset":860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1604,"parent_record_offset":1484,"ancestor_record_offset":860,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1256,"parent_record_offset":1216,"ancestor_record_offset":860,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":860,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":860,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":1188,"parent_record_offset":1160,"ancestor_record_offset":860,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":860,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1312,"parent_record_offset":1216,"ancestor_record_offset":860,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1520,"parent_record_offset":1484,"ancestor_record_offset":860,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":860,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":1484,"parent_record_offset":1456,"ancestor_record_offset":860,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":860,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":860,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":1040,"parent_record_offset":1000,"ancestor_record_offset":860,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":1092,"parent_record_offset":1000,"ancestor_record_offset":860,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":1068,"parent_record_offset":1000,"ancestor_record_offset":860,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::on_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":1404,"parent_record_offset":1160,"ancestor_record_offset":860,"inlinee":40054,"source_line":391,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::format
// 记录形式: compiled_procedure; instances=1
// 源码行: 79
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":1680,"rva":12642272,"va":5381351392}
// 编译实例 RVAs: 0xc0e7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::resize
// 记录形式: compiled_procedure; instances=1
// 源码行: 82
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":1800,"rva":12642288,"va":5381351408}
// 编译实例 RVAs: 0xc0e7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30190197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351408,"raw_hex":"e875aacc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381351574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30130102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351408,"raw_hex":"e8b6bfcb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381351515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] rhapsody::raw::managed::swapchain::ManagedSwapchain::resize
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t13355393","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381351408,"raw_hex":"e981c9cb00","size":5,"soft_fail":false,"target_va":5394706864,"va":5381351466,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2052,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":1932,"parent_record_offset":1800,"ancestor_record_offset":1800,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":1968,"parent_record_offset":1932,"ancestor_record_offset":1800,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2020,"parent_record_offset":1932,"ancestor_record_offset":1800,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":1996,"parent_record_offset":1932,"ancestor_record_offset":1800,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::set_vsync
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":2088,"rva":12642464,"va":5381351584}
// 编译实例 RVAs: 0xc0e8a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30190021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351584,"raw_hex":"e8c5a9cc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381351750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30129926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351584,"raw_hex":"e806bfcb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381351691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] rhapsody::raw::managed::swapchain::ManagedSwapchain::set_vsync
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t13355665","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381351584,"raw_hex":"e991cacb00","size":5,"soft_fail":false,"target_va":5394707312,"va":5381351642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2344,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":2224,"parent_record_offset":2088,"ancestor_record_offset":2088,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2260,"parent_record_offset":2224,"ancestor_record_offset":2088,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2312,"parent_record_offset":2224,"ancestor_record_offset":2088,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2288,"parent_record_offset":2224,"ancestor_record_offset":2088,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::mark_stale
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":2380,"rva":12642640,"va":5381351760}
// 编译实例 RVAs: 0xc0e950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30189700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351760,"raw_hex":"e884a8cc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381352071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30129641","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351760,"raw_hex":"e8e9bdcb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381351976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30133126","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351760,"raw_hex":"e886cbcb01","size":5,"soft_fail":false,"target_va":5411485168,"va":5381352037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t471752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351760,"raw_hex":"e8c8320700","size":5,"soft_fail":false,"target_va":5381823600,"va":5381351843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t472112","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381351760,"raw_hex":"e830340700","size":5,"soft_fail":false,"target_va":5381824016,"va":5381351899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t30135520","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381351760,"raw_hex":"e9e0d4cb01","size":5,"soft_fail":false,"target_va":5411487584,"va":5381352059,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2644,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":2520,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::swapchain::ManagedSwapchain::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":2680,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":40055,"source_line":80,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2380,"inlinee":15140,"source_line":1642,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":2976,"parent_record_offset":2948,"ancestor_record_offset":2380,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":3016,"parent_record_offset":2680,"ancestor_record_offset":2380,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":3196,"parent_record_offset":3072,"ancestor_record_offset":2380,"inlinee":5818,"source_line":2871,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":2776,"parent_record_offset":2736,"ancestor_record_offset":2380,"inlinee":5824,"source_line":2920,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2380,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2380,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2380,"inlinee":5840,"source_line":479,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":710,"record_offset":3044,"parent_record_offset":3016,"ancestor_record_offset":2380,"inlinee":11657,"source_line":1667,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_acquire
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":2832,"parent_record_offset":2736,"ancestor_record_offset":2380,"inlinee":5826,"source_line":1139,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":3108,"parent_record_offset":3072,"ancestor_record_offset":2380,"inlinee":5812,"source_line":1145,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2380,"inlinee":5841,"source_line":67,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":710,"record_offset":3072,"parent_record_offset":3044,"ancestor_record_offset":2380,"inlinee":5817,"source_line":94,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":2380,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":710,"record_offset":3136,"parent_record_offset":3108,"ancestor_record_offset":2380,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2560,"parent_record_offset":2520,"ancestor_record_offset":2380,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2612,"parent_record_offset":2520,"ancestor_record_offset":2380,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":2588,"parent_record_offset":2520,"ancestor_record_offset":2380,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::swapchain::SwapchainInner::mark_stale
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/swapchain.rs
//     evidence={"module":710,"record_offset":2924,"parent_record_offset":2680,"ancestor_record_offset":2380,"inlinee":40056,"source_line":446,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::extent
// 记录形式: compiled_procedure; instances=1
// 源码行: 102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":3272,"rva":12642960,"va":5381352080}
// 编译实例 RVAs: 0xc0ea90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30189523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352080,"raw_hex":"e8d3a7cc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381352248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30129428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352080,"raw_hex":"e814bdcb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381352189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::desc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13354551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352080,"raw_hex":"e837c6cb00","size":5,"soft_fail":false,"target_va":5394706688,"va":5381352132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":3520,"parent_record_offset":3272,"ancestor_record_offset":3272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":3404,"parent_record_offset":3272,"ancestor_record_offset":3272,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":3440,"parent_record_offset":3404,"ancestor_record_offset":3272,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":3488,"parent_record_offset":3404,"ancestor_record_offset":3272,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":3464,"parent_record_offset":3404,"ancestor_record_offset":3272,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::acquire_next_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":3556,"rva":12643136,"va":5381352256}
// 编译实例 RVAs: 0xc0eb40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30188498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8d2a3cc01","size":5,"soft_fail":false,"target_va":5411541584,"va":5381353081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30188631","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e857a4cc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381353140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30128550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8a6b9cb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381353067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30190825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8e9accc01","size":5,"soft_fail":false,"target_va":5411543360,"va":5381352530,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e878800000","size":5,"soft_fail":false,"target_va":5381385536,"va":5381352643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t38081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8c1940000","size":5,"soft_fail":false,"target_va":5381390640,"va":5381352554,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t33539","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e803830000","size":5,"soft_fail":false,"target_va":5381386480,"va":5381352936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t32401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8917e0000","size":5,"soft_fail":false,"target_va":5381384704,"va":5381352298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::acquire_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13355002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8fac7cb00","size":5,"soft_fail":false,"target_va":5394707744,"va":5381352737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30384827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8bba2cf01","size":5,"soft_fail":false,"target_va":5411737808,"va":5381352976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t13319888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e8d03ecb00","size":5,"soft_fail":false,"target_va":5394673152,"va":5381353259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30184077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381352256,"raw_hex":"e88d92cc01","size":5,"soft_fail":false,"target_va":5411537104,"va":5381353022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":5808,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":6036,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":710,"record_offset":5844,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":4692,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":3716,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":5676,"parent_record_offset":3556,"ancestor_record_offset":3556,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3556,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":3556,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":3556,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":4976,"parent_record_offset":4948,"ancestor_record_offset":3556,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5000,"parent_record_offset":4976,"ancestor_record_offset":3556,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5172,"parent_record_offset":5000,"ancestor_record_offset":3556,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":710,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":3556,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":710,"record_offset":5104,"parent_record_offset":5080,"ancestor_record_offset":3556,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":710,"record_offset":4556,"parent_record_offset":4520,"ancestor_record_offset":3556,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":710,"record_offset":4844,"parent_record_offset":4800,"ancestor_record_offset":3556,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":710,"record_offset":4640,"parent_record_offset":4612,"ancestor_record_offset":3556,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":710,"record_offset":5632,"parent_record_offset":4800,"ancestor_record_offset":3556,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":5236,"parent_record_offset":4948,"ancestor_record_offset":3556,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":710,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":3556,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":710,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":3556,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":710,"record_offset":4308,"parent_record_offset":4280,"ancestor_record_offset":3556,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":5128,"parent_record_offset":5104,"ancestor_record_offset":3556,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":710,"record_offset":4052,"parent_record_offset":4028,"ancestor_record_offset":3556,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":710,"record_offset":4076,"parent_record_offset":4052,"ancestor_record_offset":3556,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":710,"record_offset":4380,"parent_record_offset":4352,"ancestor_record_offset":3556,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":710,"record_offset":4028,"parent_record_offset":3916,"ancestor_record_offset":3556,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":710,"record_offset":4520,"parent_record_offset":4488,"ancestor_record_offset":3556,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":710,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":3556,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":710,"record_offset":3884,"parent_record_offset":3852,"ancestor_record_offset":3556,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":710,"record_offset":3852,"parent_record_offset":3816,"ancestor_record_offset":3556,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":710,"record_offset":3792,"parent_record_offset":3768,"ancestor_record_offset":3556,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":710,"record_offset":3816,"parent_record_offset":3792,"ancestor_record_offset":3556,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":710,"record_offset":3768,"parent_record_offset":3744,"ancestor_record_offset":3556,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":710,"record_offset":4352,"parent_record_offset":3816,"ancestor_record_offset":3556,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":710,"record_offset":4488,"parent_record_offset":4456,"ancestor_record_offset":3556,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":710,"record_offset":4588,"parent_record_offset":4488,"ancestor_record_offset":3556,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":710,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":3556,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":710,"record_offset":4432,"parent_record_offset":3716,"ancestor_record_offset":3556,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":710,"record_offset":4612,"parent_record_offset":4588,"ancestor_record_offset":3556,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":710,"record_offset":3916,"parent_record_offset":3884,"ancestor_record_offset":3556,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":710,"record_offset":4196,"parent_record_offset":3916,"ancestor_record_offset":3556,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":710,"record_offset":4168,"parent_record_offset":3916,"ancestor_record_offset":3556,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":710,"record_offset":3972,"parent_record_offset":3916,"ancestor_record_offset":3556,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":3556,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":3556,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":4728,"parent_record_offset":4692,"ancestor_record_offset":3556,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":6104,"parent_record_offset":6072,"ancestor_record_offset":3556,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":6604,"parent_record_offset":6344,"ancestor_record_offset":3556,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":710,"record_offset":4872,"parent_record_offset":4844,"ancestor_record_offset":3556,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":4948,"parent_record_offset":4800,"ancestor_record_offset":3556,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":710,"record_offset":5208,"parent_record_offset":4948,"ancestor_record_offset":3556,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":3556,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5720,"parent_record_offset":5676,"ancestor_record_offset":3556,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5776,"parent_record_offset":5676,"ancestor_record_offset":3556,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5748,"parent_record_offset":5676,"ancestor_record_offset":3556,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":3556,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::swapchain::impl$0::acquire_next_image::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/swapchain.rs
//     evidence={"module":710,"record_offset":5876,"parent_record_offset":5844,"ancestor_record_offset":3556,"inlinee":40058,"source_line":117,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":710,"record_offset":4900,"parent_record_offset":4872,"ancestor_record_offset":3556,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":710,"record_offset":4800,"parent_record_offset":4764,"ancestor_record_offset":3556,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":710,"record_offset":4764,"parent_record_offset":4728,"ancestor_record_offset":3556,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$0::acquire_next_image::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C7A
// 首条位置: {"module":710,"inlinee":40058}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":5964,"parent_record_offset":5928,"ancestor_record_offset":5876,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":5876,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":5928,"parent_record_offset":5904,"ancestor_record_offset":5876,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::any_present_successful
// 记录形式: compiled_procedure; instances=1
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":7196,"rva":12644208,"va":5381353328}
// 编译实例 RVAs: 0xc0ef70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::Swapchain::present
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":7332,"rva":12644224,"va":5381353344}
// 编译实例 RVAs: 0xc0ef80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30188252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353344,"raw_hex":"e8dca2cc01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381353519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30128157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353344,"raw_hex":"e81db8cb01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381353460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::present
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13357553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353344,"raw_hex":"e8f1d1cb00","size":5,"soft_fail":false,"target_va":5394710960,"va":5381353402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":710,"record_offset":7584,"parent_record_offset":7332,"ancestor_record_offset":7332,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":710,"record_offset":7464,"parent_record_offset":7332,"ancestor_record_offset":7332,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":7500,"parent_record_offset":7464,"ancestor_record_offset":7332,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":7552,"parent_record_offset":7464,"ancestor_record_offset":7332,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":7528,"parent_record_offset":7464,"ancestor_record_offset":7332,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::SwapchainLeaseSync::into_sync_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":7620,"rva":12644416,"va":5381353536}
// 编译实例 RVAs: 0xc0f040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be0060
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-192664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353536,"raw_hex":"e8680ffdff","size":5,"soft_fail":false,"target_va":5381161056,"va":5381353715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-183298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353536,"raw_hex":"e8fe33fdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381353645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30188380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353536,"raw_hex":"e85ca3cc01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381353679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30184917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353536,"raw_hex":"e8d595cc01","size":5,"soft_fail":false,"target_va":5411538608,"va":5381353686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":7892,"parent_record_offset":7620,"ancestor_record_offset":7620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":7768,"parent_record_offset":7620,"ancestor_record_offset":7620,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":8104,"parent_record_offset":7892,"ancestor_record_offset":7620,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":7828,"parent_record_offset":7792,"ancestor_record_offset":7620,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":8032,"parent_record_offset":8004,"ancestor_record_offset":7620,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7620,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":7620,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":7620,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::swapchain::impl$2::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/swapchain.rs
//     evidence={"module":710,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":7620,"inlinee":39303,"source_line":161,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":710,"record_offset":8004,"parent_record_offset":7976,"ancestor_record_offset":7620,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":710,"record_offset":7976,"parent_record_offset":7948,"ancestor_record_offset":7620,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":710,"record_offset":7948,"parent_record_offset":7920,"ancestor_record_offset":7620,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$2::drop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9987
// 首条位置: {"module":690,"inlinee":39303}
// 编译实例 RVAs: 0xbe2610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30371093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170704,"raw_hex":"e8156dcf01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381170806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30367861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381170704,"raw_hex":"e87560cf01","size":5,"soft_fail":false,"target_va":5411538608,"va":5381170742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":690,"record_offset":64528,"parent_record_offset":64356,"ancestor_record_offset":64356,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_direct] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\mod.rs
//     evidence={"module":690,"record_offset":64388,"parent_record_offset":64356,"ancestor_record_offset":64356,"inlinee":5903,"source_line":814,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":64464,"parent_record_offset":64436,"ancestor_record_offset":64356,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":690,"record_offset":64484,"parent_record_offset":64464,"ancestor_record_offset":64356,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":690,"record_offset":64436,"parent_record_offset":64412,"ancestor_record_offset":64356,"inlinee":5907,"source_line":469,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\panicking.rs
//     evidence={"module":690,"record_offset":64412,"parent_record_offset":64388,"ancestor_record_offset":64356,"inlinee":5905,"source_line":621,"archive":"_pdb/B/modules/00690.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::TrackedSwapchainTextureLease::image
// 记录形式: compiled_procedure; instances=1
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":8216,"rva":12644640,"va":5381353760}
// 编译实例 RVAs: 0xc0f120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":8364,"parent_record_offset":8216,"ancestor_record_offset":8216,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":8428,"parent_record_offset":8388,"ancestor_record_offset":8216,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":710,"record_offset":8448,"parent_record_offset":8428,"ancestor_record_offset":8216,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":710,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":8216,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::TrackedSwapchainTextureLease::into_present_source
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":710,"record_offset":8492,"rva":12644720,"va":5381353840}
// 编译实例 RVAs: 0xc0f170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-183528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381353840,"raw_hex":"e81833fdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381353875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":8652,"parent_record_offset":8492,"ancestor_record_offset":8492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":710,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":8492,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00710.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_swapchain
// 记录形式: compiled_procedure; instances=1
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":133516,"rva":12572448,"va":5381281568}
// 编译实例 RVAs: 0xbfd720
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30565348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e8e463d201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381281815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30565266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e89263d201","size":5,"soft_fail":false,"target_va":5411847184,"va":5381281913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::AppendOnlyVec$LT$T$GT$::push
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     evidence={"assembly":"\tcallq\t13362283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e86be4cb00","size":5,"soft_fail":false,"target_va":5394644272,"va":5381281984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] append_only_vec::indices
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     evidence={"assembly":"\tcallq\t80083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e8d3380100","size":5,"soft_fail":false,"target_va":5381362096,"va":5381282008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30259541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e855b9cd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381282230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30199473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e8b1cecc01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381282144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30259841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e881bacd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381282218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30260044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e84cbbcd01","size":5,"soft_fail":false,"target_va":5411542296,"va":5381282247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13420665","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e879c8cc00","size":5,"soft_fail":false,"target_va":5394702384,"va":5381281714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::swapchain::ManagedSwapchain::retire
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13431763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381281568,"raw_hex":"e8d3f3cc00","size":5,"soft_fail":false,"target_va":5394713424,"va":5381281656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":133668,"parent_record_offset":133516,"ancestor_record_offset":133516,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":133908,"parent_record_offset":133516,"ancestor_record_offset":133516,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":134472,"parent_record_offset":133516,"ancestor_record_offset":133516,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":133944,"parent_record_offset":133516,"ancestor_record_offset":133516,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::AppendOnlyVec::len
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":703,"record_offset":134344,"parent_record_offset":134296,"ancestor_record_offset":133516,"inlinee":16141,"source_line":104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] append_only_vec::impl$4::index
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"append-only-vec","version":"0.1.2","source":"git+https://github.com/h3r2tic/append-only-vec.git?rev=4031d674025f4765ad8fdd7fd41ef3bc7bea65e8#4031d674025f4765ad8fdd7fd41ef3bc7bea65e8"}]
//     definition_source=/home/h3/.cargo/git/checkouts/append-only-vec-05a13811c738e3a6/4031d67/src/lib.rs
//     evidence={"module":703,"record_offset":134296,"parent_record_offset":134024,"ancestor_record_offset":133516,"inlinee":39243,"source_line":225,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":133864,"parent_record_offset":133704,"ancestor_record_offset":133516,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":134172,"parent_record_offset":134148,"ancestor_record_offset":133516,"inlinee":27245,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":134200,"parent_record_offset":134172,"ancestor_record_offset":133516,"inlinee":27247,"source_line":3874,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":134428,"parent_record_offset":134296,"ancestor_record_offset":133516,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":134148,"parent_record_offset":134024,"ancestor_record_offset":133516,"inlinee":32600,"source_line":38,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134240,"parent_record_offset":134024,"ancestor_record_offset":133516,"inlinee":39241,"source_line":1779,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134092,"parent_record_offset":134024,"ancestor_record_offset":133516,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134368,"parent_record_offset":134344,"ancestor_record_offset":133516,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134264,"parent_record_offset":134240,"ancestor_record_offset":133516,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134116,"parent_record_offset":134092,"ancestor_record_offset":133516,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133780,"parent_record_offset":133736,"ancestor_record_offset":133516,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133832,"parent_record_offset":133736,"ancestor_record_offset":133516,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133808,"parent_record_offset":133736,"ancestor_record_offset":133516,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceStore::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":134024,"parent_record_offset":133988,"ancestor_record_offset":133516,"inlinee":39240,"source_line":71,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":133736,"parent_record_offset":133704,"ancestor_record_offset":133516,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$8::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":133988,"parent_record_offset":133944,"ancestor_record_offset":133516,"inlinee":39767,"source_line":389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::swapchain::impl$4::create_swapchain::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/swapchain.rs
//     evidence={"module":703,"record_offset":133704,"parent_record_offset":133668,"ancestor_record_offset":133516,"inlinee":39765,"source_line":207,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::swapchain::impl$4::create_swapchain::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B55
// 首条位置: {"module":703,"inlinee":39765}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":133864,"parent_record_offset":133704,"ancestor_record_offset":133704,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":133736,"parent_record_offset":133704,"ancestor_record_offset":133704,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133780,"parent_record_offset":133736,"ancestor_record_offset":133704,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133832,"parent_record_offset":133736,"ancestor_record_offset":133704,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":133808,"parent_record_offset":133736,"ancestor_record_offset":133704,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::validate_swapchain_and_acquire_image
// 记录形式: compiled_procedure; instances=1
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":134504,"rva":12573136,"va":5381282256}
// 编译实例 RVAs: 0xbfd9d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t1361945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e819c81400","size":5,"soft_fail":false,"target_va":5382644528,"va":5381282578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6201427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e8ad5fa1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381282926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5766426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e8e602a8ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381282453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-112677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e8db47feff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381283024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140befd60
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-57868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e8f41dffff","size":5,"soft_fail":false,"target_va":5381225824,"va":5381283687,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfd720
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: swapchain.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e861faffff","size":5,"soft_fail":false,"target_va":5381281568,"va":5381283002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t30258243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e843b4cd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381283656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::acquire_next_image
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t69938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381282256,"raw_hex":"e832110100","size":5,"soft_fail":false,"target_va":5381352256,"va":5381282313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":134768,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":136176,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":5501,"source_line":600,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":136148,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":135268,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":134800,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":134684,"parent_record_offset":134504,"ancestor_record_offset":134504,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":135464,"parent_record_offset":135440,"ancestor_record_offset":134504,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":135440,"parent_record_offset":135380,"ancestor_record_offset":134504,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":135412,"parent_record_offset":135380,"ancestor_record_offset":134504,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":135380,"parent_record_offset":135352,"ancestor_record_offset":134504,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":135352,"parent_record_offset":135324,"ancestor_record_offset":134504,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":134856,"parent_record_offset":134824,"ancestor_record_offset":134504,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":703,"record_offset":134884,"parent_record_offset":134824,"ancestor_record_offset":134504,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":135296,"parent_record_offset":135268,"ancestor_record_offset":134504,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134708,"parent_record_offset":134684,"ancestor_record_offset":134504,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":134732,"parent_record_offset":134708,"ancestor_record_offset":134504,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":134912,"parent_record_offset":134824,"ancestor_record_offset":134504,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":703,"record_offset":134824,"parent_record_offset":134800,"ancestor_record_offset":134504,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":703,"record_offset":134936,"parent_record_offset":134912,"ancestor_record_offset":134504,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: swapchain.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::Swapchain
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 12; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::swapchain::SwapchainError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::swapchain::SwapchainError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::Swapchain", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<rhapsody::raw::swapchain::SwapchainDesc>", "constructor": "core::fmt::impl$74::fmt", "arguments": ["rhapsody::raw::swapchain::SwapchainDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::swapchain::SwapchainError>", "constructor": "enum2$", "arguments": ["rhapsody::raw::swapchain::SwapchainError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::swapchain::Swapchain", "enum2$<rhapsody::raw::swapchain::SwapchainError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::swapchain::Swapchain,enum2$<rhapsody::raw::swapchain::SwapchainError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainLeaseSync
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::swapchain::SwapchainLeaseSync>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::swapchain::SwapchainLeaseSync"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::SwapchainTextureLease
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::swapchain::SwapchainTextureLease>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::swapchain::SwapchainTextureLease"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::swapchain::TrackedSwapchainTextureLease
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::swapchain::TrackedSwapchainTextureLease>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::swapchain::TrackedSwapchainTextureLease"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: swapchain.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "rhapsody::raw::swapchain::TrackedSwapchainTextureLease", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: swapchain.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_swapchain
// 信息 ID: cc3c836f571ec1ca1ce82b1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x596bea -> 0xbfd720 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x596e8f -> 0xbfd720 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x5973f5 -> 0xbfd720 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::validate_swapchain_and_acquire_image
// 信息 ID: 0803b07f22e1bfe538a498d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9bf8 -> 0xbfd9d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::acquire_next_image
// 信息 ID: 648d24262882d931d8ff7b6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::validate_swapchain_and_acquire_image @ 0xbfda09 -> 0xc0eb40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::any_present_successful
// 信息 ID: bd994b0a26231394b94a405e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4de860 -> 0xc0ef70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::enumerate_surface_formats
// 信息 ID: 1a209392864c10373fa1936e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x596936 -> 0xc0e650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::extent
// 信息 ID: 67fc88f83aa48a3cef6b3254
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4dcab4 -> 0xc0ea90 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e8813 -> 0xc0ea90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::format
// 信息 ID: 022704fb521bead962494495
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e87f7 -> 0xc0e7e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::mark_stale
// 信息 ID: ca116ca86dc2947c75c49edc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::on_focus
// 信息 ID: 6a03fc09e74b334aa71a9a41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::present
// 信息 ID: 7b3eac4ce23cdc20ca9f5e56
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9f74 -> 0xc0ef80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::resize
// 信息 ID: 0a030c78fb2e8f3a3fc634d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4dcae1 -> 0xc0e7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::Swapchain::set_vsync
// 信息 ID: 43fd35740ad6a2484d6d3a60
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4dea4d -> 0xc0e8a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::SwapchainLeaseSync::into_sync_data
// 信息 ID: a6b2bcbfcf2da1ecadb3d62b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute_compiled<country_core::render::pass_builder::pass_resources::PassResources> @ 0xfa76a -> 0xc0f040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::TrackedSwapchainTextureLease::image
// 信息 ID: 7c3660743dcfb1c89abdaae3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9c9a -> 0xc0f120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::TrackedSwapchainTextureLease::into_present_source
// 信息 ID: 8e0c23f7cda05629acddc76e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::render @ 0x4e9ecf -> 0xc0f170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$0::acquire_next_image::closure$0
// 信息 ID: f8bba302c2c977847d77ecba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::swapchain::impl$0::acquire_next_image", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::acquire_next_image; module=710, inline_record=5876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$2::drop
// 信息 ID: d8fb1a3a069d1a2ab5655819
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutionRecorder::execute<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0xfb627 -> 0xbe2610 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::swapchain::SwapchainLeaseSync> @ 0x5f9796 -> 0xbe2610 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::raw::swapchain::SwapchainLeaseSync>; module=690, inline_record=64356; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::swapchain::SwapchainLeaseSync::into_sync_data; module=710, inline_record=7920; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$4::create_swapchain::closure$0
// 信息 ID: cbe053ac4f9325eef9770dc2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::swapchain::impl$4::create_swapchain", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_swapchain; module=703, inline_record=133704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$8::fmt
// 信息 ID: 5a89e0364fe4ad2c8d281bbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=61, inline_record=13196; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=2311, inline_record=756; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<rhapsody::raw::swapchain::SwapchainDesc>; module=3316, inline_record=504; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::swapchain::impl$9::fmt
// 信息 ID: 85480dcd3ea24d5aa364c7dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >; module=61, inline_record=13396; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::swapchain::SwapchainError> >; module=2311, inline_record=524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
