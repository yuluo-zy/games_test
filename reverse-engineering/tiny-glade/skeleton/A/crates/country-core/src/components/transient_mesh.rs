// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/components/transient_mesh.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 87ee798dee589db6a7b7bbac14d647498fb87e62834d89fe77b6b3bf20ab135d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: transient_mesh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 469, 500, 1258, 1396, 1430
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1430,"record_offset":56648,"rva":17329584,"va":5386038704}
// 编译实例 RVAs: 0x1086db0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16365565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8034806ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5386039304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13789994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8d6942dff","size":5,"soft_fail":false,"target_va":5372249392,"va":5386039381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16940930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8827f0201","size":5,"soft_fail":false,"target_va":5402980128,"va":5386039193,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11470304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e820fa50ff","size":5,"soft_fail":false,"target_va":5374568688,"va":5386038987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::vao_library::AssetVAOLibrary::remove
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11379742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8e25b52ff","size":5,"soft_fail":false,"target_va":5374659264,"va":5386039001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8685198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e872797bff","size":5,"soft_fail":false,"target_va":5377353776,"va":5386038969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t16936197","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8056d0201","size":5,"soft_fail":false,"target_va":5402975328,"va":5386039126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16936395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5386038704,"raw_hex":"e8cb6d0201","size":5,"soft_fail":false,"target_va":5402975504,"va":5386039104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t16936465","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5386038704,"raw_hex":"e9116e0201","size":5,"soft_fail":false,"target_va":5402975696,"va":5386039226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1430,"record_offset":57872,"parent_record_offset":56648,"ancestor_record_offset":56648,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1430,"record_offset":57900,"parent_record_offset":56648,"ancestor_record_offset":56648,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1430,"record_offset":57428,"parent_record_offset":56648,"ancestor_record_offset":56648,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/backend/transient_mesh.rs
//     evidence={"module":1430,"record_offset":57472,"parent_record_offset":56648,"ancestor_record_offset":56648,"inlinee":66262,"source_line":16,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1430,"record_offset":56932,"parent_record_offset":56648,"ancestor_record_offset":56648,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1430,"record_offset":57780,"parent_record_offset":57756,"ancestor_record_offset":56648,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":1430,"record_offset":57756,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1430,"record_offset":57672,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1430,"record_offset":57700,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1430,"record_offset":57552,"parent_record_offset":57512,"ancestor_record_offset":56648,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1430,"record_offset":57060,"parent_record_offset":57000,"ancestor_record_offset":56648,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1430,"record_offset":57512,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":57348,"parent_record_offset":57320,"ancestor_record_offset":56648,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":58152,"parent_record_offset":57964,"ancestor_record_offset":56648,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":57000,"parent_record_offset":56968,"ancestor_record_offset":56648,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":58244,"parent_record_offset":58212,"ancestor_record_offset":56648,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":57252,"parent_record_offset":57220,"ancestor_record_offset":56648,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":57032,"parent_record_offset":57000,"ancestor_record_offset":56648,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":57276,"parent_record_offset":57252,"ancestor_record_offset":56648,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":58176,"parent_record_offset":58152,"ancestor_record_offset":56648,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1430,"record_offset":58268,"parent_record_offset":58244,"ancestor_record_offset":56648,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1430,"record_offset":57728,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1430,"record_offset":57588,"parent_record_offset":57472,"ancestor_record_offset":56648,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1430,"record_offset":57220,"parent_record_offset":57192,"ancestor_record_offset":56648,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1430,"record_offset":57192,"parent_record_offset":57164,"ancestor_record_offset":56648,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1430,"record_offset":57096,"parent_record_offset":56932,"ancestor_record_offset":56648,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1430,"record_offset":57928,"parent_record_offset":57900,"ancestor_record_offset":56648,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1430,"record_offset":57964,"parent_record_offset":57928,"ancestor_record_offset":56648,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1430,"record_offset":57320,"parent_record_offset":57124,"ancestor_record_offset":56648,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1430,"record_offset":57124,"parent_record_offset":57096,"ancestor_record_offset":56648,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1430,"record_offset":56968,"parent_record_offset":56932,"ancestor_record_offset":56648,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1430,"record_offset":58212,"parent_record_offset":57928,"ancestor_record_offset":56648,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1430,"record_offset":57164,"parent_record_offset":57124,"ancestor_record_offset":56648,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01430.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1258,"record_offset":25072,"rva":13535648,"va":5382244768}
// 编译实例 RVAs: 0xce89a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12571309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e8532d40ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5382245048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9995706","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e8467a67ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5382245093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10939803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e8651259ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5382244854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20735142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e8a6643c01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382244981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20730399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e81f523c01","size":5,"soft_fail":false,"target_va":5402975328,"va":5382244924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20730582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382244768,"raw_hex":"e8d6523c01","size":5,"soft_fail":false,"target_va":5402975504,"va":5382244917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t20730692","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5382244768,"raw_hex":"e944533c01","size":5,"soft_fail":false,"target_va":5402975696,"va":5382244999,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1258,"record_offset":25776,"parent_record_offset":25072,"ancestor_record_offset":25072,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1258,"record_offset":25900,"parent_record_offset":25072,"ancestor_record_offset":25072,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1258,"record_offset":25736,"parent_record_offset":25072,"ancestor_record_offset":25072,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1258,"record_offset":25256,"parent_record_offset":25072,"ancestor_record_offset":25072,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1258,"record_offset":25836,"parent_record_offset":25812,"ancestor_record_offset":25072,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1258,"record_offset":25812,"parent_record_offset":25776,"ancestor_record_offset":25072,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1258,"record_offset":25868,"parent_record_offset":25776,"ancestor_record_offset":25072,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1258,"record_offset":25376,"parent_record_offset":25316,"ancestor_record_offset":25072,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":25656,"parent_record_offset":25628,"ancestor_record_offset":25072,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":26136,"parent_record_offset":25956,"ancestor_record_offset":25072,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":25316,"parent_record_offset":25288,"ancestor_record_offset":25072,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":26228,"parent_record_offset":26196,"ancestor_record_offset":25072,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":25560,"parent_record_offset":25532,"ancestor_record_offset":25072,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":25348,"parent_record_offset":25316,"ancestor_record_offset":25072,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":25584,"parent_record_offset":25560,"ancestor_record_offset":25072,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":26160,"parent_record_offset":26136,"ancestor_record_offset":25072,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1258,"record_offset":26252,"parent_record_offset":26228,"ancestor_record_offset":25072,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1258,"record_offset":25532,"parent_record_offset":25504,"ancestor_record_offset":25072,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":1258,"record_offset":25504,"parent_record_offset":25476,"ancestor_record_offset":25072,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1258,"record_offset":25412,"parent_record_offset":25256,"ancestor_record_offset":25072,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":1258,"record_offset":25924,"parent_record_offset":25900,"ancestor_record_offset":25072,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1258,"record_offset":25956,"parent_record_offset":25924,"ancestor_record_offset":25072,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1258,"record_offset":25628,"parent_record_offset":25440,"ancestor_record_offset":25072,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":1258,"record_offset":25440,"parent_record_offset":25412,"ancestor_record_offset":25072,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1258,"record_offset":25288,"parent_record_offset":25256,"ancestor_record_offset":25072,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":1258,"record_offset":26196,"parent_record_offset":25924,"ancestor_record_offset":25072,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":1258,"record_offset":25476,"parent_record_offset":25440,"ancestor_record_offset":25072,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/01258.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::TransientMesh::new_with_registry
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 32, 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x684A
// 首条位置: {"module":469,"inlinee":26698}
// 编译实例 RVAs: 0xce8b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":469,"record_offset":41764,"parent_record_offset":41740,"ancestor_record_offset":41740,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":41792,"parent_record_offset":41764,"ancestor_record_offset":41740,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":469,"record_offset":41816,"parent_record_offset":41792,"ancestor_record_offset":41740,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00469.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::components::transient_mesh::impl$2::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":500,"record_offset":832588,"rva":7282432,"va":5375991552}
// 编译实例 RVAs: 0x6f1f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6318143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e8c1979fff","size":5,"soft_fail":false,"target_va":5369673744,"va":5375991882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: transient_mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3742540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e8b4e4c6ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5375991927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4686599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e8f97cb8ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5375991650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26988328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e828cf9b01","size":5,"soft_fail":false,"target_va":5402980128,"va":5375991795,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26983588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e8a4bc9b01","size":5,"soft_fail":false,"target_va":5402975328,"va":5375991735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26983780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375991552,"raw_hex":"e864bd9b01","size":5,"soft_fail":false,"target_va":5402975504,"va":5375991719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t26983876","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375991552,"raw_hex":"e9c4bd9b01","size":5,"soft_fail":false,"target_va":5402975696,"va":5375991815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":500,"record_offset":832744,"parent_record_offset":832588,"ancestor_record_offset":832588,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":833296,"parent_record_offset":832588,"ancestor_record_offset":832588,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":833420,"parent_record_offset":832588,"ancestor_record_offset":832588,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":500,"record_offset":833256,"parent_record_offset":832588,"ancestor_record_offset":832588,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":500,"record_offset":832772,"parent_record_offset":832588,"ancestor_record_offset":832588,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":500,"record_offset":833356,"parent_record_offset":833332,"ancestor_record_offset":832588,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":500,"record_offset":833332,"parent_record_offset":833296,"ancestor_record_offset":832588,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":500,"record_offset":833388,"parent_record_offset":833296,"ancestor_record_offset":832588,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":500,"record_offset":832896,"parent_record_offset":832836,"ancestor_record_offset":832588,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833176,"parent_record_offset":833148,"ancestor_record_offset":832588,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833660,"parent_record_offset":833476,"ancestor_record_offset":832588,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":832836,"parent_record_offset":832808,"ancestor_record_offset":832588,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833752,"parent_record_offset":833720,"ancestor_record_offset":832588,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833080,"parent_record_offset":833052,"ancestor_record_offset":832588,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":832868,"parent_record_offset":832836,"ancestor_record_offset":832588,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833104,"parent_record_offset":833080,"ancestor_record_offset":832588,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833684,"parent_record_offset":833660,"ancestor_record_offset":832588,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":500,"record_offset":833776,"parent_record_offset":833752,"ancestor_record_offset":832588,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":500,"record_offset":833052,"parent_record_offset":833024,"ancestor_record_offset":832588,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":500,"record_offset":833024,"parent_record_offset":832996,"ancestor_record_offset":832588,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":500,"record_offset":832932,"parent_record_offset":832772,"ancestor_record_offset":832588,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":500,"record_offset":833444,"parent_record_offset":833420,"ancestor_record_offset":832588,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":500,"record_offset":833476,"parent_record_offset":833444,"ancestor_record_offset":832588,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":500,"record_offset":833148,"parent_record_offset":832960,"ancestor_record_offset":832588,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":500,"record_offset":832960,"parent_record_offset":832932,"ancestor_record_offset":832588,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":500,"record_offset":832808,"parent_record_offset":832772,"ancestor_record_offset":832588,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":500,"record_offset":833720,"parent_record_offset":833444,"ancestor_record_offset":832588,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00500.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":500,"record_offset":832996,"parent_record_offset":832960,"ancestor_record_offset":832588,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00500.jsonl.gz"}
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
// 完整结构及原始类型记录: transient_mesh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::components::transient_mesh::TransientMesh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 29
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::system::commands::insert::closure_env$0", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "constructor": "bevy_ecs::system::commands::WithEntity", "arguments": ["tuple$<>", "bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::components::WoodenGate"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::system::commands::insert::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::system::commands::WithEntity", "arguments": ["tuple$<>", "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>", "constructor": "tuple$", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::resources::walls::plaster::Plaster", "country_core::components::drawable::DontCastShadow"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::system::commands::insert::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::system::commands::WithEntity", "arguments": ["tuple$<>", "bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > >", "constructor": "bevy_ecs::world::command_queue::impl$4::push::closure_env$0", "arguments": ["bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::EntityCommands::insert<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::system::commands::EntityCommands::insert", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::components::transient_mesh::TransientMesh,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh> >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::components::transient_mesh::TransientMesh", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<country_core::components::transient_mesh::TransientMesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$4::write_components::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>,bevy_ecs::archetype::AddBundle>", "constructor": "bevy_ecs::bundle::impl$4::write_components::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>", "bevy_ecs::archetype::AddBundle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::components::drawable::impl$3::get_components<bevy_ecs::bundle::impl$4::write_components::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>,bevy_ecs::archetype::AddBundle> >", "constructor": "country_core::components::drawable::impl$3::get_components", "arguments": ["bevy_ecs::bundle::impl$4::write_components::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>,bevy_ecs::archetype::AddBundle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::entity_ref::EntityWorldMut::insert<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::world::entity_ref::EntityWorldMut::insert", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::entity_ref::EntityWorldMut::insert<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::world::entity_ref::EntityWorldMut::insert", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::entity_ref::EntityWorldMut::insert<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::world::entity_ref::EntityWorldMut::insert", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::init_info<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::bundle::Bundles::init_info", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::init_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::Bundles::init_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::Bundles::init_info<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::bundle::Bundles::init_info", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >", "constructor": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$14::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::bundle::impl$14::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::components::WoodenGate", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >", "constructor": "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$16::component_ids<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impl$16::component_ids", "arguments": ["country_core::components::drawable::DrawableMeshBundle", "country_core::components::transient_mesh::TransientMesh", "country_core::resources::walls::plaster::Plaster", "country_core::components::drawable::DontCastShadow", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::Components::init_component<country_core::components::transient_mesh::TransientMesh>", "constructor": "bevy_ecs::component::Components::init_component", "arguments": ["country_core::components::transient_mesh::TransientMesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::components::WoodenGate,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::components::WoodenGate", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impl$0::component_ids<country_core::resources::walls::plaster::Plaster,bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > >", "constructor": "bevy_ecs::bundle::impl$0::component_ids", "arguments": ["country_core::resources::walls::plaster::Plaster", "bevy_ecs::bundle::impl$7::init_info::closure$0::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::components::transient_mesh::TransientMeshRegistry
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::components::transient_mesh::TransientMeshRegistry,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::components::transient_mesh::TransientMeshRegistry", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>", "constructor": "country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete", "arguments": ["country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: transient_mesh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: transient_mesh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 78; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>,bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>", "bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::history::replay_mode::RestoreAppTo>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::critters::sheep::CmdUnfreezeSheep>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >,bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::window_lights::WindowLight>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > >,enum2$<core::option::Option<ref$<country_core::components::drawable::AlbedoMap> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorWindowGlassInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window_lights::GlobalWindowLightsControl>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>,bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>,bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>,bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::history::History>", "bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>", "bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>", "bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>", "bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>", "bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::history::history_process::PendingHistoryCommands>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::HistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::history::history_process::DebugHistoryMsg>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::history::history_process::NewReplayCommandEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::timelapse_replay::InterruptTimelapse>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>,bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>,bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession> > >", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>", "bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>", "bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::NextSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::load_game::LoadGameManager>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::NewSessionStartedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::session::SessionLoadedCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >,bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>,bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::components::drawable::NaniActiveInstance>", "bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<utils::components::transform::Transform> >,bevy_ecs::query::filter::Without<country_core::components::drawable::NaniActiveInstance> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniInstanceDesc>,ref$<country_core::components::drawable::NaniActiveInstance> >,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniInstanceDesc> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::render::nani::systems::NaniActiveInstanceCollection>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMesh::new_with_registry
// 信息 ID: 4d3d47d719d00deeb9f9e2bb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::create_plaster @ 0x4ee596 -> 0xce8b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc348d -> 0xce8b00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui::signifiers::SignifierEntityBuilder::build; module=469, inline_record=41740; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::ui::debug_visualize_volumes::debug_visualize_roof_uvs; module=1396, inline_record=20676; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMeshRegistry::deferred_delete_mesh
// 信息 ID: 3350874ae855e5468b652883
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_roof::visual::switch_roof_type::switch_roof_type @ 0x198a09d -> 0xce89a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete<country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0>
// 信息 ID: 847f33ef17bbad0f1b4f56c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 156db0489f893be77b36dd47: void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)
// [类型关系] country_core::components::transient_mesh::TransientMeshRegistry::drain_meshes_to_delete -> ["country_core::systems::backend::transient_mesh::delete_dropped_transient_meshes::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)> > @ 0xe54b35 -> 0x1086db0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>),void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>)> @ 0x1005005 -> 0x1086db0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::components::transient_mesh::impl$2::drop
// 信息 ID: 381e04eeba75b52c7c0ba940
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > > | core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh> | core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > > | core::ptr::drop_in_place<country_core::components::transient_mesh::TransientMesh> | 其余 2 个同地址别名见共享索引 @ 0x6df9d6 -> 0x6f1f00 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > > > | core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > | core::ptr::drop_in_place<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::components::WoodenGate> > @ 0xc97b76 -> 0x6f1f00 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<tuple$<country_core::components::drawable::DrawableMeshBundle,country_core::components::transient_mesh::TransientMesh,country_core::resources::walls::plaster::Plaster,country_core::components::drawable::DontCastShadow> > > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > > @ 0xcbfc5d -> 0x6f1f00 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$4::push::closure_env$0<bevy_ecs::system::commands::WithEntity<tuple$<>,bevy_ecs::system::commands::insert::closure_env$0<country_core::components::transient_mesh::TransientMesh> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > > @ 0xedfe70 -> 0x6f1f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
