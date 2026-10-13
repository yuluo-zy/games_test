// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/walls/material_stats.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 926e120a1c53824c34fa2480570be9e0338dd9fe35131d065c9b4ed573f3b1f7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: material_stats.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 491, 556, 1451
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use atomic_float; // namespace candidate
//   package identities: [{"name":"atomic_float","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CAE
// 首条位置: {"module":556,"inlinee":31918}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::init_this_update
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":3904,"rva":11043456,"va":5379752576}
// 编译实例 RVAs: 0xa88280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4671121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379752576,"raw_hex":"e86fb9b8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379752620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::store
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":4368,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":31870,"source_line":204,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":4072,"parent_record_offset":3904,"ancestor_record_offset":3904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":556,"record_offset":4308,"parent_record_offset":4284,"ancestor_record_offset":3904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":556,"record_offset":4284,"parent_record_offset":4196,"ancestor_record_offset":3904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":4224,"parent_record_offset":4196,"ancestor_record_offset":3904,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":4196,"parent_record_offset":4172,"ancestor_record_offset":3904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":4172,"parent_record_offset":4152,"ancestor_record_offset":3904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":4480,"parent_record_offset":4452,"ancestor_record_offset":3904,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":556,"record_offset":4252,"parent_record_offset":4224,"ancestor_record_offset":3904,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":4092,"parent_record_offset":4072,"ancestor_record_offset":3904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3904,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3904,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":4636,"rva":11043616,"va":5379752736}
// 编译实例 RVAs: 0xa88320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4671326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379752736,"raw_hex":"e8a2b8b8ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379752825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31739473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379752736,"raw_hex":"e8514ee401","size":5,"soft_fail":false,"target_va":5411492368,"va":5379752890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2070798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379752736,"raw_hex":"e80e991f00","size":5,"soft_fail":false,"target_va":5381823600,"va":5379752797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2071155","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379752736,"raw_hex":"e8739a1f00","size":5,"soft_fail":false,"target_va":5381824016,"va":5379752856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tjmp\t31741885","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379752736,"raw_hex":"e9bd57e401","size":5,"soft_fail":false,"target_va":5411494800,"va":5379752910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":4964,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":4808,"parent_record_offset":4636,"ancestor_record_offset":4636,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":556,"record_offset":5140,"parent_record_offset":5116,"ancestor_record_offset":4636,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":556,"record_offset":5116,"parent_record_offset":5028,"ancestor_record_offset":4636,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":5056,"parent_record_offset":5028,"ancestor_record_offset":4636,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":5028,"parent_record_offset":5004,"ancestor_record_offset":4636,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":556,"record_offset":5004,"parent_record_offset":4984,"ancestor_record_offset":4636,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":556,"record_offset":5084,"parent_record_offset":5056,"ancestor_record_offset":4636,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":4984,"parent_record_offset":4964,"ancestor_record_offset":4636,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":5300,"parent_record_offset":5240,"ancestor_record_offset":4636,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":4872,"parent_record_offset":4836,"ancestor_record_offset":4636,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":4636,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":4636,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":5212,"parent_record_offset":5188,"ancestor_record_offset":4636,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":4836,"parent_record_offset":4808,"ancestor_record_offset":4636,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":5240,"parent_record_offset":5212,"ancestor_record_offset":4636,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":556,"record_offset":4928,"parent_record_offset":4836,"ancestor_record_offset":4636,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":556,"record_offset":5272,"parent_record_offset":5240,"ancestor_record_offset":4636,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CB3
// 首条位置: {"module":1451,"inlinee":31923}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1451,"record_offset":10152,"parent_record_offset":10012,"ancestor_record_offset":10012,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10040,"parent_record_offset":10012,"ancestor_record_offset":10012,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":10012,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":5604,"rva":11043872,"va":5379752992}
// 编译实例 RVAs: 0xa88420
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":5776,"parent_record_offset":5604,"ancestor_record_offset":5604,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":5796,"parent_record_offset":5776,"ancestor_record_offset":5604,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":5832,"rva":11043888,"va":5379753008}
// 编译实例 RVAs: 0xa88430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6004,"parent_record_offset":5832,"ancestor_record_offset":5832,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6024,"parent_record_offset":6004,"ancestor_record_offset":5832,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":6060,"rva":11043904,"va":5379753024}
// 编译实例 RVAs: 0xa88440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6236,"parent_record_offset":6060,"ancestor_record_offset":6060,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6256,"parent_record_offset":6236,"ancestor_record_offset":6060,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":6292,"rva":11043920,"va":5379753040}
// 编译实例 RVAs: 0xa88450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6464,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":31871,"source_line":536,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6848,"parent_record_offset":6292,"ancestor_record_offset":6292,"inlinee":31882,"source_line":559,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6508,"parent_record_offset":6484,"ancestor_record_offset":6292,"inlinee":31873,"source_line":458,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6484,"parent_record_offset":6464,"ancestor_record_offset":6292,"inlinee":31872,"source_line":511,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6744,"parent_record_offset":6724,"ancestor_record_offset":6292,"inlinee":31881,"source_line":537,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7136,"parent_record_offset":7116,"ancestor_record_offset":6292,"inlinee":31884,"source_line":560,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6612,"parent_record_offset":6532,"ancestor_record_offset":6292,"inlinee":31876,"source_line":469,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":6724,"parent_record_offset":6612,"ancestor_record_offset":6292,"inlinee":31879,"source_line":515,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":6292,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":6688,"parent_record_offset":6668,"ancestor_record_offset":6292,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":6668,"parent_record_offset":6636,"ancestor_record_offset":6292,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6776,"parent_record_offset":6532,"ancestor_record_offset":6292,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":6292,"inlinee":31874,"source_line":3169,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6560,"parent_record_offset":6532,"ancestor_record_offset":6292,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6292,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":6580,"parent_record_offset":6560,"ancestor_record_offset":6292,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_arches_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 98
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":7244,"rva":11044000,"va":5379753120}
// 编译实例 RVAs: 0xa884a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7412,"parent_record_offset":7244,"ancestor_record_offset":7244,"inlinee":31871,"source_line":536,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7796,"parent_record_offset":7244,"ancestor_record_offset":7244,"inlinee":31882,"source_line":559,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7456,"parent_record_offset":7432,"ancestor_record_offset":7244,"inlinee":31873,"source_line":458,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7432,"parent_record_offset":7412,"ancestor_record_offset":7244,"inlinee":31872,"source_line":511,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7692,"parent_record_offset":7672,"ancestor_record_offset":7244,"inlinee":31881,"source_line":537,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8084,"parent_record_offset":8064,"ancestor_record_offset":7244,"inlinee":31884,"source_line":560,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7560,"parent_record_offset":7480,"ancestor_record_offset":7244,"inlinee":31876,"source_line":469,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":7672,"parent_record_offset":7560,"ancestor_record_offset":7244,"inlinee":31879,"source_line":515,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":7584,"parent_record_offset":7560,"ancestor_record_offset":7244,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":7636,"parent_record_offset":7616,"ancestor_record_offset":7244,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":7616,"parent_record_offset":7584,"ancestor_record_offset":7244,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":7724,"parent_record_offset":7480,"ancestor_record_offset":7244,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":7244,"inlinee":31874,"source_line":3169,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":7508,"parent_record_offset":7480,"ancestor_record_offset":7244,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":7748,"parent_record_offset":7724,"ancestor_record_offset":7244,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":7528,"parent_record_offset":7508,"ancestor_record_offset":7244,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_gate_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":8192,"rva":11044080,"va":5379753200}
// 编译实例 RVAs: 0xa884f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8360,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":31871,"source_line":536,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8744,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":31882,"source_line":559,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8404,"parent_record_offset":8380,"ancestor_record_offset":8192,"inlinee":31873,"source_line":458,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8380,"parent_record_offset":8360,"ancestor_record_offset":8192,"inlinee":31872,"source_line":511,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8640,"parent_record_offset":8620,"ancestor_record_offset":8192,"inlinee":31881,"source_line":537,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9032,"parent_record_offset":9012,"ancestor_record_offset":8192,"inlinee":31884,"source_line":560,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8508,"parent_record_offset":8428,"ancestor_record_offset":8192,"inlinee":31876,"source_line":469,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":8620,"parent_record_offset":8508,"ancestor_record_offset":8192,"inlinee":31879,"source_line":515,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":8192,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":8584,"parent_record_offset":8564,"ancestor_record_offset":8192,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":8564,"parent_record_offset":8532,"ancestor_record_offset":8192,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":8672,"parent_record_offset":8428,"ancestor_record_offset":8192,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":8192,"inlinee":31874,"source_line":3169,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":8456,"parent_record_offset":8428,"ancestor_record_offset":8192,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":8696,"parent_record_offset":8672,"ancestor_record_offset":8192,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":8476,"parent_record_offset":8456,"ancestor_record_offset":8192,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_sewer_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":9140,"rva":11044160,"va":5379753280}
// 编译实例 RVAs: 0xa88540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9308,"parent_record_offset":9140,"ancestor_record_offset":9140,"inlinee":31871,"source_line":536,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9692,"parent_record_offset":9140,"ancestor_record_offset":9140,"inlinee":31882,"source_line":559,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9352,"parent_record_offset":9328,"ancestor_record_offset":9140,"inlinee":31873,"source_line":458,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9328,"parent_record_offset":9308,"ancestor_record_offset":9140,"inlinee":31872,"source_line":511,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9588,"parent_record_offset":9568,"ancestor_record_offset":9140,"inlinee":31881,"source_line":537,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9980,"parent_record_offset":9960,"ancestor_record_offset":9140,"inlinee":31884,"source_line":560,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9456,"parent_record_offset":9376,"ancestor_record_offset":9140,"inlinee":31876,"source_line":469,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":9568,"parent_record_offset":9456,"ancestor_record_offset":9140,"inlinee":31879,"source_line":515,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":9480,"parent_record_offset":9456,"ancestor_record_offset":9140,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":9532,"parent_record_offset":9512,"ancestor_record_offset":9140,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":9512,"parent_record_offset":9480,"ancestor_record_offset":9140,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":9620,"parent_record_offset":9376,"ancestor_record_offset":9140,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":9376,"parent_record_offset":9352,"ancestor_record_offset":9140,"inlinee":31874,"source_line":3169,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":9404,"parent_record_offset":9376,"ancestor_record_offset":9140,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":9140,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":9424,"parent_record_offset":9404,"ancestor_record_offset":9140,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
// 记录形式: compiled_procedure; instances=1
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":10088,"rva":11044240,"va":5379753360}
// 编译实例 RVAs: 0xa88590
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10256,"parent_record_offset":10088,"ancestor_record_offset":10088,"inlinee":31871,"source_line":536,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10640,"parent_record_offset":10088,"ancestor_record_offset":10088,"inlinee":31882,"source_line":559,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10300,"parent_record_offset":10276,"ancestor_record_offset":10088,"inlinee":31873,"source_line":458,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10276,"parent_record_offset":10256,"ancestor_record_offset":10088,"inlinee":31872,"source_line":511,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10536,"parent_record_offset":10516,"ancestor_record_offset":10088,"inlinee":31881,"source_line":537,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10928,"parent_record_offset":10908,"ancestor_record_offset":10088,"inlinee":31884,"source_line":560,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10404,"parent_record_offset":10324,"ancestor_record_offset":10088,"inlinee":31876,"source_line":469,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":10516,"parent_record_offset":10404,"ancestor_record_offset":10088,"inlinee":31879,"source_line":515,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":10428,"parent_record_offset":10404,"ancestor_record_offset":10088,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":10480,"parent_record_offset":10460,"ancestor_record_offset":10088,"inlinee":11240,"source_line":1096,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":10460,"parent_record_offset":10428,"ancestor_record_offset":10088,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":10568,"parent_record_offset":10324,"ancestor_record_offset":10088,"inlinee":7020,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":10324,"parent_record_offset":10300,"ancestor_record_offset":10088,"inlinee":31874,"source_line":3169,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":10352,"parent_record_offset":10324,"ancestor_record_offset":10088,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":10592,"parent_record_offset":10568,"ancestor_record_offset":10088,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":10372,"parent_record_offset":10352,"ancestor_record_offset":10088,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 118
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":11036,"rva":11044320,"va":5379753440}
// 编译实例 RVAs: 0xa885e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11212,"parent_record_offset":11036,"ancestor_record_offset":11036,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11232,"parent_record_offset":11212,"ancestor_record_offset":11036,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 124
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":11268,"rva":11044336,"va":5379753456}
// 编译实例 RVAs: 0xa885f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11440,"parent_record_offset":11268,"ancestor_record_offset":11268,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11460,"parent_record_offset":11440,"ancestor_record_offset":11268,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":11496,"rva":11044352,"va":5379753472}
// 编译实例 RVAs: 0xa88600
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11668,"parent_record_offset":11496,"ancestor_record_offset":11496,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11688,"parent_record_offset":11668,"ancestor_record_offset":11496,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":11724,"rva":11044368,"va":5379753488}
// 编译实例 RVAs: 0xa88610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11896,"parent_record_offset":11724,"ancestor_record_offset":11724,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":11916,"parent_record_offset":11896,"ancestor_record_offset":11724,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":11952,"rva":11044384,"va":5379753504}
// 编译实例 RVAs: 0xa88620
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12124,"parent_record_offset":11952,"ancestor_record_offset":11952,"inlinee":20619,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12144,"parent_record_offset":12124,"ancestor_record_offset":11952,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
// 记录形式: compiled_procedure; instances=1
// 源码行: 140
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":12180,"rva":11044400,"va":5379753520}
// 编译实例 RVAs: 0xa88630
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12352,"parent_record_offset":12180,"ancestor_record_offset":12180,"inlinee":7965,"source_line":2725,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12372,"parent_record_offset":12352,"ancestor_record_offset":12180,"inlinee":5913,"source_line":3712,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CC2
// 首条位置: {"module":1451,"inlinee":31938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10680,"parent_record_offset":10652,"ancestor_record_offset":10652,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":10652,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CC5
// 首条位置: {"module":1451,"inlinee":31941}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":14348,"parent_record_offset":14324,"ancestor_record_offset":14324,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":14324,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C8F
// 首条位置: {"module":556,"inlinee":31887}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":13932,"parent_record_offset":13900,"ancestor_record_offset":13900,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10796,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":13900,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13988,"parent_record_offset":13956,"ancestor_record_offset":13900,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C94
// 首条位置: {"module":556,"inlinee":31892}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":14924,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14972,"parent_record_offset":14948,"ancestor_record_offset":14924,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14996,"parent_record_offset":14972,"ancestor_record_offset":14924,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C91
// 首条位置: {"module":556,"inlinee":31889}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":14384,"parent_record_offset":14348,"ancestor_record_offset":14348,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":11180,"parent_record_offset":11156,"ancestor_record_offset":11132,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14412,"parent_record_offset":14384,"ancestor_record_offset":14348,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14444,"parent_record_offset":14412,"ancestor_record_offset":14348,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C92
// 首条位置: {"module":556,"inlinee":31890}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":14580,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":14580,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C93
// 首条位置: {"module":556,"inlinee":31891}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":14692,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":11012,"parent_record_offset":10988,"ancestor_record_offset":10964,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14748,"parent_record_offset":14720,"ancestor_record_offset":14692,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14784,"parent_record_offset":14748,"ancestor_record_offset":14692,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CC8
// 首条位置: {"module":1451,"inlinee":31944}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":9896,"parent_record_offset":9868,"ancestor_record_offset":9868,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":9868,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CC9
// 首条位置: {"module":1451,"inlinee":31945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10212,"parent_record_offset":10184,"ancestor_record_offset":10184,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":10236,"parent_record_offset":10212,"ancestor_record_offset":10184,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement
// 记录形式: compiled_procedure; instances=1
// 源码行: 204
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":12408,"rva":11044416,"va":5379753536}
// 编译实例 RVAs: 0xa88640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9595777","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379753536,"raw_hex":"e87f946dff","size":5,"soft_fail":false,"target_va":5370158128,"va":5379753900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31738560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379753536,"raw_hex":"e8c04ae401","size":5,"soft_fail":false,"target_va":5411492368,"va":5379753803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t31740955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379753536,"raw_hex":"e81b54e401","size":5,"soft_fail":false,"target_va":5411494800,"va":5379753840,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2070004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379753536,"raw_hex":"e8f4951f00","size":5,"soft_fail":false,"target_va":5381823600,"va":5379753591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2070272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379753536,"raw_hex":"e800971f00","size":5,"soft_fail":false,"target_va":5381824016,"va":5379753739,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":13128,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":556,"record_offset":13052,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":556,"record_offset":12912,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":556,"record_offset":13308,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":556,"record_offset":13216,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":556,"record_offset":13160,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":12600,"parent_record_offset":12408,"ancestor_record_offset":12408,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":13008,"parent_record_offset":12984,"ancestor_record_offset":12408,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":12984,"parent_record_offset":12960,"ancestor_record_offset":12408,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":556,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":12408,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":556,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":12408,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":556,"record_offset":13084,"parent_record_offset":13052,"ancestor_record_offset":12408,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":556,"record_offset":13184,"parent_record_offset":13160,"ancestor_record_offset":12408,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13428,"parent_record_offset":13364,"ancestor_record_offset":12408,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12664,"parent_record_offset":12628,"ancestor_record_offset":12408,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":12408,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":12688,"parent_record_offset":12664,"ancestor_record_offset":12408,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":556,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":12408,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":556,"record_offset":13336,"parent_record_offset":13308,"ancestor_record_offset":12408,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":12628,"parent_record_offset":12600,"ancestor_record_offset":12408,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":556,"record_offset":13364,"parent_record_offset":13336,"ancestor_record_offset":12408,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":556,"record_offset":12720,"parent_record_offset":12628,"ancestor_record_offset":12408,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":556,"record_offset":13400,"parent_record_offset":13364,"ancestor_record_offset":12408,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":556,"record_offset":13272,"parent_record_offset":13240,"ancestor_record_offset":12408,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C90
// 首条位置: {"module":556,"inlinee":31888}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":14148,"parent_record_offset":14124,"ancestor_record_offset":14124,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":11264,"parent_record_offset":11240,"ancestor_record_offset":11216,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14172,"parent_record_offset":14148,"ancestor_record_offset":14124,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":14124,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7C95
// 首条位置: {"module":556,"inlinee":31893}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":15156,"parent_record_offset":15120,"ancestor_record_offset":15120,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":11048,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":15120,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":15216,"parent_record_offset":15184,"ancestor_record_offset":15120,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CCB
// 首条位置: {"module":1451,"inlinee":31947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":1451,"record_offset":9136,"parent_record_offset":9064,"ancestor_record_offset":9064,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":9248,"parent_record_offset":9064,"ancestor_record_offset":9064,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1451,"record_offset":9160,"parent_record_offset":9136,"ancestor_record_offset":9064,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":9384,"parent_record_offset":9360,"ancestor_record_offset":9064,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":9408,"parent_record_offset":9384,"ancestor_record_offset":9064,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CCC
// 首条位置: {"module":1451,"inlinee":31948}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":1451,"record_offset":11800,"parent_record_offset":11768,"ancestor_record_offset":11768,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":11768,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1451,"record_offset":11848,"parent_record_offset":11824,"ancestor_record_offset":11768,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01451.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most
// 记录形式: compiled_procedure; instances=1
// 源码行: 280
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":556,"record_offset":13688,"rva":11044816,"va":5379753936}
// 编译实例 RVAs: 0xa887d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":14320,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":556,"record_offset":15380,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":19410,"source_line":3198,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":14124,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31888,"source_line":220,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":14924,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31892,"source_line":166,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":13900,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31887,"source_line":161,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":14580,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31890,"source_line":176,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":14348,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31889,"source_line":171,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":14692,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31891,"source_line":180,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":15120,"parent_record_offset":13688,"ancestor_record_offset":13688,"inlinee":31893,"source_line":225,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=13
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":556,"record_offset":13932,"parent_record_offset":13900,"ancestor_record_offset":13688,"inlinee":31885,"source_line":172,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":15800,"parent_record_offset":15772,"ancestor_record_offset":13688,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":15668,"parent_record_offset":15620,"ancestor_record_offset":13688,"inlinee":19414,"source_line":1594,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":14628,"parent_record_offset":14604,"ancestor_record_offset":13688,"inlinee":31877,"source_line":1141,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":15772,"parent_record_offset":15744,"ancestor_record_offset":13688,"inlinee":8314,"source_line":1353,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":556,"record_offset":15476,"parent_record_offset":15428,"ancestor_record_offset":13688,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":556,"record_offset":15572,"parent_record_offset":15524,"ancestor_record_offset":13688,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":556,"record_offset":15620,"parent_record_offset":15572,"ancestor_record_offset":13688,"inlinee":19413,"source_line":3205,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":556,"record_offset":15428,"parent_record_offset":15380,"ancestor_record_offset":13688,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":556,"record_offset":15716,"parent_record_offset":15668,"ancestor_record_offset":13688,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":556,"record_offset":15524,"parent_record_offset":15476,"ancestor_record_offset":13688,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13956,"parent_record_offset":13932,"ancestor_record_offset":13688,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":556,"record_offset":13988,"parent_record_offset":13956,"ancestor_record_offset":13688,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":556,"record_offset":15744,"parent_record_offset":15716,"ancestor_record_offset":13688,"inlinee":27215,"source_line":310,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6A4F
// 首条位置: {"module":491,"inlinee":27215}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":556,"record_offset":15772,"parent_record_offset":15744,"ancestor_record_offset":15744,"inlinee":8314,"source_line":1353,"archive":"_pdb/B/modules/00556.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$77::cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":556,"record_offset":15800,"parent_record_offset":15772,"ancestor_record_offset":15744,"inlinee":8316,"source_line":1836,"archive":"_pdb/B/modules/00556.jsonl.gz"}
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
// 完整结构及原始类型记录: material_stats.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::material_stats::MaterialStats
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 38; 过程引用数: 25
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::material_stats::MaterialStats>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::material_stats::MaterialStats"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: material_stats.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/B-owners/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::resources::walls::material_stats::MaterialStats", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: material_stats.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_arches_length
// 信息 ID: bbf6773c380dde003eb2280e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7629ea -> 0xa884a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_gate_length
// 信息 ID: b1e5ab3172e1f29f939507d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x762a01 -> 0xa884f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
// 信息 ID: 7bdb3766d4255e4f98854d9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76f316 -> 0xa88450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_sewer_length
// 信息 ID: a7554544e6ba1b452e13807a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x762a2e -> 0xa88540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
// 信息 ID: 6fb29519484306b372e539f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7774aa -> 0xa88590 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 信息 ID: ca1ea5d4367cc1d57679ddac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=14124; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11216; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 信息 ID: 0de3c959d39c1fa7bd6d88b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 信息 ID: 5dc84d2f4761bef6f4094cb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=14924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 信息 ID: b4fbd2e9154fcf135dfc6162
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=13900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10796; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 信息 ID: 058a191c86a328a7bf89ebdc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=14580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10880; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 信息 ID: f0d6ee0ab3e02d43eb531c05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=14324; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 信息 ID: 384e61198c4ee5d9afe26716
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=9064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::init_this_update
// 信息 ID: 227538632a18de99a259a7d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x74f54e -> 0xa88280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 信息 ID: 7a6ea894b98ead7799962785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 信息 ID: 5e68f4463dd1ae84c560a17a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=14348; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11132; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 信息 ID: 4001deacadf9496d7e536ccd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=9868; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11300; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement
// 信息 ID: ec5d82bb722f64622afd8f91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator @ 0x123515b -> 0xa88640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 信息 ID: ed3e4b89cc5d4044c1a188de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10652; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13412; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 信息 ID: 0f9e0ada4082691df66975e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11444; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
// 信息 ID: 2601b9a79220409b13fe7d40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x75495a -> 0xa88430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count
// 信息 ID: 374f795610eb00cc43ba0a42
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7629d3 -> 0xa885f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
// 信息 ID: 34bbe9c74798a4e425fd2788
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x765e2d -> 0xa88620 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
// 信息 ID: eabfaf3c0d2d58687f51a086
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x765e38 -> 0xa88630 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count
// 信息 ID: 3f192e9c2fe85c721fe1d768
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x762a17 -> 0xa88600 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
// 信息 ID: a7e01d01944b98b091c381c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x754942 -> 0xa88420 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count
// 信息 ID: a19b518ef55a6d83cbbc53c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x76f2ff -> 0xa885e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
// 信息 ID: a356321abfe6b06c7a4767a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x74f55e -> 0xa88440 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
// 信息 ID: 1bdde993059de9ca0de3643d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x75498e -> 0xa88320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count
// 信息 ID: 74527e50a616d2ec8874aacd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x762a44 -> 0xa88610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 信息 ID: 6116859487bf52ab2ea89fdb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=14692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=10964; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 信息 ID: 972b29eec916c51743e299e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=15120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=11048; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=1451, inline_record=13808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most
// 信息 ID: 9abc34ef308ec021c9b1c2e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator @ 0x1235236 -> 0xa887d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 信息 ID: 4ed1f9ff81ecec19779612bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=15744; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=15868; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=15992; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=16116; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=16240; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=556, inline_record=16364; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$1::default
// 信息 ID: f171233e7ab455891be37995
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=556, inline_record=81844; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=556, inline_record=81876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
