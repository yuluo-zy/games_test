// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/frozen/src/lib.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: c82a983d100bb0675686912229ebd4da7e2faa3dba80f3ebf777e36221e6cecd
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: lib.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 219, 419, 475, 480, 489, 744, 745, 1211, 1212, 1304, 1385, 3212, 3421, 3531, 3603, 3697, 3840, 4208, 4219, 4250
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":26124,"rva":6719952,"va":5375429072}
// 编译实例 RVAs: 0x6689d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5815691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e87542a7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375429206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3157346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e89ed2cfff","size":5,"soft_fail":false,"target_va":5372272592,"va":5375429933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2679766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e82a1cd7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375429473,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140666f10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e847e1ffff","size":5,"soft_fail":false,"target_va":5375422224,"va":5375430084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t506654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e81ebb0700","size":5,"soft_fail":false,"target_va":5375936112,"va":5375429453,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d8d30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27722023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e82701a701","size":5,"soft_fail":false,"target_va":5403151664,"va":5375429636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27550564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e86463a401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375429559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27546161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e83152a401","size":5,"soft_fail":false,"target_va":5402975328,"va":5375429162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27548343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e8b75aa401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375429444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27548352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375429072,"raw_hex":"e8c05aa401","size":5,"soft_fail":false,"target_va":5402977488,"va":5375429131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":27984,"parent_record_offset":26124,"ancestor_record_offset":26124,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":26892,"parent_record_offset":26124,"ancestor_record_offset":26124,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":26932,"parent_record_offset":26124,"ancestor_record_offset":26124,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":28580,"parent_record_offset":26124,"ancestor_record_offset":26124,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":26428,"parent_record_offset":26124,"ancestor_record_offset":26124,"inlinee":6693,"source_line":359,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":28456,"parent_record_offset":28432,"ancestor_record_offset":26124,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":28432,"parent_record_offset":28408,"ancestor_record_offset":26124,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":475,"record_offset":28408,"parent_record_offset":28380,"ancestor_record_offset":26124,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":29156,"parent_record_offset":29132,"ancestor_record_offset":26124,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":27184,"parent_record_offset":27160,"ancestor_record_offset":26124,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":27268,"parent_record_offset":27244,"ancestor_record_offset":26124,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":27208,"parent_record_offset":27184,"ancestor_record_offset":26124,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":29388,"parent_record_offset":29360,"ancestor_record_offset":26124,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":27380,"parent_record_offset":27356,"ancestor_record_offset":26124,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":27544,"parent_record_offset":27520,"ancestor_record_offset":26124,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":27520,"parent_record_offset":27496,"ancestor_record_offset":26124,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":29600,"parent_record_offset":29572,"ancestor_record_offset":26124,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":29628,"parent_record_offset":29600,"ancestor_record_offset":26124,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":26544,"parent_record_offset":26484,"ancestor_record_offset":26124,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":26812,"parent_record_offset":26784,"ancestor_record_offset":26124,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":28220,"parent_record_offset":28044,"ancestor_record_offset":26124,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":26484,"parent_record_offset":26460,"ancestor_record_offset":26124,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":28312,"parent_record_offset":28280,"ancestor_record_offset":26124,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":26716,"parent_record_offset":26688,"ancestor_record_offset":26124,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":26516,"parent_record_offset":26484,"ancestor_record_offset":26124,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":26740,"parent_record_offset":26716,"ancestor_record_offset":26124,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":28244,"parent_record_offset":28220,"ancestor_record_offset":26124,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":28336,"parent_record_offset":28312,"ancestor_record_offset":26124,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":27632,"parent_record_offset":27608,"ancestor_record_offset":26124,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":27608,"parent_record_offset":27584,"ancestor_record_offset":26124,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":27916,"parent_record_offset":26960,"ancestor_record_offset":26124,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":26960,"parent_record_offset":26932,"ancestor_record_offset":26124,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":27000,"parent_record_offset":26960,"ancestor_record_offset":26124,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27496,"parent_record_offset":27472,"ancestor_record_offset":26124,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27472,"parent_record_offset":27448,"ancestor_record_offset":26124,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27824,"parent_record_offset":27800,"ancestor_record_offset":26124,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29572,"parent_record_offset":28628,"ancestor_record_offset":26124,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27764,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27800,"parent_record_offset":27028,"ancestor_record_offset":26124,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27028,"parent_record_offset":26960,"ancestor_record_offset":26124,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":28628,"parent_record_offset":28604,"ancestor_record_offset":26124,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27940,"parent_record_offset":27916,"ancestor_record_offset":26124,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29432,"parent_record_offset":28628,"ancestor_record_offset":26124,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27056,"parent_record_offset":27028,"ancestor_record_offset":26124,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":28700,"parent_record_offset":28668,"ancestor_record_offset":26124,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":28880,"parent_record_offset":28700,"ancestor_record_offset":26124,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29044,"parent_record_offset":28700,"ancestor_record_offset":26124,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29076,"parent_record_offset":29044,"ancestor_record_offset":26124,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":28668,"parent_record_offset":28628,"ancestor_record_offset":26124,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27132,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29360,"parent_record_offset":29336,"ancestor_record_offset":26124,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29336,"parent_record_offset":28668,"ancestor_record_offset":26124,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":27328,"parent_record_offset":27300,"ancestor_record_offset":26124,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":27356,"parent_record_offset":27328,"ancestor_record_offset":26124,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":27416,"parent_record_offset":27300,"ancestor_record_offset":26124,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":27300,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27104,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":27448,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29100,"parent_record_offset":29076,"ancestor_record_offset":26124,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":29544,"parent_record_offset":28628,"ancestor_record_offset":26124,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":27160,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":29132,"parent_record_offset":29044,"ancestor_record_offset":26124,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":27244,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":27672,"parent_record_offset":27056,"ancestor_record_offset":26124,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":28920,"parent_record_offset":28880,"ancestor_record_offset":26124,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":28604,"parent_record_offset":28580,"ancestor_record_offset":26124,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":27584,"parent_record_offset":27448,"ancestor_record_offset":26124,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":26688,"parent_record_offset":26664,"ancestor_record_offset":26124,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":26664,"parent_record_offset":26640,"ancestor_record_offset":26124,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":28044,"parent_record_offset":28012,"ancestor_record_offset":26124,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":26784,"parent_record_offset":26608,"ancestor_record_offset":26124,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":26608,"parent_record_offset":26580,"ancestor_record_offset":26124,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":26580,"parent_record_offset":26428,"ancestor_record_offset":26124,"inlinee":6696,"source_line":590,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":28012,"parent_record_offset":27984,"ancestor_record_offset":26124,"inlinee":6705,"source_line":722,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":26460,"parent_record_offset":26428,"ancestor_record_offset":26124,"inlinee":6694,"source_line":152,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":28280,"parent_record_offset":28012,"ancestor_record_offset":26124,"inlinee":6707,"source_line":159,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":475,"record_offset":26640,"parent_record_offset":26608,"ancestor_record_offset":26124,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":22080,"rva":6718848,"va":5375427968}
// 编译实例 RVAs: 0x668580
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5814587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e8c546a7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375428102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3156258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e8ded6cfff","size":5,"soft_fail":false,"target_va":5372272592,"va":5375428845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3017317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e89bf5d1ff","size":5,"soft_fail":false,"target_va":5372411040,"va":5375428352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2678674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e86e20d7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375428381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406670c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e837e7ffff","size":5,"soft_fail":false,"target_va":5375422656,"va":5375428996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e7310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t519170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e802ec0700","size":5,"soft_fail":false,"target_va":5375947536,"va":5375428361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420d9a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27726411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e84b12a701","size":5,"soft_fail":false,"target_va":5403154960,"va":5375428544,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27551656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e8a867a401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375428467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27547265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e88156a401","size":5,"soft_fail":false,"target_va":5402975328,"va":5375428058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27549447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e8075fa401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375428340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27549456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375427968,"raw_hex":"e8105fa401","size":5,"soft_fail":false,"target_va":5402977488,"va":5375428027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":23976,"parent_record_offset":22080,"ancestor_record_offset":22080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":22884,"parent_record_offset":22080,"ancestor_record_offset":22080,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":22924,"parent_record_offset":22080,"ancestor_record_offset":22080,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":24628,"parent_record_offset":22080,"ancestor_record_offset":22080,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":22420,"parent_record_offset":22080,"ancestor_record_offset":22080,"inlinee":6693,"source_line":359,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":24504,"parent_record_offset":24480,"ancestor_record_offset":22080,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":24480,"parent_record_offset":24456,"ancestor_record_offset":22080,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":475,"record_offset":24456,"parent_record_offset":24372,"ancestor_record_offset":22080,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":25204,"parent_record_offset":25180,"ancestor_record_offset":22080,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":23176,"parent_record_offset":23152,"ancestor_record_offset":22080,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":23260,"parent_record_offset":23236,"ancestor_record_offset":22080,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":23200,"parent_record_offset":23176,"ancestor_record_offset":22080,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":25436,"parent_record_offset":25408,"ancestor_record_offset":22080,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":23372,"parent_record_offset":23348,"ancestor_record_offset":22080,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":24400,"parent_record_offset":24372,"ancestor_record_offset":22080,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":23536,"parent_record_offset":23512,"ancestor_record_offset":22080,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":23512,"parent_record_offset":23488,"ancestor_record_offset":22080,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":25648,"parent_record_offset":25620,"ancestor_record_offset":22080,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":25676,"parent_record_offset":25648,"ancestor_record_offset":22080,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":22536,"parent_record_offset":22476,"ancestor_record_offset":22080,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":22804,"parent_record_offset":22776,"ancestor_record_offset":22080,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":24212,"parent_record_offset":24036,"ancestor_record_offset":22080,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":22476,"parent_record_offset":22452,"ancestor_record_offset":22080,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":24304,"parent_record_offset":24272,"ancestor_record_offset":22080,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":22708,"parent_record_offset":22680,"ancestor_record_offset":22080,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":22508,"parent_record_offset":22476,"ancestor_record_offset":22080,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":22732,"parent_record_offset":22708,"ancestor_record_offset":22080,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":24236,"parent_record_offset":24212,"ancestor_record_offset":22080,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":24328,"parent_record_offset":24304,"ancestor_record_offset":22080,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":23624,"parent_record_offset":23600,"ancestor_record_offset":22080,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":23600,"parent_record_offset":23576,"ancestor_record_offset":22080,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":23908,"parent_record_offset":22952,"ancestor_record_offset":22080,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":22952,"parent_record_offset":22924,"ancestor_record_offset":22080,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":22992,"parent_record_offset":22952,"ancestor_record_offset":22080,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23488,"parent_record_offset":23464,"ancestor_record_offset":22080,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23464,"parent_record_offset":23440,"ancestor_record_offset":22080,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23816,"parent_record_offset":23792,"ancestor_record_offset":22080,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25620,"parent_record_offset":24676,"ancestor_record_offset":22080,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23756,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23792,"parent_record_offset":23020,"ancestor_record_offset":22080,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23020,"parent_record_offset":22952,"ancestor_record_offset":22080,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":24676,"parent_record_offset":24652,"ancestor_record_offset":22080,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23932,"parent_record_offset":23908,"ancestor_record_offset":22080,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25480,"parent_record_offset":24676,"ancestor_record_offset":22080,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23048,"parent_record_offset":23020,"ancestor_record_offset":22080,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":24748,"parent_record_offset":24716,"ancestor_record_offset":22080,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":24928,"parent_record_offset":24748,"ancestor_record_offset":22080,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25092,"parent_record_offset":24748,"ancestor_record_offset":22080,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25124,"parent_record_offset":25092,"ancestor_record_offset":22080,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":24716,"parent_record_offset":24676,"ancestor_record_offset":22080,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23124,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25408,"parent_record_offset":25384,"ancestor_record_offset":22080,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25384,"parent_record_offset":24716,"ancestor_record_offset":22080,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":23320,"parent_record_offset":23292,"ancestor_record_offset":22080,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":23348,"parent_record_offset":23320,"ancestor_record_offset":22080,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":23408,"parent_record_offset":23292,"ancestor_record_offset":22080,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":23292,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23096,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":23440,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25148,"parent_record_offset":25124,"ancestor_record_offset":22080,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":25592,"parent_record_offset":24676,"ancestor_record_offset":22080,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":23152,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":25180,"parent_record_offset":25092,"ancestor_record_offset":22080,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":23236,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":23664,"parent_record_offset":23048,"ancestor_record_offset":22080,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":24968,"parent_record_offset":24928,"ancestor_record_offset":22080,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":24652,"parent_record_offset":24628,"ancestor_record_offset":22080,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":23576,"parent_record_offset":23440,"ancestor_record_offset":22080,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":22080,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":22080,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":24036,"parent_record_offset":24004,"ancestor_record_offset":22080,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":22776,"parent_record_offset":22600,"ancestor_record_offset":22080,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":22600,"parent_record_offset":22572,"ancestor_record_offset":22080,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":22572,"parent_record_offset":22420,"ancestor_record_offset":22080,"inlinee":6696,"source_line":590,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":24004,"parent_record_offset":23976,"ancestor_record_offset":22080,"inlinee":6705,"source_line":722,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":22452,"parent_record_offset":22420,"ancestor_record_offset":22080,"inlinee":6694,"source_line":152,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":24272,"parent_record_offset":24004,"ancestor_record_offset":22080,"inlinee":6707,"source_line":159,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":475,"record_offset":22632,"parent_record_offset":22600,"ancestor_record_offset":22080,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 记录形式: compiled_procedure; instances=1
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":17564,"rva":6717488,"va":5375426608}
// 编译实例 RVAs: 0x668030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5813227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8154ca7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375426742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140365fd0
//     owner=unresolved_namespace; count=2
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3154994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8cedbcfff","size":5,"soft_fail":false,"target_va":5372272592,"va":5375427581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2677411","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e85d25d7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375427118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140666f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e897eaffff","size":5,"soft_fail":false,"target_va":5375422336,"va":5375427812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8c80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t527051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8cb0a0800","size":5,"soft_fail":false,"target_va":5375954048,"va":5375426992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e9a10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t529839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8af150800","size":5,"soft_fail":false,"target_va":5375957520,"va":5375427676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df04d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7897827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8e3827800","size":5,"soft_fail":false,"target_va":5383324880,"va":5375427048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8620179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e893888300","size":5,"soft_fail":false,"target_va":5384047280,"va":5375427096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cd7d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27677946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8fa54a601","size":5,"soft_fail":false,"target_va":5403105232,"va":5375427281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27552919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8976ca401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375427204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27548625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e8d15ba401","size":5,"soft_fail":false,"target_va":5402975328,"va":5375426698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27550807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e85764a401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375426980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27550816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375426608,"raw_hex":"e86064a401","size":5,"soft_fail":false,"target_va":5402977488,"va":5375426667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":19456,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":18364,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":18404,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":20588,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":17900,"parent_record_offset":17564,"ancestor_record_offset":17564,"inlinee":6693,"source_line":359,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":20464,"parent_record_offset":20440,"ancestor_record_offset":17564,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":20440,"parent_record_offset":20416,"ancestor_record_offset":17564,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":475,"record_offset":20416,"parent_record_offset":19852,"ancestor_record_offset":17564,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":475,"record_offset":20160,"parent_record_offset":20124,"ancestor_record_offset":17564,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":475,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":17564,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":21160,"parent_record_offset":21136,"ancestor_record_offset":17564,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":18656,"parent_record_offset":18632,"ancestor_record_offset":17564,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":18740,"parent_record_offset":18716,"ancestor_record_offset":17564,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":17564,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":21392,"parent_record_offset":21364,"ancestor_record_offset":17564,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":18852,"parent_record_offset":18828,"ancestor_record_offset":17564,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":19888,"parent_record_offset":19852,"ancestor_record_offset":17564,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":19016,"parent_record_offset":18992,"ancestor_record_offset":17564,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":18992,"parent_record_offset":18968,"ancestor_record_offset":17564,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":21604,"parent_record_offset":21576,"ancestor_record_offset":17564,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":475,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":17564,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":21632,"parent_record_offset":21604,"ancestor_record_offset":17564,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":18016,"parent_record_offset":17956,"ancestor_record_offset":17564,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":18284,"parent_record_offset":18256,"ancestor_record_offset":17564,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":19692,"parent_record_offset":19516,"ancestor_record_offset":17564,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":17956,"parent_record_offset":17932,"ancestor_record_offset":17564,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":19784,"parent_record_offset":19752,"ancestor_record_offset":17564,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":18188,"parent_record_offset":18160,"ancestor_record_offset":17564,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":17988,"parent_record_offset":17956,"ancestor_record_offset":17564,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":18212,"parent_record_offset":18188,"ancestor_record_offset":17564,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":17564,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":17564,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":20252,"parent_record_offset":20124,"ancestor_record_offset":17564,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":19104,"parent_record_offset":19080,"ancestor_record_offset":17564,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":19080,"parent_record_offset":19056,"ancestor_record_offset":17564,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":19388,"parent_record_offset":18432,"ancestor_record_offset":17564,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":18432,"parent_record_offset":18404,"ancestor_record_offset":17564,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":18472,"parent_record_offset":18432,"ancestor_record_offset":17564,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18968,"parent_record_offset":18944,"ancestor_record_offset":17564,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18944,"parent_record_offset":18920,"ancestor_record_offset":17564,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":19296,"parent_record_offset":19272,"ancestor_record_offset":17564,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21576,"parent_record_offset":20636,"ancestor_record_offset":17564,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":19236,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":19272,"parent_record_offset":18500,"ancestor_record_offset":17564,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18500,"parent_record_offset":18432,"ancestor_record_offset":17564,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":20636,"parent_record_offset":20612,"ancestor_record_offset":17564,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":19412,"parent_record_offset":19388,"ancestor_record_offset":17564,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21436,"parent_record_offset":20636,"ancestor_record_offset":17564,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18528,"parent_record_offset":18500,"ancestor_record_offset":17564,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":20708,"parent_record_offset":20676,"ancestor_record_offset":17564,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":20884,"parent_record_offset":20708,"ancestor_record_offset":17564,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21048,"parent_record_offset":20708,"ancestor_record_offset":17564,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21080,"parent_record_offset":21048,"ancestor_record_offset":17564,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":20676,"parent_record_offset":20636,"ancestor_record_offset":17564,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18604,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21364,"parent_record_offset":21340,"ancestor_record_offset":17564,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21340,"parent_record_offset":20676,"ancestor_record_offset":17564,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":18800,"parent_record_offset":18772,"ancestor_record_offset":17564,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":18828,"parent_record_offset":18800,"ancestor_record_offset":17564,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":18888,"parent_record_offset":18772,"ancestor_record_offset":17564,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":18772,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18576,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":20056,"parent_record_offset":20032,"ancestor_record_offset":17564,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":18920,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21104,"parent_record_offset":21080,"ancestor_record_offset":17564,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":21548,"parent_record_offset":20636,"ancestor_record_offset":17564,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":18632,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":21136,"parent_record_offset":21048,"ancestor_record_offset":17564,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":18716,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":19144,"parent_record_offset":18528,"ancestor_record_offset":17564,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":20924,"parent_record_offset":20884,"ancestor_record_offset":17564,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":20612,"parent_record_offset":20588,"ancestor_record_offset":17564,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":19056,"parent_record_offset":18920,"ancestor_record_offset":17564,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":17564,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":17564,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":19516,"parent_record_offset":19484,"ancestor_record_offset":17564,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":18256,"parent_record_offset":18080,"ancestor_record_offset":17564,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":18080,"parent_record_offset":18052,"ancestor_record_offset":17564,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":18052,"parent_record_offset":17900,"ancestor_record_offset":17564,"inlinee":6696,"source_line":590,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$32::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":19484,"parent_record_offset":19456,"ancestor_record_offset":17564,"inlinee":6705,"source_line":722,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":17932,"parent_record_offset":17900,"ancestor_record_offset":17564,"inlinee":6694,"source_line":152,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":19752,"parent_record_offset":19484,"ancestor_record_offset":17564,"inlinee":6707,"source_line":159,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":475,"record_offset":18112,"parent_record_offset":18080,"ancestor_record_offset":17564,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 36
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":13156,"rva":6716064,"va":5375425184}
// 编译实例 RVAs: 0x667aa0, 0x1dc84b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140365fd0
//     owner=unresolved_namespace; count=4
//     target_aliases=21 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3153608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e838e1cfff","size":5,"soft_fail":false,"target_va":5372272592,"va":5375426195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2675942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e81a2bd7ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375425649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140666e60
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e8b1eeffff","size":5,"soft_fail":false,"target_va":5375422048,"va":5375426474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407db0e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1520924","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e81c351700","size":5,"soft_fail":false,"target_va":5376946400,"va":5375425471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cc590
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27674705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e85148a601","size":5,"soft_fail":false,"target_va":5403100560,"va":5375425850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142116030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3463393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5399938224,"raw_hex":"e8e1d83400","size":5,"soft_fail":false,"target_va":5403402288,"va":5399938890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27554352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e83072a401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375425771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t27550027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e84b61a401","size":5,"soft_fail":false,"target_va":5402975328,"va":5375425296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27552185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e8b969a401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375425602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::write_contended
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27552218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375425184,"raw_hex":"e8da69a401","size":5,"soft_fail":false,"target_va":5402977488,"va":5375425265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5375425184,"raw_hex":"ff10","size":2,"soft_fail":false,"va":5375425626,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":15120,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":14016,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":14112,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":16068,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":13552,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":6693,"source_line":359,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::impl$26::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":14056,"parent_record_offset":13156,"ancestor_record_offset":13156,"inlinee":26959,"source_line":665,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":13156,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":475,"record_offset":15684,"parent_record_offset":15600,"ancestor_record_offset":13156,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":475,"record_offset":15600,"parent_record_offset":15572,"ancestor_record_offset":13156,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":475,"record_offset":15628,"parent_record_offset":15600,"ancestor_record_offset":13156,"inlinee":4115,"source_line":199,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":475,"record_offset":14080,"parent_record_offset":14056,"ancestor_record_offset":13156,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":13156,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":13156,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":14428,"parent_record_offset":14404,"ancestor_record_offset":13156,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":14368,"parent_record_offset":14344,"ancestor_record_offset":13156,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":475,"record_offset":15652,"parent_record_offset":15628,"ancestor_record_offset":13156,"inlinee":4117,"source_line":392,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":475,"record_offset":16876,"parent_record_offset":16848,"ancestor_record_offset":13156,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":14544,"parent_record_offset":14520,"ancestor_record_offset":13156,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":15544,"parent_record_offset":15516,"ancestor_record_offset":13156,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":13156,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":14684,"parent_record_offset":14660,"ancestor_record_offset":13156,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":17088,"parent_record_offset":17060,"ancestor_record_offset":13156,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":17116,"parent_record_offset":17088,"ancestor_record_offset":13156,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":13668,"parent_record_offset":13608,"ancestor_record_offset":13156,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":13936,"parent_record_offset":13908,"ancestor_record_offset":13156,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":15356,"parent_record_offset":15180,"ancestor_record_offset":13156,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":13156,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":15448,"parent_record_offset":15416,"ancestor_record_offset":13156,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":13840,"parent_record_offset":13812,"ancestor_record_offset":13156,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":13640,"parent_record_offset":13608,"ancestor_record_offset":13156,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13156,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":15380,"parent_record_offset":15356,"ancestor_record_offset":13156,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":15472,"parent_record_offset":15448,"ancestor_record_offset":13156,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":475,"record_offset":14772,"parent_record_offset":14748,"ancestor_record_offset":13156,"inlinee":9045,"source_line":124,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":15052,"parent_record_offset":14144,"ancestor_record_offset":13156,"inlinee":6552,"source_line":1105,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::rustc_entry
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":14144,"parent_record_offset":14112,"ancestor_record_offset":13156,"inlinee":6541,"source_line":34,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14660,"parent_record_offset":14636,"ancestor_record_offset":13156,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14636,"parent_record_offset":14612,"ancestor_record_offset":13156,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14960,"parent_record_offset":14936,"ancestor_record_offset":13156,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":17060,"parent_record_offset":16116,"ancestor_record_offset":13156,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14900,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14936,"parent_record_offset":14180,"ancestor_record_offset":13156,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14180,"parent_record_offset":14144,"ancestor_record_offset":13156,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16116,"parent_record_offset":16092,"ancestor_record_offset":13156,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":15076,"parent_record_offset":15052,"ancestor_record_offset":13156,"inlinee":6553,"source_line":1175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16920,"parent_record_offset":16116,"ancestor_record_offset":13156,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14208,"parent_record_offset":14180,"ancestor_record_offset":13156,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16188,"parent_record_offset":16156,"ancestor_record_offset":13156,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16368,"parent_record_offset":16188,"ancestor_record_offset":13156,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16532,"parent_record_offset":16188,"ancestor_record_offset":13156,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16564,"parent_record_offset":16532,"ancestor_record_offset":13156,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16156,"parent_record_offset":16116,"ancestor_record_offset":13156,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14292,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16848,"parent_record_offset":16824,"ancestor_record_offset":13156,"inlinee":6566,"source_line":2743,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16824,"parent_record_offset":16156,"ancestor_record_offset":13156,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":14492,"parent_record_offset":14460,"ancestor_record_offset":13156,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":14520,"parent_record_offset":14492,"ancestor_record_offset":13156,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":14580,"parent_record_offset":14460,"ancestor_record_offset":13156,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":14460,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14264,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":14612,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":16588,"parent_record_offset":16564,"ancestor_record_offset":13156,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":17032,"parent_record_offset":16116,"ancestor_record_offset":13156,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":14320,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":16620,"parent_record_offset":16532,"ancestor_record_offset":13156,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":14404,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":14808,"parent_record_offset":14208,"ancestor_record_offset":13156,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":16408,"parent_record_offset":16368,"ancestor_record_offset":13156,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":16092,"parent_record_offset":16068,"ancestor_record_offset":13156,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::impl$0::rustc_entry::closure$0
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":475,"record_offset":14748,"parent_record_offset":14612,"ancestor_record_offset":13156,"inlinee":6544,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":13156,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":475,"record_offset":13788,"parent_record_offset":13764,"ancestor_record_offset":13156,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":15180,"parent_record_offset":15148,"ancestor_record_offset":13156,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":13908,"parent_record_offset":13732,"ancestor_record_offset":13156,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::guard
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":13732,"parent_record_offset":13704,"ancestor_record_offset":13156,"inlinee":6697,"source_line":42,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockWriteGuard::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":13704,"parent_record_offset":13552,"ancestor_record_offset":13156,"inlinee":6696,"source_line":590,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$32::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":15148,"parent_record_offset":15120,"ancestor_record_offset":13156,"inlinee":6705,"source_line":722,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":13584,"parent_record_offset":13552,"ancestor_record_offset":13156,"inlinee":6694,"source_line":152,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::write_unlock
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":15416,"parent_record_offset":15148,"ancestor_record_offset":13156,"inlinee":6707,"source_line":159,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":475,"record_offset":13764,"parent_record_offset":13732,"ancestor_record_offset":13156,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":8168,"rva":6714944,"va":5375424064}
// 编译实例 RVAs: 0x667640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5810642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424064,"raw_hex":"e82e56a7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375424157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27555685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424064,"raw_hex":"e86577a401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375424438,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27552822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424064,"raw_hex":"e8366ca401","size":5,"soft_fail":false,"target_va":5402977200,"va":5375424373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27553336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424064,"raw_hex":"e8386ea401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375424451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":10288,"parent_record_offset":8168,"ancestor_record_offset":8168,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":10316,"parent_record_offset":8168,"ancestor_record_offset":8168,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":8976,"parent_record_offset":8168,"ancestor_record_offset":8168,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":9212,"parent_record_offset":8168,"ancestor_record_offset":8168,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":8540,"parent_record_offset":8168,"ancestor_record_offset":8168,"inlinee":6660,"source_line":271,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":9932,"parent_record_offset":9908,"ancestor_record_offset":8168,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":8168,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":8168,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":9500,"parent_record_offset":9476,"ancestor_record_offset":8168,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":9676,"parent_record_offset":9652,"ancestor_record_offset":8168,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":9020,"parent_record_offset":8976,"ancestor_record_offset":8168,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":9844,"parent_record_offset":9820,"ancestor_record_offset":8168,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":9820,"parent_record_offset":9796,"ancestor_record_offset":8168,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":8752,"parent_record_offset":8576,"ancestor_record_offset":8168,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":8776,"parent_record_offset":8752,"ancestor_record_offset":8168,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":8896,"parent_record_offset":8868,"ancestor_record_offset":8168,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":8696,"parent_record_offset":8576,"ancestor_record_offset":8168,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":9136,"parent_record_offset":9104,"ancestor_record_offset":8168,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":8612,"parent_record_offset":8576,"ancestor_record_offset":8168,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":8720,"parent_record_offset":8696,"ancestor_record_offset":8168,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":8168,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":9160,"parent_record_offset":9136,"ancestor_record_offset":8168,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":8168,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":8168,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":475,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":8168,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":9236,"parent_record_offset":9212,"ancestor_record_offset":8168,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":8168,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":9884,"parent_record_offset":9744,"ancestor_record_offset":8168,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":9292,"parent_record_offset":9260,"ancestor_record_offset":8168,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9796,"parent_record_offset":9772,"ancestor_record_offset":8168,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9772,"parent_record_offset":9744,"ancestor_record_offset":8168,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":8168,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":10120,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":10156,"parent_record_offset":9344,"ancestor_record_offset":8168,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9344,"parent_record_offset":9320,"ancestor_record_offset":8168,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9320,"parent_record_offset":9260,"ancestor_record_offset":8168,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9372,"parent_record_offset":9344,"ancestor_record_offset":8168,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":9624,"parent_record_offset":9592,"ancestor_record_offset":8168,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":9652,"parent_record_offset":9624,"ancestor_record_offset":8168,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":9712,"parent_record_offset":9592,"ancestor_record_offset":8168,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":9592,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9424,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":9744,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":9452,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":9536,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":10028,"parent_record_offset":9372,"ancestor_record_offset":8168,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":8840,"parent_record_offset":8812,"ancestor_record_offset":8168,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":8168,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":8812,"parent_record_offset":8540,"ancestor_record_offset":8168,"inlinee":6667,"source_line":578,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$31::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":9076,"parent_record_offset":9048,"ancestor_record_offset":8168,"inlinee":6690,"source_line":712,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":8576,"parent_record_offset":8540,"ancestor_record_offset":8168,"inlinee":6662,"source_line":78,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":9104,"parent_record_offset":9076,"ancestor_record_offset":8168,"inlinee":6691,"source_line":91,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":8668,"parent_record_offset":8576,"ancestor_record_offset":8168,"inlinee":6664,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":3024,"rva":6713664,"va":5375422784}
// 编译实例 RVAs: 0x667140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5809362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375422784,"raw_hex":"e82e5ba7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375422877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27556965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375422784,"raw_hex":"e8657ca401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375423158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27554102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375422784,"raw_hex":"e83671a401","size":5,"soft_fail":false,"target_va":5402977200,"va":5375423093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27554616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375422784,"raw_hex":"e83873a401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375423171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":5200,"parent_record_offset":3024,"ancestor_record_offset":3024,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":5228,"parent_record_offset":3024,"ancestor_record_offset":3024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":3888,"parent_record_offset":3024,"ancestor_record_offset":3024,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":4124,"parent_record_offset":3024,"ancestor_record_offset":3024,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":3452,"parent_record_offset":3024,"ancestor_record_offset":3024,"inlinee":6660,"source_line":271,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":4844,"parent_record_offset":4820,"ancestor_record_offset":3024,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":4388,"parent_record_offset":4364,"ancestor_record_offset":3024,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3024,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":4412,"parent_record_offset":4388,"ancestor_record_offset":3024,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":4588,"parent_record_offset":4564,"ancestor_record_offset":3024,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":3932,"parent_record_offset":3888,"ancestor_record_offset":3024,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":4756,"parent_record_offset":4732,"ancestor_record_offset":3024,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":4732,"parent_record_offset":4708,"ancestor_record_offset":3024,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":3664,"parent_record_offset":3488,"ancestor_record_offset":3024,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":3024,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":3024,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":3608,"parent_record_offset":3488,"ancestor_record_offset":3024,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":4048,"parent_record_offset":4016,"ancestor_record_offset":3024,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":3524,"parent_record_offset":3488,"ancestor_record_offset":3024,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3024,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":3548,"parent_record_offset":3524,"ancestor_record_offset":3024,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":4072,"parent_record_offset":4048,"ancestor_record_offset":3024,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":4892,"parent_record_offset":4868,"ancestor_record_offset":3024,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":4868,"parent_record_offset":4844,"ancestor_record_offset":3024,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":475,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3024,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":3024,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":4172,"parent_record_offset":4148,"ancestor_record_offset":3024,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":4796,"parent_record_offset":4656,"ancestor_record_offset":3024,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":4204,"parent_record_offset":4172,"ancestor_record_offset":3024,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":3024,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4684,"parent_record_offset":4656,"ancestor_record_offset":3024,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":5092,"parent_record_offset":5068,"ancestor_record_offset":3024,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":5032,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":5068,"parent_record_offset":4256,"ancestor_record_offset":3024,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":3024,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4232,"parent_record_offset":4172,"ancestor_record_offset":3024,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":3024,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":4536,"parent_record_offset":4504,"ancestor_record_offset":3024,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":4564,"parent_record_offset":4536,"ancestor_record_offset":3024,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":4624,"parent_record_offset":4504,"ancestor_record_offset":3024,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":4504,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4336,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":4656,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":4364,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":4448,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":4940,"parent_record_offset":4284,"ancestor_record_offset":3024,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":3752,"parent_record_offset":3724,"ancestor_record_offset":3024,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":3780,"parent_record_offset":3752,"ancestor_record_offset":3024,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":3724,"parent_record_offset":3452,"ancestor_record_offset":3024,"inlinee":6667,"source_line":578,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$31::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":3024,"inlinee":6690,"source_line":712,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":3488,"parent_record_offset":3452,"ancestor_record_offset":3024,"inlinee":6662,"source_line":78,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":3024,"inlinee":6691,"source_line":91,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":3580,"parent_record_offset":3488,"ancestor_record_offset":3024,"inlinee":6664,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":475,"record_offset":10636,"rva":6715504,"va":5375424624}
// 编译实例 RVAs: 0x667870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5811202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424624,"raw_hex":"e8fe53a7ff","size":5,"soft_fail":false,"target_va":5369613520,"va":5375424717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27555125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424624,"raw_hex":"e83575a401","size":5,"soft_fail":false,"target_va":5402980128,"va":5375424998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27552262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424624,"raw_hex":"e8066aa401","size":5,"soft_fail":false,"target_va":5402977200,"va":5375424933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t27552776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375424624,"raw_hex":"e8086ca401","size":5,"soft_fail":false,"target_va":5402977792,"va":5375425011,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":475,"record_offset":12808,"parent_record_offset":10636,"ancestor_record_offset":10636,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":12836,"parent_record_offset":10636,"ancestor_record_offset":10636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":11496,"parent_record_offset":10636,"ancestor_record_offset":10636,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":475,"record_offset":11732,"parent_record_offset":10636,"ancestor_record_offset":10636,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":11060,"parent_record_offset":10636,"ancestor_record_offset":10636,"inlinee":6660,"source_line":271,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":475,"record_offset":12452,"parent_record_offset":12428,"ancestor_record_offset":10636,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":11996,"parent_record_offset":11972,"ancestor_record_offset":10636,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":475,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":10636,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":475,"record_offset":12020,"parent_record_offset":11996,"ancestor_record_offset":10636,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":475,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":10636,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":475,"record_offset":11540,"parent_record_offset":11496,"ancestor_record_offset":10636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":10636,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":475,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":10636,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":11272,"parent_record_offset":11096,"ancestor_record_offset":10636,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":475,"record_offset":11296,"parent_record_offset":11272,"ancestor_record_offset":10636,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11416,"parent_record_offset":11388,"ancestor_record_offset":10636,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11216,"parent_record_offset":11096,"ancestor_record_offset":10636,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11656,"parent_record_offset":11624,"ancestor_record_offset":10636,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11132,"parent_record_offset":11096,"ancestor_record_offset":10636,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11240,"parent_record_offset":11216,"ancestor_record_offset":10636,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":10636,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":475,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":10636,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":12500,"parent_record_offset":12476,"ancestor_record_offset":10636,"inlinee":22473,"source_line":47,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::impl$7::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":475,"record_offset":12476,"parent_record_offset":12452,"ancestor_record_offset":10636,"inlinee":22471,"source_line":103,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":475,"record_offset":12428,"parent_record_offset":12404,"ancestor_record_offset":10636,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":11756,"parent_record_offset":11732,"ancestor_record_offset":10636,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":10636,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":12404,"parent_record_offset":12264,"ancestor_record_offset":10636,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":475,"record_offset":11812,"parent_record_offset":11780,"ancestor_record_offset":10636,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":10636,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12292,"parent_record_offset":12264,"ancestor_record_offset":10636,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12700,"parent_record_offset":12676,"ancestor_record_offset":10636,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12640,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12676,"parent_record_offset":11864,"ancestor_record_offset":10636,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":10636,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":11840,"parent_record_offset":11780,"ancestor_record_offset":10636,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":10636,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":12144,"parent_record_offset":12112,"ancestor_record_offset":10636,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":12172,"parent_record_offset":12144,"ancestor_record_offset":10636,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":12232,"parent_record_offset":12112,"ancestor_record_offset":10636,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":475,"record_offset":12112,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":11944,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":475,"record_offset":12264,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":11972,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":12056,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":475,"record_offset":12548,"parent_record_offset":11892,"ancestor_record_offset":10636,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":11360,"parent_record_offset":11332,"ancestor_record_offset":10636,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":475,"record_offset":11388,"parent_record_offset":11360,"ancestor_record_offset":10636,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":11332,"parent_record_offset":11060,"ancestor_record_offset":10636,"inlinee":6667,"source_line":578,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$31::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":475,"record_offset":11596,"parent_record_offset":11568,"ancestor_record_offset":10636,"inlinee":6690,"source_line":712,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":11096,"parent_record_offset":11060,"ancestor_record_offset":10636,"inlinee":6662,"source_line":78,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":11624,"parent_record_offset":11596,"ancestor_record_offset":10636,"inlinee":6691,"source_line":91,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":475,"record_offset":11188,"parent_record_offset":11096,"ancestor_record_offset":10636,"inlinee":6664,"source_line":49,"archive":"_pdb/A/modules/00475.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
// 记录形式: compiled_procedure; instances=13
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":219,"record_offset":9740,"rva":3381024,"va":5372090144}
// 编译实例 RVAs: 0x339720, 0x667370, 0x8d5270, 0xc98240, 0x198f130, 0x1a99fd0, 0x1adfa50, 0x1b2daf0, 0x1b9b310, 0x1c3d380, 0x1db9d10, 0x1dbc300, 0x1dc81e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1407db0e0
//     owner=unresolved_namespace; count=13
//     target_aliases=2 candidates; complete list: lib.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4856027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372090144,"raw_hex":"e8db184a00","size":5,"soft_fail":false,"target_va":5376946400,"va":5372090368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t30889603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372090144,"raw_hex":"e88356d701","size":5,"soft_fail":false,"target_va":5402980128,"va":5372090520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::read_contended
//     owner=rust_standard_library; count=13
//     evidence={"assembly":"\tcallq\t30886739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372090144,"raw_hex":"e8534bd701","size":5,"soft_fail":false,"target_va":5402977200,"va":5372090456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers
//     owner=rust_standard_library; count=39
//     evidence={"assembly":"\tcallq\t30887190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5372090144,"raw_hex":"e8164dd701","size":5,"soft_fail":false,"target_va":5402977792,"va":5372090597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":219,"record_offset":12008,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":12036,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":219,"record_offset":10616,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":219,"record_offset":10856,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::read
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":219,"record_offset":10180,"parent_record_offset":9740,"ancestor_record_offset":9740,"inlinee":6660,"source_line":271,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":219,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":9740,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":219,"record_offset":11228,"parent_record_offset":11204,"ancestor_record_offset":9740,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":219,"record_offset":11312,"parent_record_offset":11288,"ancestor_record_offset":9740,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::BuildHasher::hash_one
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":219,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":9740,"inlinee":8489,"source_line":696,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$5::hash
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":219,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":9740,"inlinee":5832,"source_line":941,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":219,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":9740,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":219,"record_offset":11428,"parent_record_offset":11404,"ancestor_record_offset":9740,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":219,"record_offset":10660,"parent_record_offset":10616,"ancestor_record_offset":9740,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":219,"record_offset":11592,"parent_record_offset":11568,"ancestor_record_offset":9740,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":219,"record_offset":11568,"parent_record_offset":11544,"ancestor_record_offset":9740,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":219,"record_offset":10392,"parent_record_offset":10216,"ancestor_record_offset":9740,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":219,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":9740,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10536,"parent_record_offset":10508,"ancestor_record_offset":9740,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10336,"parent_record_offset":10216,"ancestor_record_offset":9740,"inlinee":6665,"source_line":2629,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10780,"parent_record_offset":10744,"ancestor_record_offset":9740,"inlinee":6692,"source_line":2695,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10252,"parent_record_offset":10216,"ancestor_record_offset":9740,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":9740,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10276,"parent_record_offset":10252,"ancestor_record_offset":9740,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":219,"record_offset":10804,"parent_record_offset":10780,"ancestor_record_offset":9740,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$11::eq
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":219,"record_offset":11704,"parent_record_offset":11680,"ancestor_record_offset":9740,"inlinee":9045,"source_line":124,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::impl$9::hash
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":219,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":9740,"inlinee":13032,"source_line":124,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":219,"record_offset":11656,"parent_record_offset":11632,"ancestor_record_offset":9740,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":219,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":9740,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":219,"record_offset":10904,"parent_record_offset":10880,"ancestor_record_offset":9740,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":219,"record_offset":11632,"parent_record_offset":11496,"ancestor_record_offset":9740,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":219,"record_offset":10932,"parent_record_offset":10904,"ancestor_record_offset":9740,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":9740,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":9740,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11900,"parent_record_offset":11876,"ancestor_record_offset":9740,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11840,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11876,"parent_record_offset":11096,"ancestor_record_offset":9740,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":9740,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11072,"parent_record_offset":10904,"ancestor_record_offset":9740,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":9740,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":219,"record_offset":11376,"parent_record_offset":11344,"ancestor_record_offset":9740,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":219,"record_offset":11404,"parent_record_offset":11376,"ancestor_record_offset":9740,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":219,"record_offset":11464,"parent_record_offset":11344,"ancestor_record_offset":9740,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":219,"record_offset":11344,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11176,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":219,"record_offset":11496,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":219,"record_offset":11204,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=26
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":219,"record_offset":11288,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=13
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":219,"record_offset":11748,"parent_record_offset":11124,"ancestor_record_offset":9740,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":219,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9740,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":219,"record_offset":10508,"parent_record_offset":10480,"ancestor_record_offset":9740,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::RwLockReadGuard::new
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":219,"record_offset":10452,"parent_record_offset":10180,"ancestor_record_offset":9740,"inlinee":6667,"source_line":578,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::rwlock::impl$31::drop
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":219,"record_offset":10716,"parent_record_offset":10688,"ancestor_record_offset":9740,"inlinee":6690,"source_line":712,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":219,"record_offset":10216,"parent_record_offset":10180,"ancestor_record_offset":9740,"inlinee":6662,"source_line":78,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::RwLock::read_unlock
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":219,"record_offset":10744,"parent_record_offset":10716,"ancestor_record_offset":9740,"inlinee":6691,"source_line":91,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::rwlock::futex::is_read_lockable
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\rwlock\futex.rs
//     evidence={"module":219,"record_offset":10308,"parent_record_offset":10216,"ancestor_record_offset":9740,"inlinee":6664,"source_line":49,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   [pdb_inline_transitive] utils::name_hash::impl$0::hash
//     owner=private_source_ownership_candidate; count=13
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/name_hash.rs
//     evidence={"module":219,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":9740,"inlinee":12744,"source_line":10,"archive":"_pdb/A/modules/00219.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::FrozenMap::as_mut
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 113
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E50
// 首条位置: {"module":419,"inlinee":24144}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":419,"record_offset":7428,"parent_record_offset":7260,"ancestor_record_offset":7260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::get_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":419,"record_offset":7288,"parent_record_offset":7260,"ancestor_record_offset":7260,"inlinee":24145,"source_line":531,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":419,"record_offset":11268,"parent_record_offset":11124,"ancestor_record_offset":11096,"inlinee":24149,"source_line":2147,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":419,"record_offset":7360,"parent_record_offset":7336,"ancestor_record_offset":7260,"inlinee":6672,"source_line":609,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":419,"record_offset":7384,"parent_record_offset":7360,"ancestor_record_offset":7260,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::borrow
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":419,"record_offset":7312,"parent_record_offset":7288,"ancestor_record_offset":7260,"inlinee":6669,"source_line":36,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":419,"record_offset":7336,"parent_record_offset":7312,"ancestor_record_offset":7260,"inlinee":6670,"source_line":64,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: frozen::impl$7::default
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5E4C
// 首条位置: {"module":419,"inlinee":24140}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":419,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_direct] std::sync::rwlock::RwLock::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\rwlock.rs
//     evidence={"module":419,"record_offset":1476,"parent_record_offset":1008,"ancestor_record_offset":1008,"inlinee":6258,"source_line":223,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":419,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":1008,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":419,"record_offset":1364,"parent_record_offset":1340,"ancestor_record_offset":1008,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":419,"record_offset":1340,"parent_record_offset":1288,"ancestor_record_offset":1008,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":419,"record_offset":1388,"parent_record_offset":1364,"ancestor_record_offset":1008,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":419,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":1008,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":419,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":1008,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":1008,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":419,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":1008,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":1288,"parent_record_offset":1148,"ancestor_record_offset":1008,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":419,"record_offset":1064,"parent_record_offset":1036,"ancestor_record_offset":1008,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":419,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":1008,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":419,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":1008,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":419,"record_offset":1148,"parent_record_offset":1120,"ancestor_record_offset":1008,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00419.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":419,"record_offset":1120,"parent_record_offset":1092,"ancestor_record_offset":1008,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00419.jsonl.gz"}
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
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: lib.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-gates/findings.md; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "exact_PDB_named_entity_mention", "name": "frozen::FrozenMap", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: lib.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 21; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>,bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>,bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>", "bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>", "bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::path_audio::PathAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::path_stroke::PathEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::stairs_audio::SuppressPathSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>,bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>,bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>", "bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>", "bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::garden_audio::GardenAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::garden::GardenDrawEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::garden::GardenEraseEvent>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>,bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::terrain_audio::TerrainAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::audio::terrain_audio::PlayTerrainSound> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>,bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>,bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>,bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>,bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >,bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>", "bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>", "bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>", "bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>", "bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>", "bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>", "bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >", "bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::signifier_audio::SignifierAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::ModeJustChanged>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::save::load_game::LoadingGameComplete>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::InitAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::FinishAdjWallHeightMode>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<country_core::systems::ui::resources::widget_params::PlayUiSound> >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>,bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>", "bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::replay_audio::ReplayAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::replay_audio::PlayReplayAudio>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>,bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>", "bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::as_mut
// 信息 ID: f2466733588476ba60ebc8db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut; module=419, inline_record=7260; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::remove; module=419, inline_record=11096; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::export_from_rg; module=475, inline_record=30608; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roof_tile_ssbo::RoofTileSsbo>; module=744, inline_record=504; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>; module=1211, inline_record=524; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>; module=1304, inline_record=74472; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_mut<country_core::geometry::instanced_wall::BrickTransformSsbo>; module=1304, inline_record=76080; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=28996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >
// 信息 ID: 448924da8e01a79e90d99a73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::mesh::Mesh", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$51,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> > @ 0x521033 -> 0x667640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get @ 0x5964b2 -> 0x667640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name @ 0x59662d -> 0x667640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::vao_library::AssetVAOLibrary::add @ 0x5ac5a9 -> 0x667640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::vao_library::AssetVAOLibrary::rebuild_vao @ 0x5ac75a -> 0x667640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::backend::vao_update::build_missing_vaos @ 0x5c1545 -> 0x667640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram> >
// 信息 ID: bb73534a66cb798904c3674c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::ComputeShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_compute_by_name @ 0x6a8de1 -> 0x667140 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x1000e14 -> 0x667140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram> >
// 信息 ID: a3cd0a47dff557a04f2258bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::RasterShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster @ 0x6a83dd -> 0x667870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x6aaf35 -> 0x667870 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::impl$0::drop @ 0x6f212b -> 0x667870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
// 信息 ID: 01fde37f40b5a35e0d5b4571
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::boxed::Box -> ["country_core::asset_libraries::ssbo_library::BufferState", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] nohash_hasher::NoHashHasher -> ["country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// [类型关系] core::hash::BuildHasherDefault -> ["nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::get -> ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >", "country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 330 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x10738e -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x1073f4 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x107469 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x10766e -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x1076d4 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x107749 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x10794e -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x1079b4 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x107a29 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x107c2e -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x107c94 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_rg_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x107d09 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3bdcd4 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3bded3 -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3be0bd -> 0x339720 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::read_mouse_depth_value::read_mouse_depth_value_from_gpu @ 0x505b46 -> 0x667370 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: 8e6635e70b6df3eb1caba699
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::mesh::Mesh"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::mesh::Mesh", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add @ 0x596006 -> 0x6689d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>,alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: 833659b2e1d5b8d0be42ac6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::ComputeShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::ComputeShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::shader::ComputeShaderProgram>", "alloc::boxed::Box<country_core::render::shader::ComputeShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_compute @ 0x6a8b24 -> 0x668580 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>,alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>,std::hash::random::RandomState>
// 信息 ID: 325f504ed6ef91dc7c5de33f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::asset_libraries::Handle -> ["country_core::render::shader::RasterShaderProgram"]; 字段布局 = unknown。
// [类型关系] alloc::boxed::Box -> ["country_core::render::shader::RasterShaderProgram", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>", "alloc::boxed::Box<country_core::render::shader::RasterShaderProgram,alloc::alloc::Global>", "std::hash::random::RandomState"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster @ 0x6a84d7 -> 0x668030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::FrozenMap::insert<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> > >
// 信息 ID: 746c48640d0032ffe1b5b484
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::boxed::Box -> ["country_core::asset_libraries::ssbo_library::BufferState", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] nohash_hasher::NoHashHasher -> ["country_core::asset_libraries::ssbo_library::SsboId"]; 字段布局 = unknown。
// [类型关系] core::hash::BuildHasherDefault -> ["nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>"]; 字段布局 = unknown。
// [类型关系] frozen::FrozenMap::insert -> ["country_core::asset_libraries::ssbo_library::SsboId", "alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>", "core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 73 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO> @ 0xd27897 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO> @ 0xd27b13 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> @ 0xd27d93 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0xd28013 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo> @ 0xd28296 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new_with_data<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0xd28516 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> @ 0xd28792 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::roof_tile_ssbo::RoofTileSsbo> @ 0xd28a2c -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo> @ 0xd28cca -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo> @ 0xd28f4a -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer> @ 0xd291c2 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::audio::stairs_audio::StairsCounterSsbo> @ 0xd2944a -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::render::nani::buffers::NaniDrawListLengthSsbo> @ 0xd296c2 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::resources::water::RippleSourceSsbo> @ 0xd29942 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0xd29bc2 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::add_new<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> @ 0xd29e42 -> 0x667aa0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: frozen::impl$7::default
// 信息 ID: 3993a95ce4a49e85af14a2ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::new; module=419, inline_record=1008; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::new; module=475, inline_record=30368; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::impl$0::insert_rendering_resources; module=480, inline_record=14104; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::new; module=489, inline_record=33336; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::new; module=489, inline_record=34352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
