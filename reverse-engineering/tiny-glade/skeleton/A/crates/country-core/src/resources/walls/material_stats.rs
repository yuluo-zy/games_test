// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/walls/material_stats.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cefd9972f5fc8e648a0f915795dd33541c46610019c6e8cae78160f531ea4b65
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: material_stats.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 420, 445, 486
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use atomic_float; // namespace candidate
//   package identities: [{"name":"atomic_float","source":"PDB_registry_path_only","version":"1.0.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x642B
// 首条位置: {"module":445,"inlinee":25643}
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
// 首条位置: {"module":445,"record_offset":3060,"rva":6265344,"va":5374974464}
// 编译实例 RVAs: 0x5f9a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2224801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374974464,"raw_hex":"e85f0ddeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374974508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::store
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":3512,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":25614,"source_line":204,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":3228,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":3456,"parent_record_offset":3432,"ancestor_record_offset":3060,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":3432,"parent_record_offset":3340,"ancestor_record_offset":3060,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":3060,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":3060,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":3624,"parent_record_offset":3596,"ancestor_record_offset":3060,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":3656,"parent_record_offset":3624,"ancestor_record_offset":3060,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":3060,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":3248,"parent_record_offset":3228,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":3060,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":3560,"parent_record_offset":3536,"ancestor_record_offset":3060,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":3808,"rva":6265504,"va":5374974624}
// 编译实例 RVAs: 0x5f9aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: material_stats.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2225001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374974624,"raw_hex":"e8970cdeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374974708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t27994311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374974624,"raw_hex":"e8c728ab01","size":5,"soft_fail":false,"target_va":5402968992,"va":5374974676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t27996032","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5374974624,"raw_hex":"e9802fab01","size":5,"soft_fail":false,"target_va":5402970800,"va":5374974763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":4212,"parent_record_offset":3808,"ancestor_record_offset":3808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":445,"record_offset":3984,"parent_record_offset":3808,"ancestor_record_offset":3808,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":445,"record_offset":4156,"parent_record_offset":3808,"ancestor_record_offset":3808,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":4368,"parent_record_offset":4344,"ancestor_record_offset":3808,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":445,"record_offset":4344,"parent_record_offset":4252,"ancestor_record_offset":3808,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":4280,"parent_record_offset":4252,"ancestor_record_offset":3808,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":445,"record_offset":4252,"parent_record_offset":4232,"ancestor_record_offset":3808,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":445,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":3808,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":445,"record_offset":4308,"parent_record_offset":4280,"ancestor_record_offset":3808,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":4232,"parent_record_offset":4212,"ancestor_record_offset":3808,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":445,"record_offset":4092,"parent_record_offset":4008,"ancestor_record_offset":3808,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":445,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3808,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":4496,"parent_record_offset":4464,"ancestor_record_offset":3808,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":4036,"parent_record_offset":4008,"ancestor_record_offset":3808,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":4520,"parent_record_offset":4496,"ancestor_record_offset":3808,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":4060,"parent_record_offset":4036,"ancestor_record_offset":3808,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":445,"record_offset":4436,"parent_record_offset":4412,"ancestor_record_offset":3808,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":445,"record_offset":4008,"parent_record_offset":3984,"ancestor_record_offset":3808,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":445,"record_offset":4464,"parent_record_offset":4436,"ancestor_record_offset":3808,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 67
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EEC
// 首条位置: {"module":420,"inlinee":24300}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41392,"parent_record_offset":41364,"ancestor_record_offset":41364,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41416,"parent_record_offset":41392,"ancestor_record_offset":41364,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":4812,"rva":6265712,"va":5374974832}
// 编译实例 RVAs: 0x5f9b70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":4984,"parent_record_offset":4812,"ancestor_record_offset":4812,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5004,"parent_record_offset":4984,"ancestor_record_offset":4812,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":5040,"rva":6265728,"va":5374974848}
// 编译实例 RVAs: 0x5f9b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5212,"parent_record_offset":5040,"ancestor_record_offset":5040,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5232,"parent_record_offset":5212,"ancestor_record_offset":5040,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":5268,"rva":6265744,"va":5374974864}
// 编译实例 RVAs: 0x5f9b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5444,"parent_record_offset":5268,"ancestor_record_offset":5268,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5464,"parent_record_offset":5444,"ancestor_record_offset":5268,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":5500,"rva":6265760,"va":5374974880}
// 编译实例 RVAs: 0x5f9ba0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":5672,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":25615,"source_line":536,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6112,"parent_record_offset":5500,"ancestor_record_offset":5500,"inlinee":25625,"source_line":559,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5500,"inlinee":25617,"source_line":458,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":5692,"parent_record_offset":5672,"ancestor_record_offset":5500,"inlinee":25616,"source_line":511,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6008,"parent_record_offset":5988,"ancestor_record_offset":5500,"inlinee":25624,"source_line":537,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6456,"parent_record_offset":6436,"ancestor_record_offset":5500,"inlinee":25627,"source_line":560,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":5820,"parent_record_offset":5740,"ancestor_record_offset":5500,"inlinee":25620,"source_line":469,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":5988,"parent_record_offset":5820,"ancestor_record_offset":5500,"inlinee":25622,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":5844,"parent_record_offset":5820,"ancestor_record_offset":5500,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":5876,"parent_record_offset":5844,"ancestor_record_offset":5500,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":5924,"parent_record_offset":5904,"ancestor_record_offset":5500,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":5948,"parent_record_offset":5924,"ancestor_record_offset":5500,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":5904,"parent_record_offset":5844,"ancestor_record_offset":5500,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":6040,"parent_record_offset":5740,"ancestor_record_offset":5500,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":5500,"inlinee":25618,"source_line":2878,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5768,"parent_record_offset":5740,"ancestor_record_offset":5500,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":5500,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":5788,"parent_record_offset":5768,"ancestor_record_offset":5500,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":6564,"rva":6265840,"va":5374974960}
// 编译实例 RVAs: 0x5f9bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6732,"parent_record_offset":6564,"ancestor_record_offset":6564,"inlinee":25615,"source_line":536,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7172,"parent_record_offset":6564,"ancestor_record_offset":6564,"inlinee":25625,"source_line":559,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":6564,"inlinee":25617,"source_line":458,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6752,"parent_record_offset":6732,"ancestor_record_offset":6564,"inlinee":25616,"source_line":511,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7068,"parent_record_offset":7048,"ancestor_record_offset":6564,"inlinee":25624,"source_line":537,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7516,"parent_record_offset":7496,"ancestor_record_offset":6564,"inlinee":25627,"source_line":560,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":6880,"parent_record_offset":6800,"ancestor_record_offset":6564,"inlinee":25620,"source_line":469,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7048,"parent_record_offset":6880,"ancestor_record_offset":6564,"inlinee":25622,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":6904,"parent_record_offset":6880,"ancestor_record_offset":6564,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":6936,"parent_record_offset":6904,"ancestor_record_offset":6564,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":6984,"parent_record_offset":6964,"ancestor_record_offset":6564,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":6564,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":6964,"parent_record_offset":6904,"ancestor_record_offset":6564,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":7100,"parent_record_offset":6800,"ancestor_record_offset":6564,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6564,"inlinee":25618,"source_line":2878,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":6828,"parent_record_offset":6800,"ancestor_record_offset":6564,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6564,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":6848,"parent_record_offset":6828,"ancestor_record_offset":6564,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":7624,"rva":6265920,"va":5374975040}
// 编译实例 RVAs: 0x5f9c40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7792,"parent_record_offset":7624,"ancestor_record_offset":7624,"inlinee":25615,"source_line":536,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8232,"parent_record_offset":7624,"ancestor_record_offset":7624,"inlinee":25625,"source_line":559,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7836,"parent_record_offset":7812,"ancestor_record_offset":7624,"inlinee":25617,"source_line":458,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7812,"parent_record_offset":7792,"ancestor_record_offset":7624,"inlinee":25616,"source_line":511,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8128,"parent_record_offset":8108,"ancestor_record_offset":7624,"inlinee":25624,"source_line":537,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8576,"parent_record_offset":8556,"ancestor_record_offset":7624,"inlinee":25627,"source_line":560,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":7940,"parent_record_offset":7860,"ancestor_record_offset":7624,"inlinee":25620,"source_line":469,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8108,"parent_record_offset":7940,"ancestor_record_offset":7624,"inlinee":25622,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":7624,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":7996,"parent_record_offset":7964,"ancestor_record_offset":7624,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":8044,"parent_record_offset":8024,"ancestor_record_offset":7624,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":8068,"parent_record_offset":8044,"ancestor_record_offset":7624,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":8024,"parent_record_offset":7964,"ancestor_record_offset":7624,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":8160,"parent_record_offset":7860,"ancestor_record_offset":7624,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":7624,"inlinee":25618,"source_line":2878,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":7888,"parent_record_offset":7860,"ancestor_record_offset":7624,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":8184,"parent_record_offset":8160,"ancestor_record_offset":7624,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":7908,"parent_record_offset":7888,"ancestor_record_offset":7624,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":8684,"rva":6266000,"va":5374975120}
// 编译实例 RVAs: 0x5f9c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8852,"parent_record_offset":8684,"ancestor_record_offset":8684,"inlinee":25615,"source_line":536,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9292,"parent_record_offset":8684,"ancestor_record_offset":8684,"inlinee":25625,"source_line":559,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8896,"parent_record_offset":8872,"ancestor_record_offset":8684,"inlinee":25617,"source_line":458,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":8872,"parent_record_offset":8852,"ancestor_record_offset":8684,"inlinee":25616,"source_line":511,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9188,"parent_record_offset":9168,"ancestor_record_offset":8684,"inlinee":25624,"source_line":537,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9636,"parent_record_offset":9616,"ancestor_record_offset":8684,"inlinee":25627,"source_line":560,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9000,"parent_record_offset":8920,"ancestor_record_offset":8684,"inlinee":25620,"source_line":469,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9168,"parent_record_offset":9000,"ancestor_record_offset":8684,"inlinee":25622,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":9024,"parent_record_offset":9000,"ancestor_record_offset":8684,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":8684,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":9104,"parent_record_offset":9084,"ancestor_record_offset":8684,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":9128,"parent_record_offset":9104,"ancestor_record_offset":8684,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":9084,"parent_record_offset":9024,"ancestor_record_offset":8684,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":9220,"parent_record_offset":8920,"ancestor_record_offset":8684,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":8684,"inlinee":25618,"source_line":2878,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":8948,"parent_record_offset":8920,"ancestor_record_offset":8684,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":9244,"parent_record_offset":9220,"ancestor_record_offset":8684,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":8968,"parent_record_offset":8948,"ancestor_record_offset":8684,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":9744,"rva":6266080,"va":5374975200}
// 编译实例 RVAs: 0x5f9ce0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9912,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":25615,"source_line":536,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::fetch_sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":10352,"parent_record_offset":9744,"ancestor_record_offset":9744,"inlinee":25625,"source_line":559,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::fetch_update
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":9744,"inlinee":25617,"source_line":458,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::update_with
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":9932,"parent_record_offset":9912,"ancestor_record_offset":9744,"inlinee":25616,"source_line":511,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_add::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":10248,"parent_record_offset":10228,"ancestor_record_offset":9744,"inlinee":25624,"source_line":537,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_sub::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":10696,"parent_record_offset":10676,"ancestor_record_offset":9744,"inlinee":25627,"source_line":560,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::fetch_update::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":10060,"parent_record_offset":9980,"ancestor_record_offset":9744,"inlinee":25620,"source_line":469,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::impl$2::update_with::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":10228,"parent_record_offset":10060,"ancestor_record_offset":9744,"inlinee":25622,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":10084,"parent_record_offset":10060,"ancestor_record_offset":9744,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":10116,"parent_record_offset":10084,"ancestor_record_offset":9744,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":10164,"parent_record_offset":10144,"ancestor_record_offset":9744,"inlinee":12635,"source_line":1110,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::to_bits::rt_f32_to_u32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":10188,"parent_record_offset":10164,"ancestor_record_offset":9744,"inlinee":12637,"source_line":1150,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":10144,"parent_record_offset":10084,"ancestor_record_offset":9744,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":10280,"parent_record_offset":9980,"ancestor_record_offset":9744,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_update
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":9744,"inlinee":25618,"source_line":2878,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":10008,"parent_record_offset":9980,"ancestor_record_offset":9744,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":9744,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":10028,"parent_record_offset":10008,"ancestor_record_offset":9744,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":10804,"rva":6266160,"va":5374975280}
// 编译实例 RVAs: 0x5f9d30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":10980,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11000,"parent_record_offset":10980,"ancestor_record_offset":10804,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":11036,"rva":6266176,"va":5374975296}
// 编译实例 RVAs: 0x5f9d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11208,"parent_record_offset":11036,"ancestor_record_offset":11036,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11228,"parent_record_offset":11208,"ancestor_record_offset":11036,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":11264,"rva":6266192,"va":5374975312}
// 编译实例 RVAs: 0x5f9d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11436,"parent_record_offset":11264,"ancestor_record_offset":11264,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11456,"parent_record_offset":11436,"ancestor_record_offset":11264,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":11492,"rva":6266208,"va":5374975328}
// 编译实例 RVAs: 0x5f9d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11664,"parent_record_offset":11492,"ancestor_record_offset":11492,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11684,"parent_record_offset":11664,"ancestor_record_offset":11492,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":11720,"rva":6266224,"va":5374975344}
// 编译实例 RVAs: 0x5f9d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11892,"parent_record_offset":11720,"ancestor_record_offset":11720,"inlinee":18065,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":11912,"parent_record_offset":11892,"ancestor_record_offset":11720,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":11948,"rva":6266240,"va":5374975360}
// 编译实例 RVAs: 0x5f9d80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12120,"parent_record_offset":11948,"ancestor_record_offset":11948,"inlinee":5322,"source_line":2440,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12140,"parent_record_offset":12120,"ancestor_record_offset":11948,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EEE
// 首条位置: {"module":420,"inlinee":24302}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41940,"parent_record_offset":41912,"ancestor_record_offset":41912,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41964,"parent_record_offset":41940,"ancestor_record_offset":41912,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 157
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF6
// 首条位置: {"module":420,"inlinee":24310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":47208,"parent_record_offset":47184,"ancestor_record_offset":47184,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":47232,"parent_record_offset":47208,"ancestor_record_offset":47184,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 161
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EEF
// 首条位置: {"module":420,"inlinee":24303}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":42084,"parent_record_offset":42056,"ancestor_record_offset":42056,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42108,"parent_record_offset":42084,"ancestor_record_offset":42056,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42132,"parent_record_offset":42108,"ancestor_record_offset":42056,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":13748,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":13748,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 166
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x641C
// 首条位置: {"module":445,"inlinee":25628}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":15092,"parent_record_offset":15068,"ancestor_record_offset":15068,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":15116,"parent_record_offset":15092,"ancestor_record_offset":15068,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":15140,"parent_record_offset":15116,"ancestor_record_offset":15068,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 171
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF3
// 首条位置: {"module":420,"inlinee":24307}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":42876,"parent_record_offset":42848,"ancestor_record_offset":42848,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":42848,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42924,"parent_record_offset":42900,"ancestor_record_offset":42848,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14324,"parent_record_offset":14296,"ancestor_record_offset":14260,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14360,"parent_record_offset":14324,"ancestor_record_offset":14260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 176
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF0
// 首条位置: {"module":420,"inlinee":24304}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":42280,"parent_record_offset":42256,"ancestor_record_offset":42256,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42304,"parent_record_offset":42280,"ancestor_record_offset":42256,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42328,"parent_record_offset":42304,"ancestor_record_offset":42256,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF1
// 首条位置: {"module":420,"inlinee":24305}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":42476,"parent_record_offset":42452,"ancestor_record_offset":42452,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42500,"parent_record_offset":42476,"ancestor_record_offset":42452,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42524,"parent_record_offset":42500,"ancestor_record_offset":42452,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14836,"parent_record_offset":14808,"ancestor_record_offset":14780,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14872,"parent_record_offset":14836,"ancestor_record_offset":14780,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EEB
// 首条位置: {"module":420,"inlinee":24299}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41248,"parent_record_offset":41220,"ancestor_record_offset":41220,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41272,"parent_record_offset":41248,"ancestor_record_offset":41220,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EED
// 首条位置: {"module":420,"inlinee":24301}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41536,"parent_record_offset":41508,"ancestor_record_offset":41508,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":41560,"parent_record_offset":41536,"ancestor_record_offset":41508,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":12176,"rva":6266256,"va":5374975376}
// 编译实例 RVAs: 0x5f9d90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t27993538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374975376,"raw_hex":"e8c225ab01","size":5,"soft_fail":false,"target_va":5402968992,"va":5374975449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t27995191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374975376,"raw_hex":"e8372cab01","size":5,"soft_fail":false,"target_va":5402970800,"va":5374975604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":12792,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":445,"record_offset":12716,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":445,"record_offset":12972,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":12880,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":12824,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":445,"record_offset":12364,"parent_record_offset":12176,"ancestor_record_offset":12176,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":445,"record_offset":12748,"parent_record_offset":12716,"ancestor_record_offset":12176,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":445,"record_offset":12848,"parent_record_offset":12824,"ancestor_record_offset":12176,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":445,"record_offset":12480,"parent_record_offset":12392,"ancestor_record_offset":12176,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":445,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":12176,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13056,"parent_record_offset":13020,"ancestor_record_offset":12176,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12424,"parent_record_offset":12392,"ancestor_record_offset":12176,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13084,"parent_record_offset":13056,"ancestor_record_offset":12176,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":12176,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":445,"record_offset":12904,"parent_record_offset":12880,"ancestor_record_offset":12176,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":445,"record_offset":12996,"parent_record_offset":12972,"ancestor_record_offset":12176,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":445,"record_offset":12392,"parent_record_offset":12364,"ancestor_record_offset":12176,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":445,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":12176,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":445,"record_offset":12936,"parent_record_offset":12904,"ancestor_record_offset":12176,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 220
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF4
// 首条位置: {"module":420,"inlinee":24308}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":43076,"parent_record_offset":43048,"ancestor_record_offset":43048,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":43100,"parent_record_offset":43076,"ancestor_record_offset":43048,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":43124,"parent_record_offset":43100,"ancestor_record_offset":43048,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":14036,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":14108,"parent_record_offset":14084,"ancestor_record_offset":14036,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 225
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF2
// 首条位置: {"module":420,"inlinee":24306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":42676,"parent_record_offset":42648,"ancestor_record_offset":42648,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42700,"parent_record_offset":42676,"ancestor_record_offset":42648,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":42724,"parent_record_offset":42700,"ancestor_record_offset":42648,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":15332,"parent_record_offset":15304,"ancestor_record_offset":15264,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":15368,"parent_record_offset":15332,"ancestor_record_offset":15264,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 230
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EE5
// 首条位置: {"module":420,"inlinee":24293}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":40320,"parent_record_offset":40256,"ancestor_record_offset":40256,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":40488,"parent_record_offset":40256,"ancestor_record_offset":40256,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":40344,"parent_record_offset":40320,"ancestor_record_offset":40256,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":420,"record_offset":40368,"parent_record_offset":40344,"ancestor_record_offset":40256,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":40652,"parent_record_offset":40628,"ancestor_record_offset":40256,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":40676,"parent_record_offset":40652,"ancestor_record_offset":40256,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5EF5
// 首条位置: {"module":420,"inlinee":24309}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":420,"record_offset":45552,"parent_record_offset":45512,"ancestor_record_offset":45512,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":45576,"parent_record_offset":45552,"ancestor_record_offset":45512,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00420.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":420,"record_offset":45600,"parent_record_offset":45576,"ancestor_record_offset":45512,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00420.jsonl.gz"}
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
// 首条位置: {"module":445,"record_offset":13536,"rva":6266656,"va":5374975776}
// 编译实例 RVAs: 0x5f9f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":15592,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":17069,"source_line":3248,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":14036,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24308,"source_line":220,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":15068,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":25628,"source_line":166,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":13748,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24303,"source_line":161,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":14556,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24304,"source_line":176,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":14260,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24307,"source_line":171,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":14780,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24305,"source_line":180,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":15264,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":24306,"source_line":225,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":445,"record_offset":14232,"parent_record_offset":13536,"ancestor_record_offset":13536,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] atomic_float::atomic_f32::AtomicF32::load
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"atomic_float","version":"1.0.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/atomic_float-1.0.0/src/atomic_f32.rs
//     evidence={"module":445,"record_offset":13780,"parent_record_offset":13748,"ancestor_record_offset":13536,"inlinee":24295,"source_line":172,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":445,"record_offset":15880,"parent_record_offset":15832,"ancestor_record_offset":13536,"inlinee":25631,"source_line":1344,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":14604,"parent_record_offset":14580,"ancestor_record_offset":13536,"inlinee":24296,"source_line":1203,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::from_bits::rt_u32_to_f32
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":13868,"parent_record_offset":13804,"ancestor_record_offset":13536,"inlinee":24298,"source_line":1245,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":13536,"inlinee":12558,"source_line":1464,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":445,"record_offset":15688,"parent_record_offset":15640,"ancestor_record_offset":13536,"inlinee":11890,"source_line":96,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":445,"record_offset":15784,"parent_record_offset":15736,"ancestor_record_offset":13536,"inlinee":6128,"source_line":106,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":15832,"parent_record_offset":15784,"ancestor_record_offset":13536,"inlinee":25630,"source_line":3255,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":445,"record_offset":15640,"parent_record_offset":15592,"ancestor_record_offset":13536,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":445,"record_offset":15928,"parent_record_offset":15880,"ancestor_record_offset":13536,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":445,"record_offset":15736,"parent_record_offset":15688,"ancestor_record_offset":13536,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":13536,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":445,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":13536,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/material_stats.rs
//     evidence={"module":445,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":13536,"inlinee":25633,"source_line":310,"archive":"_pdb/A/modules/00445.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 310
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6421
// 首条位置: {"module":445,"inlinee":25633}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::total_cmp
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":445,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":15952,"inlinee":12558,"source_line":1464,"archive":"_pdb/A/modules/00445.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 38; 过程引用数: 26
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::material_stats::MaterialStats>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::material_stats::MaterialStats"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: material_stats.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: material_stats.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_arches_length
// 信息 ID: 39a14eae7920e807d25a63bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da1cc -> 0x5f9bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_gate_length
// 信息 ID: 4de2e040755a1503fa902e24
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da1e3 -> 0x5f9c40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_lintels_length
// 信息 ID: 5f9093c42b047322a92d8754
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4def9b -> 0x5f9ba0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_sewer_length
// 信息 ID: 8cc8b92fe1224ca2292ac581
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da210 -> 0x5f9c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::add_stairs_length
// 信息 ID: 922eee2500a654e2bc4895a7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1a36 -> 0x5f9ce0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_length_delta
// 信息 ID: fc7430051911cedcea76b8fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=43048; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=14036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::arches_pillar_delta
// 信息 ID: c28b7b7de648c54131ccd891
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=41508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::bricks_length_delta
// 信息 ID: d0b19adbf8ac55524e1dcdcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=15068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::fence_length_delta
// 信息 ID: 8eb94e7f50a692e077970c90
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=42056; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=43944; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=13748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::gates_length_delta
// 信息 ID: a1a0e0bc01e8fcbf904aeb9a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=42256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=44144; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=14556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_fence_seed_changed
// 信息 ID: a9627eee73835570df660e14
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=47184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::has_material_switched
// 信息 ID: 3600533d903b122db8c303f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=40256; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::init_this_update
// 信息 ID: 2965c83ba8b7c509da4b7e4f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x4f4075 -> 0x5f9a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::latest_update_material_proportions
// 信息 ID: 468e975cb6ab035d315a55e9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=45512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_length_delta
// 信息 ID: bdb18a4d4aaa9e649f987179
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=42848; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=44736; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=14260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::lintels_pillar_delta
// 信息 ID: f6adaaa8428c81380ec716ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=41220; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=43248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::max_pillar_movement
// 信息 ID: 04e83f64ae2da8ab6e9d18ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator @ 0x59fa72 -> 0x5f9d90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::path_segmentation_delta
// 信息 ID: 9e34a63bcfefe522533b6ec5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=41912; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=43800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::pillar_count
// 信息 ID: 78b79bad20ca9454df596f7f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=41364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=43392; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arch_pillars_count
// 信息 ID: 2eab122527a7270fbb0a9009
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e7bd8 -> 0x5f9b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_arches_segments_count
// 信息 ID: 9d3ccad376121b4eb9a0fb6c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da1b5 -> 0x5f9d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_count
// 信息 ID: ba2a514d451b4cb11026ea96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x4e911f -> 0x5f9d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_fences_segments_seed
// 信息 ID: 96c9825f3906a59b9457f1cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_fences::construct_fences @ 0x4e912a -> 0x5f9d80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_gates_segments_count
// 信息 ID: f70a59f0d68e12541c2359c1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da1f9 -> 0x5f9d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintel_pillars_count
// 信息 ID: c6da24defc8c06c76be5c587
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e7bc0 -> 0x5f9b70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_lintels_segments_count
// 信息 ID: 37417d79e0bacf6f6354eef5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x4def88 -> 0x5f9d30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_path_segmentation_count
// 信息 ID: ec7d26e43628e77798e2a44b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::segmentation::calculate_wall_path_segmentation::calculate_path_wall_segmentation @ 0x4f4085 -> 0x5f9b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_pillar_placements
// 信息 ID: 3f01d91b5dc41333bcbf2c4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::place_pillars::place_pillars @ 0x4e7c0c -> 0x5f9aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::set_sewer_segments_count
// 信息 ID: 74ff57666c4e5432070e8408
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x4da226 -> 0x5f9d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::sewer_length_delta
// 信息 ID: 02b2f2140e46b025074e8851
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=42452; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=44340; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=14780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::stairs_length_delta
// 信息 ID: 49cb4eee661b1d23b89de052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=42648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator; module=420, inline_record=44536; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=15264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most
// 信息 ID: 8d1b4e8e49df2663bdf54bad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_orchestrator::wall_audio_orchestrator @ 0x59fbb7 -> 0x5f9f20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most::closure$0
// 信息 ID: b277af38f92ef5da01736fc4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::material_stats::impl$0::which_material_changed_the_most", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=15952; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=16044; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=16140; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=16236; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=16336; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::material_stats::MaterialStats::which_material_changed_the_most; module=445, inline_record=16436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::material_stats::impl$1::default
// 信息 ID: 2b08bf55cf79221766a415b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state; module=445, inline_record=42992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
