// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/backend/flush_screen_log.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3b2907a6ead8be0ec94d56244a3426a44e8d2c285161bfee4e0fe01a757136c9
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: flush_screen_log.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 488
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
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
// 首条位置: {"module":488,"record_offset":41112,"rva":6972992,"va":5375682112}
// 编译实例 RVAs: 0x6a6640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6009059","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e81d4fa4ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5375682798,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3433474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e8fe9bcbff","size":5,"soft_fail":false,"target_va":5372249392,"va":5375682861,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2932707","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e81d40d3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375682414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406dcac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t221660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e8dc610300","size":5,"soft_fail":false,"target_va":5375904448,"va":5375682783,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27297465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e8b986a001","size":5,"soft_fail":false,"target_va":5402980128,"va":5375682658,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::DebugGui::log_to_screen
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-1282651","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e8a56decff","size":5,"soft_fail":false,"target_va":5374399744,"va":5375682390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27292745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e84974a001","size":5,"soft_fail":false,"target_va":5402975328,"va":5375682578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27292937","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682112,"raw_hex":"e80975a001","size":5,"soft_fail":false,"target_va":5402975504,"va":5375682562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t27293003","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375682112,"raw_hex":"e94b75a001","size":5,"soft_fail":false,"target_va":5402975696,"va":5375682688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":41840,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":5990,"source_line":2129,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":488,"record_offset":42100,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":488,"record_offset":42248,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":488,"record_offset":42072,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":42304,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":41796,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":41296,"parent_record_offset":41112,"ancestor_record_offset":41112,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":42456,"parent_record_offset":42432,"ancestor_record_offset":41112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":42432,"parent_record_offset":42376,"ancestor_record_offset":41112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":42404,"parent_record_offset":42376,"ancestor_record_offset":41112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":41952,"parent_record_offset":41928,"ancestor_record_offset":41112,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":42376,"parent_record_offset":42352,"ancestor_record_offset":41112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":41928,"parent_record_offset":41840,"ancestor_record_offset":41112,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":41872,"parent_record_offset":41840,"ancestor_record_offset":41112,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":41900,"parent_record_offset":41840,"ancestor_record_offset":41112,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":488,"record_offset":42528,"parent_record_offset":42504,"ancestor_record_offset":41112,"inlinee":5995,"source_line":175,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":488,"record_offset":42272,"parent_record_offset":42248,"ancestor_record_offset":41112,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":488,"record_offset":42212,"parent_record_offset":42100,"ancestor_record_offset":41112,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":42328,"parent_record_offset":42304,"ancestor_record_offset":41112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":488,"record_offset":42032,"parent_record_offset":42008,"ancestor_record_offset":41112,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":488,"record_offset":42176,"parent_record_offset":42132,"ancestor_record_offset":41112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":41428,"parent_record_offset":41368,"ancestor_record_offset":41112,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":488,"record_offset":41984,"parent_record_offset":41840,"ancestor_record_offset":41112,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":488,"record_offset":42008,"parent_record_offset":41984,"ancestor_record_offset":41112,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":488,"record_offset":42132,"parent_record_offset":42100,"ancestor_record_offset":41112,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":41716,"parent_record_offset":41688,"ancestor_record_offset":41112,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43088,"parent_record_offset":42900,"ancestor_record_offset":41112,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":41368,"parent_record_offset":41336,"ancestor_record_offset":41112,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43180,"parent_record_offset":43148,"ancestor_record_offset":41112,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":41620,"parent_record_offset":41588,"ancestor_record_offset":41112,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":41400,"parent_record_offset":41368,"ancestor_record_offset":41112,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":41644,"parent_record_offset":41620,"ancestor_record_offset":41112,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43112,"parent_record_offset":43088,"ancestor_record_offset":41112,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43204,"parent_record_offset":43180,"ancestor_record_offset":41112,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":41588,"parent_record_offset":41560,"ancestor_record_offset":41112,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":41560,"parent_record_offset":41532,"ancestor_record_offset":41112,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":41464,"parent_record_offset":41296,"ancestor_record_offset":41112,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":42864,"parent_record_offset":42836,"ancestor_record_offset":41112,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":42900,"parent_record_offset":42864,"ancestor_record_offset":41112,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":41688,"parent_record_offset":41492,"ancestor_record_offset":41112,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":41492,"parent_record_offset":41464,"ancestor_record_offset":41112,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":41336,"parent_record_offset":41296,"ancestor_record_offset":41112,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":43148,"parent_record_offset":42864,"ancestor_record_offset":41112,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":488,"record_offset":41532,"parent_record_offset":41492,"ancestor_record_offset":41112,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00488.jsonl.gz"}
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
// 首条位置: {"module":488,"record_offset":43452,"rva":6973776,"va":5375682896}
// 编译实例 RVAs: 0x6a6950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6009668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8bc4ca4ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5375683407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3434142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e86299cbff","size":5,"soft_fail":false,"target_va":5372249392,"va":5375683529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2933320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8b83dd3ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375683011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2933775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8f13bd3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375683482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27910310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8a6e0a901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375683045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27294775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8377ca001","size":5,"soft_fail":false,"target_va":5402978144,"va":5375683364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4377858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8fe32bdff","size":5,"soft_fail":false,"target_va":5371305232,"va":5375683085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1843770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e83a221c00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375683345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27296838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e84684a001","size":5,"soft_fail":false,"target_va":5402980128,"va":5375683285,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27292115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e8d371a001","size":5,"soft_fail":false,"target_va":5402975328,"va":5375683208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27292301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375682896,"raw_hex":"e88d72a001","size":5,"soft_fail":false,"target_va":5402975504,"va":5375683198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t27292380","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375682896,"raw_hex":"e9dc72a001","size":5,"soft_fail":false,"target_va":5402975696,"va":5375683311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":488,"record_offset":44164,"parent_record_offset":43452,"ancestor_record_offset":43452,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":44744,"parent_record_offset":43452,"ancestor_record_offset":43452,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":45116,"parent_record_offset":43452,"ancestor_record_offset":43452,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":44120,"parent_record_offset":43452,"ancestor_record_offset":43452,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":43624,"parent_record_offset":43452,"ancestor_record_offset":43452,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":44528,"parent_record_offset":44504,"ancestor_record_offset":43452,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":44552,"parent_record_offset":44528,"ancestor_record_offset":43452,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":45060,"parent_record_offset":45036,"ancestor_record_offset":43452,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":44504,"parent_record_offset":44460,"ancestor_record_offset":43452,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":45036,"parent_record_offset":44980,"ancestor_record_offset":43452,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":45008,"parent_record_offset":44980,"ancestor_record_offset":43452,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":44824,"parent_record_offset":44792,"ancestor_record_offset":43452,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":44460,"parent_record_offset":44428,"ancestor_record_offset":43452,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":44428,"parent_record_offset":44396,"ancestor_record_offset":43452,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":44620,"parent_record_offset":44428,"ancestor_record_offset":43452,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":44980,"parent_record_offset":44956,"ancestor_record_offset":43452,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":44356,"parent_record_offset":44324,"ancestor_record_offset":43452,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":44324,"parent_record_offset":44292,"ancestor_record_offset":43452,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":44260,"parent_record_offset":44228,"ancestor_record_offset":43452,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":44292,"parent_record_offset":44260,"ancestor_record_offset":43452,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":44228,"parent_record_offset":44196,"ancestor_record_offset":43452,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":488,"record_offset":44196,"parent_record_offset":44164,"ancestor_record_offset":43452,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":44792,"parent_record_offset":44744,"ancestor_record_offset":43452,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":44396,"parent_record_offset":44356,"ancestor_record_offset":43452,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":488,"record_offset":44684,"parent_record_offset":44660,"ancestor_record_offset":43452,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":488,"record_offset":44660,"parent_record_offset":44356,"ancestor_record_offset":43452,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":44884,"parent_record_offset":44744,"ancestor_record_offset":43452,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":44576,"parent_record_offset":44552,"ancestor_record_offset":43452,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":44856,"parent_record_offset":44744,"ancestor_record_offset":43452,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":43756,"parent_record_offset":43696,"ancestor_record_offset":43452,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":44040,"parent_record_offset":44012,"ancestor_record_offset":43452,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45368,"parent_record_offset":45180,"ancestor_record_offset":43452,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43696,"parent_record_offset":43664,"ancestor_record_offset":43452,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45460,"parent_record_offset":45428,"ancestor_record_offset":43452,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43944,"parent_record_offset":43912,"ancestor_record_offset":43452,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43728,"parent_record_offset":43696,"ancestor_record_offset":43452,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":43968,"parent_record_offset":43944,"ancestor_record_offset":43452,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45392,"parent_record_offset":45368,"ancestor_record_offset":43452,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":43452,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":43912,"parent_record_offset":43884,"ancestor_record_offset":43452,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":43884,"parent_record_offset":43856,"ancestor_record_offset":43452,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":43792,"parent_record_offset":43624,"ancestor_record_offset":43452,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":45144,"parent_record_offset":45116,"ancestor_record_offset":43452,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":45180,"parent_record_offset":45144,"ancestor_record_offset":43452,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":44012,"parent_record_offset":43820,"ancestor_record_offset":43452,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":43820,"parent_record_offset":43792,"ancestor_record_offset":43452,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":43664,"parent_record_offset":43624,"ancestor_record_offset":43452,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":45428,"parent_record_offset":45144,"ancestor_record_offset":43452,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":488,"record_offset":43856,"parent_record_offset":43820,"ancestor_record_offset":43452,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00488.jsonl.gz"}
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
// 首条位置: {"module":488,"record_offset":45532,"rva":6974432,"va":5375683552}
// 编译实例 RVAs: 0x6a6be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6010324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e82c4aa4ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5375684063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3434798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8d296cbff","size":5,"soft_fail":false,"target_va":5372249392,"va":5375684185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2933976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8283bd3ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375683667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2934431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e86139d3ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375684138,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: flush_screen_log.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27909654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e816dea901","size":5,"soft_fail":false,"target_va":5403593360,"va":5375683701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27294119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8a779a001","size":5,"soft_fail":false,"target_va":5402978144,"va":5375684020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4378514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e86e30bdff","size":5,"soft_fail":false,"target_va":5371305232,"va":5375683741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1843114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8aa1f1c00","size":5,"soft_fail":false,"target_va":5377527120,"va":5375684001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27296182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8b681a001","size":5,"soft_fail":false,"target_va":5402980128,"va":5375683941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27291459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8436fa001","size":5,"soft_fail":false,"target_va":5402975328,"va":5375683864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::lock_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27291645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375683552,"raw_hex":"e8fd6fa001","size":5,"soft_fail":false,"target_va":5402975504,"va":5375683854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t27291724","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375683552,"raw_hex":"e94c70a001","size":5,"soft_fail":false,"target_va":5402975696,"va":5375683967,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":488,"record_offset":46244,"parent_record_offset":45532,"ancestor_record_offset":45532,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":46824,"parent_record_offset":45532,"ancestor_record_offset":45532,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":47196,"parent_record_offset":45532,"ancestor_record_offset":45532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":46200,"parent_record_offset":45532,"ancestor_record_offset":45532,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":45704,"parent_record_offset":45532,"ancestor_record_offset":45532,"inlinee":8235,"source_line":315,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":46608,"parent_record_offset":46584,"ancestor_record_offset":45532,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":46632,"parent_record_offset":46608,"ancestor_record_offset":45532,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":47140,"parent_record_offset":47116,"ancestor_record_offset":45532,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":46584,"parent_record_offset":46540,"ancestor_record_offset":45532,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":488,"record_offset":47116,"parent_record_offset":47060,"ancestor_record_offset":45532,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":47088,"parent_record_offset":47060,"ancestor_record_offset":45532,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":46904,"parent_record_offset":46872,"ancestor_record_offset":45532,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":46540,"parent_record_offset":46508,"ancestor_record_offset":45532,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":46508,"parent_record_offset":46476,"ancestor_record_offset":45532,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":46700,"parent_record_offset":46508,"ancestor_record_offset":45532,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":488,"record_offset":47060,"parent_record_offset":47036,"ancestor_record_offset":45532,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":46436,"parent_record_offset":46404,"ancestor_record_offset":45532,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":46404,"parent_record_offset":46372,"ancestor_record_offset":45532,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":46340,"parent_record_offset":46308,"ancestor_record_offset":45532,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":46372,"parent_record_offset":46340,"ancestor_record_offset":45532,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":488,"record_offset":46308,"parent_record_offset":46276,"ancestor_record_offset":45532,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":488,"record_offset":46276,"parent_record_offset":46244,"ancestor_record_offset":45532,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":46872,"parent_record_offset":46824,"ancestor_record_offset":45532,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":488,"record_offset":46476,"parent_record_offset":46436,"ancestor_record_offset":45532,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":488,"record_offset":46764,"parent_record_offset":46740,"ancestor_record_offset":45532,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":488,"record_offset":46740,"parent_record_offset":46436,"ancestor_record_offset":45532,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":46964,"parent_record_offset":46824,"ancestor_record_offset":45532,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":46656,"parent_record_offset":46632,"ancestor_record_offset":45532,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":488,"record_offset":46936,"parent_record_offset":46824,"ancestor_record_offset":45532,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":488,"record_offset":45836,"parent_record_offset":45776,"ancestor_record_offset":45532,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":46120,"parent_record_offset":46092,"ancestor_record_offset":45532,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":47448,"parent_record_offset":47260,"ancestor_record_offset":45532,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45776,"parent_record_offset":45744,"ancestor_record_offset":45532,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":47540,"parent_record_offset":47508,"ancestor_record_offset":45532,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":46024,"parent_record_offset":45992,"ancestor_record_offset":45532,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":45808,"parent_record_offset":45776,"ancestor_record_offset":45532,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":46048,"parent_record_offset":46024,"ancestor_record_offset":45532,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":47472,"parent_record_offset":47448,"ancestor_record_offset":45532,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":488,"record_offset":47564,"parent_record_offset":47540,"ancestor_record_offset":45532,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":45992,"parent_record_offset":45964,"ancestor_record_offset":45532,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":488,"record_offset":45964,"parent_record_offset":45936,"ancestor_record_offset":45532,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::MutexGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":45872,"parent_record_offset":45704,"ancestor_record_offset":45532,"inlinee":8239,"source_line":524,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":488,"record_offset":47224,"parent_record_offset":47196,"ancestor_record_offset":45532,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":47260,"parent_record_offset":47224,"ancestor_record_offset":45532,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":46092,"parent_record_offset":45900,"ancestor_record_offset":45532,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":488,"record_offset":45900,"parent_record_offset":45872,"ancestor_record_offset":45532,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::lock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":45744,"parent_record_offset":45704,"ancestor_record_offset":45532,"inlinee":8237,"source_line":38,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":488,"record_offset":47508,"parent_record_offset":47224,"ancestor_record_offset":45532,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00488.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":488,"record_offset":45936,"parent_record_offset":45900,"ancestor_record_offset":45532,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00488.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "std::sync::mutex::MutexGuard", "arguments": ["alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::mutex::MutexGuard<alloc::vec::Vec<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<tuple$<alloc::string::String,country_core::systems::backend::flush_screen_log::LogType>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::flush_error_log_to_screen
// 信息 ID: 28a1f67422c3076f87a97f05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 86d2c9c7ede139c54c7d002b: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> > @ 0xc9087 -> 0x6a6640 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)> @ 0x2d1ec1 -> 0x6a6640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_error_to_screen
// 信息 ID: d9bedbc9cff1f785716ca532
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x4f1ec5 -> 0x6a6950 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::wall::draw_wall::draw_wall @ 0x5118ad -> 0x6a6950 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::save::load_game::load_game @ 0x17f892c -> 0x6a6950 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::backend::flush_screen_log::log_warning_to_screen
// 信息 ID: 94908e5a69a3b92a6aab6d22
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain @ 0xf28a18 -> 0x6a6be0 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
