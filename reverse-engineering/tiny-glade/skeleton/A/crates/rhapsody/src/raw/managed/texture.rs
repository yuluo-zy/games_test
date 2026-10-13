// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/managed/texture.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3d904dbd2cf2bebc2fa520ca5cabe4e21d4b980bb9e8cec626af9ee4d76ddd1e
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 566
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arc_swap; // namespace candidate
//   package identities: [{"name":"arc-swap","source":"PDB_registry_path_only","version":"1.7.1"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
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
// 首条位置: {"module":566,"record_offset":1060,"rva":7555168,"va":5376264288}
// 编译实例 RVAs: 0x734860
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264288,"raw_hex":"e8177affff","size":5,"soft_fail":false,"target_va":5376231008,"va":5376265284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27328796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264288,"raw_hex":"e81c01a101","size":5,"soft_fail":false,"target_va":5403593392,"va":5376264591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4959508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264288,"raw_hex":"e8ec52b4ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5376264559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26698876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264288,"raw_hex":"e87c649701","size":5,"soft_fail":false,"target_va":5402963904,"va":5376265023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26700300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376264288,"raw_hex":"e80c6a9701","size":5,"soft_fail":false,"target_va":5402965344,"va":5376265039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::Handle::is_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":566,"record_offset":3772,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":30809,"source_line":63,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":3252,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":2768,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":1216,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::disposal::DisposalBucket::dispose
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":2580,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":30801,"source_line":119,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::drain
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":1396,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":24732,"source_line":618,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$68::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":1896,"parent_record_offset":1060,"ancestor_record_offset":1060,"inlinee":24737,"source_line":2553,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":566,"record_offset":2696,"parent_record_offset":2672,"ancestor_record_offset":1060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2672,"parent_record_offset":2636,"ancestor_record_offset":1060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":566,"record_offset":2636,"parent_record_offset":2608,"ancestor_record_offset":1060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":1060,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":1724,"parent_record_offset":1692,"ancestor_record_offset":1060,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":566,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":1060,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::write_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":566,"record_offset":2932,"parent_record_offset":2900,"ancestor_record_offset":1060,"inlinee":8557,"source_line":3145,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":566,"record_offset":1824,"parent_record_offset":1800,"ancestor_record_offset":1060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1060,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":1760,"parent_record_offset":1604,"ancestor_record_offset":1060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::copy_from_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":2996,"parent_record_offset":2840,"ancestor_record_offset":1060,"inlinee":7420,"source_line":1456,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":2412,"parent_record_offset":2388,"ancestor_record_offset":1060,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":2388,"parent_record_offset":2364,"ancestor_record_offset":1060,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":3316,"parent_record_offset":3280,"ancestor_record_offset":1060,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":1848,"parent_record_offset":1824,"ancestor_record_offset":1060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":1332,"parent_record_offset":1244,"ancestor_record_offset":1060,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":1356,"parent_record_offset":1332,"ancestor_record_offset":1060,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":3156,"parent_record_offset":3124,"ancestor_record_offset":1060,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":1276,"parent_record_offset":1244,"ancestor_record_offset":1060,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":3180,"parent_record_offset":3156,"ancestor_record_offset":1060,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":1060,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::drain
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":1424,"parent_record_offset":1396,"ancestor_record_offset":1060,"inlinee":24733,"source_line":890,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$81::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1060,"inlinee":24739,"source_line":4952,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2364,"parent_record_offset":2012,"ancestor_record_offset":1060,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1604,"parent_record_offset":1512,"ancestor_record_offset":1060,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2012,"parent_record_offset":1980,"ancestor_record_offset":1060,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1452,"parent_record_offset":1424,"ancestor_record_offset":1060,"inlinee":24734,"source_line":1622,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::drain_iter_from
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1800,"parent_record_offset":1452,"ancestor_record_offset":1060,"inlinee":24735,"source_line":1637,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":1060,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1576,"parent_record_offset":1512,"ancestor_record_offset":1060,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::clear_no_drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":1060,"inlinee":15002,"source_line":3337,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1548,"parent_record_offset":1512,"ancestor_record_offset":1060,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1512,"parent_record_offset":1484,"ancestor_record_offset":1060,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::num_ctrl_bytes
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2900,"parent_record_offset":2868,"ancestor_record_offset":1060,"inlinee":11766,"source_line":2822,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":2100,"parent_record_offset":2060,"ancestor_record_offset":1060,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":2132,"parent_record_offset":2100,"ancestor_record_offset":1060,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":2192,"parent_record_offset":2060,"ancestor_record_offset":1060,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":2060,"parent_record_offset":2012,"ancestor_record_offset":1060,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bucket_mask_to_capacity
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2964,"parent_record_offset":2868,"ancestor_record_offset":1060,"inlinee":11754,"source_line":222,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":1060,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$45::drop
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":1060,"inlinee":24743,"source_line":4394,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$46::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":1060,"inlinee":24741,"source_line":4415,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":1636,"parent_record_offset":1604,"ancestor_record_offset":1060,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":1692,"parent_record_offset":1636,"ancestor_record_offset":1060,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":3096,"parent_record_offset":3068,"ancestor_record_offset":1060,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":1060,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":1060,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$13::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":3592,"parent_record_offset":3564,"ancestor_record_offset":1060,"inlinee":30807,"source_line":263,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$4::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":3412,"parent_record_offset":3388,"ancestor_record_offset":1060,"inlinee":30805,"source_line":209,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::disposal::impl$5::dispose_to_bucket
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/disposal.rs
//     evidence={"module":566,"record_offset":2608,"parent_record_offset":2580,"ancestor_record_offset":1060,"inlinee":30803,"source_line":215,"archive":"_pdb/A/modules/00566.jsonl.gz"}
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
// 首条位置: {"module":566,"record_offset":4468,"rva":7556256,"va":5376265376}
// 编译实例 RVAs: 0x734ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402ced40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4612462","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e8929eb9ff","size":5,"soft_fail":false,"target_va":5371653440,"va":5376265897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3640893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e8c371c8ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5376265912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c660
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-35198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e88276ffff","size":5,"soft_fail":false,"target_va":5376231008,"va":5376266201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] arc_swap::debt::list::LocalNode::with
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10750669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e8cd0aa400","size":5,"soft_fail":false,"target_va":5387016528,"va":5376265854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::texture::create_vk_image_view
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e868090000","size":5,"soft_fail":false,"target_va":5376267952,"va":5376265539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1100576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376265376,"raw_hex":"e820cb1000","size":5,"soft_fail":false,"target_va":5377366224,"va":5376265643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":5204,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":4644,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":5176,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::impl$11::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":4680,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":24142,"source_line":1151,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$40::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":566,"record_offset":4620,"parent_record_offset":4468,"ancestor_record_offset":4468,"inlinee":26868,"source_line":251,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":566,"record_offset":5440,"parent_record_offset":5416,"ancestor_record_offset":4468,"inlinee":9141,"source_line":1394,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::from_raw_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":566,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":4468,"inlinee":9142,"source_line":1588,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":566,"record_offset":5660,"parent_record_offset":5636,"ancestor_record_offset":4468,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::debt::Debt::pay_all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/debt/mod.rs
//     evidence={"module":566,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":4468,"inlinee":9119,"source_line":81,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::impl$7::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/lib.rs
//     evidence={"module":566,"record_offset":5276,"parent_record_offset":5252,"ancestor_record_offset":4468,"inlinee":13375,"source_line":327,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::RefCnt::dec
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":566,"record_offset":5364,"parent_record_offset":5276,"ancestor_record_offset":4468,"inlinee":9138,"source_line":86,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$0::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":566,"record_offset":5416,"parent_record_offset":5388,"ancestor_record_offset":4468,"inlinee":9140,"source_line":121,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::ref_cnt::impl$2::from_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/ref_cnt.rs
//     evidence={"module":566,"record_offset":5388,"parent_record_offset":5364,"ancestor_record_offset":4468,"inlinee":9148,"source_line":169,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] arc_swap::strategy::hybrid::impl$5::wait_for_readers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arc-swap","version":"1.7.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arc-swap-1.7.1/src/strategy/hybrid.rs
//     evidence={"module":566,"record_offset":5308,"parent_record_offset":5276,"ancestor_record_offset":4468,"inlinee":9149,"source_line":196,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":5008,"parent_record_offset":4980,"ancestor_record_offset":4468,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":4468,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":566,"record_offset":5036,"parent_record_offset":4980,"ancestor_record_offset":4468,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":566,"record_offset":5588,"parent_record_offset":5364,"ancestor_record_offset":4468,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":566,"record_offset":5084,"parent_record_offset":5060,"ancestor_record_offset":4468,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":4468,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":5488,"parent_record_offset":5464,"ancestor_record_offset":4468,"inlinee":9143,"source_line":1077,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":4468,"inlinee":9145,"source_line":468,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":566,"record_offset":5512,"parent_record_offset":5488,"ancestor_record_offset":4468,"inlinee":9144,"source_line":1045,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":5228,"parent_record_offset":5204,"ancestor_record_offset":4468,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":5108,"parent_record_offset":5084,"ancestor_record_offset":4468,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":5696,"parent_record_offset":5660,"ancestor_record_offset":4468,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":5720,"parent_record_offset":5696,"ancestor_record_offset":4468,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":5752,"parent_record_offset":5660,"ancestor_record_offset":4468,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":4708,"parent_record_offset":4680,"ancestor_record_offset":4468,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":4764,"parent_record_offset":4736,"ancestor_record_offset":4468,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":566,"record_offset":4852,"parent_record_offset":4820,"ancestor_record_offset":4468,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":4980,"parent_record_offset":4820,"ancestor_record_offset":4468,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":566,"record_offset":4736,"parent_record_offset":4708,"ancestor_record_offset":4468,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":566,"record_offset":4880,"parent_record_offset":4852,"ancestor_record_offset":4468,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":566,"record_offset":4912,"parent_record_offset":4880,"ancestor_record_offset":4468,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":566,"record_offset":4820,"parent_record_offset":4792,"ancestor_record_offset":4468,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":566,"record_offset":4792,"parent_record_offset":4764,"ancestor_record_offset":4468,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00566.jsonl.gz"}
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
// 首条位置: {"module":566,"record_offset":5876,"rva":7557184,"va":5376266304}
// 编译实例 RVAs: 0x735040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141185d00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10816548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8240ca500","size":5,"soft_fail":false,"target_va":5387083008,"va":5376266455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411861e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10817242","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8da0ea500","size":5,"soft_fail":false,"target_va":5387084256,"va":5376267009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141188750
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10827255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8f735a500","size":5,"soft_fail":false,"target_va":5387093840,"va":5376266580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26711264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8e0949701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376267675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26696325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8855a9701","size":5,"soft_fail":false,"target_va":5402963904,"va":5376267574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::lock_shared_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26699950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8ae689701","size":5,"soft_fail":false,"target_va":5402966736,"va":5376266781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26697737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e809609701","size":5,"soft_fail":false,"target_va":5402965344,"va":5376267602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t26701617","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8316f9701","size":5,"soft_fail":false,"target_va":5402968352,"va":5376266730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::managed::texture::create_vk_image_view
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376266304,"raw_hex":"e8f3030000","size":5,"soft_fail":false,"target_va":5376267952,"va":5376266936,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::vk::Handle::is_null
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":566,"record_offset":7420,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":30809,"source_line":63,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":566,"record_offset":9276,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":7276,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":7604,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":9064,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":6052,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":5719,"source_line":447,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] lock_api::rwlock::RwLock::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":7640,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":5786,"source_line":479,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::Entry::or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":7880,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":30810,"source_line":2639,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":7824,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":6540,"source_line":855,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":6336,"parent_record_offset":5876,"ancestor_record_offset":5876,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":8488,"parent_record_offset":8464,"ancestor_record_offset":5876,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":5876,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":566,"record_offset":6692,"parent_record_offset":6668,"ancestor_record_offset":5876,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":566,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5876,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":6200,"parent_record_offset":6112,"ancestor_record_offset":5876,"inlinee":5724,"source_line":460,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":6232,"parent_record_offset":6200,"ancestor_record_offset":5876,"inlinee":5725,"source_line":1975,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":566,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":5876,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":566,"record_offset":6808,"parent_record_offset":6784,"ancestor_record_offset":5876,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":5876,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":6948,"parent_record_offset":6924,"ancestor_record_offset":5876,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":566,"record_offset":8932,"parent_record_offset":8908,"ancestor_record_offset":5876,"inlinee":7069,"source_line":1486,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":566,"record_offset":8956,"parent_record_offset":8932,"ancestor_record_offset":5876,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":7760,"parent_record_offset":7668,"ancestor_record_offset":5876,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":7784,"parent_record_offset":7760,"ancestor_record_offset":5876,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":9180,"parent_record_offset":9148,"ancestor_record_offset":5876,"inlinee":5796,"source_line":2580,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":6268,"parent_record_offset":6112,"ancestor_record_offset":5876,"inlinee":5726,"source_line":2629,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":7352,"parent_record_offset":7324,"ancestor_record_offset":5876,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":6144,"parent_record_offset":6112,"ancestor_record_offset":5876,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":9204,"parent_record_offset":9180,"ancestor_record_offset":5876,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":6292,"parent_record_offset":6268,"ancestor_record_offset":5876,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":6168,"parent_record_offset":6144,"ancestor_record_offset":5876,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":566,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":5876,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":6364,"parent_record_offset":6336,"ancestor_record_offset":5876,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":6392,"parent_record_offset":6364,"ancestor_record_offset":5876,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":7096,"parent_record_offset":6876,"ancestor_record_offset":5876,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":566,"record_offset":6428,"parent_record_offset":6392,"ancestor_record_offset":5876,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":5876,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6984,"parent_record_offset":6876,"ancestor_record_offset":5876,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":5876,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::write
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8908,"parent_record_offset":7960,"ancestor_record_offset":5876,"inlinee":6569,"source_line":632,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":7220,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":5876,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":5876,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6456,"parent_record_offset":6392,"ancestor_record_offset":5876,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::insert_no_grow
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":5876,"inlinee":6558,"source_line":1362,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8768,"parent_record_offset":7960,"ancestor_record_offset":5876,"inlinee":6567,"source_line":2501,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":5876,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8032,"parent_record_offset":8000,"ancestor_record_offset":5876,"inlinee":6560,"source_line":2111,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_insert_slot_in_group
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8212,"parent_record_offset":8032,"ancestor_record_offset":5876,"inlinee":6561,"source_line":1906,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::fix_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8376,"parent_record_offset":8032,"ancestor_record_offset":5876,"inlinee":6562,"source_line":1869,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_bucket_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8408,"parent_record_offset":8376,"ancestor_record_offset":5876,"inlinee":6563,"source_line":2816,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::prepare_insert_slot
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8000,"parent_record_offset":7960,"ancestor_record_offset":5876,"inlinee":6559,"source_line":2068,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8088,"parent_record_offset":8032,"ancestor_record_offset":5876,"inlinee":6542,"source_line":2610,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::set_ctrl_h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8668,"parent_record_offset":8000,"ancestor_record_offset":5876,"inlinee":6565,"source_line":2677,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":6756,"parent_record_offset":6724,"ancestor_record_offset":5876,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":6784,"parent_record_offset":6756,"ancestor_record_offset":5876,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":6844,"parent_record_offset":6724,"ancestor_record_offset":5876,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":566,"record_offset":6724,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6556,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":6876,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::is_full
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8432,"parent_record_offset":8408,"ancestor_record_offset":5876,"inlinee":6564,"source_line":115,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::special_is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":566,"record_offset":8880,"parent_record_offset":7960,"ancestor_record_offset":5876,"inlinee":6568,"source_line":127,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":6584,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":8464,"parent_record_offset":8376,"ancestor_record_offset":5876,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":6668,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":7128,"parent_record_offset":6504,"ancestor_record_offset":5876,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":566,"record_offset":8252,"parent_record_offset":8212,"ancestor_record_offset":5876,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcOccupiedEntry::into_mut
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":566,"record_offset":9028,"parent_record_offset":9004,"ancestor_record_offset":5876,"inlinee":6550,"source_line":412,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::rustc_entry::RustcVacantEntry::insert
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\rustc_entry.rs
//     evidence={"module":566,"record_offset":7936,"parent_record_offset":7912,"ancestor_record_offset":5876,"inlinee":6557,"source_line":574,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$17::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":5876,"inlinee":9020,"source_line":1332,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$26::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/rwlock.rs
//     evidence={"module":566,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":5876,"inlinee":9025,"source_line":1667,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::try_lock_shared_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":6112,"parent_record_offset":6080,"ancestor_record_offset":5876,"inlinee":5723,"source_line":508,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_exclusive
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":7668,"parent_record_offset":7640,"ancestor_record_offset":5876,"inlinee":5787,"source_line":67,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::lock_shared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":6080,"parent_record_offset":6052,"ancestor_record_offset":5876,"inlinee":5721,"source_line":107,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":5876,"inlinee":5795,"source_line":94,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_shared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_rwlock.rs
//     evidence={"module":566,"record_offset":7324,"parent_record_offset":7300,"ancestor_record_offset":5876,"inlinee":5735,"source_line":129,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::OccupiedEntry::into_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":9004,"parent_record_offset":7880,"ancestor_record_offset":5876,"inlinee":6548,"source_line":2927,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::VacantEntry::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":7912,"parent_record_offset":7880,"ancestor_record_offset":5876,"inlinee":6555,"source_line":3092,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::map_entry
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":566,"record_offset":7848,"parent_record_offset":7824,"ancestor_record_offset":5876,"inlinee":15015,"source_line":3182,"archive":"_pdb/A/modules/00566.jsonl.gz"}
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
// 首条位置: {"module":566,"record_offset":9652,"rva":7558832,"va":5376267952}
// 编译实例 RVAs: 0x7356b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] backtrace::capture::Backtrace::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t99670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376267952,"raw_hex":"e856850100","size":5,"soft_fail":false,"target_va":5376368480,"va":5376268805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26710662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376267952,"raw_hex":"e886929701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376268277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::format::vk_format_valid_aspect_flags
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t71659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376267952,"raw_hex":"e8eb170100","size":5,"soft_fail":false,"target_va":5376339712,"va":5376268048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rdi","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5376267952,"raw_hex":"ffd7","size":2,"soft_fail":false,"va":5376268789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::create_image_view
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":566,"record_offset":10496,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":30823,"source_line":2524,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageAspectFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":566,"record_offset":9884,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":30812,"source_line":27,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::ImageUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":566,"record_offset":9828,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":11135,"source_line":32,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$618::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":566,"record_offset":10236,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":30817,"source_line":389,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ImageViewCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":566,"record_offset":10292,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":30822,"source_line":2861,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":566,"record_offset":10264,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":566,"record_offset":10672,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":9856,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":566,"record_offset":10616,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::texture::view_desc_impl
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":9940,"parent_record_offset":9652,"ancestor_record_offset":9652,"inlinee":30815,"source_line":165,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::ImageAspectFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":566,"record_offset":9908,"parent_record_offset":9884,"ancestor_record_offset":9652,"inlinee":30813,"source_line":23,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":566,"record_offset":10528,"parent_record_offset":10496,"ancestor_record_offset":9652,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":566,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":9652,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":566,"record_offset":10576,"parent_record_offset":10552,"ancestor_record_offset":9652,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":566,"record_offset":10440,"parent_record_offset":10416,"ancestor_record_offset":9652,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":566,"record_offset":10344,"parent_record_offset":10320,"ancestor_record_offset":9652,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":566,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":9652,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":566,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":9652,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":10320,"parent_record_offset":10292,"ancestor_record_offset":9652,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":566,"record_offset":10392,"parent_record_offset":10368,"ancestor_record_offset":9652,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":10100,"parent_record_offset":9940,"ancestor_record_offset":9652,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":10008,"parent_record_offset":9940,"ancestor_record_offset":9652,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::error::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/error.rs
//     evidence={"module":566,"record_offset":10640,"parent_record_offset":10616,"ancestor_record_offset":9652,"inlinee":30763,"source_line":21,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":9652,"inlinee":30820,"source_line":202,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::view_desc_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":9652,"inlinee":30819,"source_line":181,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::view_desc_impl
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x785F
// 首条位置: {"module":566,"inlinee":30815}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":10100,"parent_record_offset":9940,"ancestor_record_offset":9940,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":566,"record_offset":10008,"parent_record_offset":9940,"ancestor_record_offset":9940,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":9940,"inlinee":30820,"source_line":202,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::texture::view_desc_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":9940,"inlinee":30819,"source_line":181,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::view_desc_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7863
// 首条位置: {"module":566,"inlinee":30819}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::texture::convert_image_type_to_view_type
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/texture.rs
//     evidence={"module":566,"record_offset":10064,"parent_record_offset":10040,"ancestor_record_offset":10040,"inlinee":30820,"source_line":202,"archive":"_pdb/A/modules/00566.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::managed::texture::convert_image_type_to_view_type
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 202
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7864
// 首条位置: {"module":566,"inlinee":30820}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 10
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::impl$15::fmt<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::handle::impl$15::fmt", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::handle::ManagedHandle", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::ManagedHandle<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResourceStore", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::resources::ManagedResourceStore<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::impl$5::dispose", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource", "arguments": ["rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>", "constructor": "rhapsody::raw::managed::resources::ManagedResource", "arguments": ["rhapsody::raw::managed::texture::ManagedTexture"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "append_only_vec::AppendOnlyVec::push<rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture> >", "constructor": "append_only_vec::AppendOnlyVec::push", "arguments": ["rhapsody::raw::managed::resources::ManagedResource<rhapsody::raw::managed::texture::ManagedTexture>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::ManagedTexture::new
// 信息 ID: 2a19370fc7461dae94208de2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0x26b379 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x26c5a4 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<ref$<slice2$<u8> > > @ 0xe9e669 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0xe9f894 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x1148c44 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> > @ 0x118d48b -> 0x734ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::iter::adapters::map::impl$2::fold<rhapsody::raw::texture::Texture,alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::texture::Texture,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::texture::Texture,alloc::alloc::Global,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<ash::vk::definitions::Image,alloc::alloc::Global>,rhapsody::raw::managed::swapchain::impl$2::new_impl::closure_env$5> > > > @ 0x1915f49 -> 0x734ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::ManagedTexture::view
// 信息 ID: 5be863c342428a79aeca513b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::get_vk_image_view @ 0x744927 -> 0x735040 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::image_view @ 0x745e92 -> 0x735040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::convert_image_type_to_view_type
// 信息 ID: 2d1088426d56b09a8aeffa30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=566, inline_record=10064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::create_vk_image_view
// 信息 ID: f4f7d1587b8d014c92de4efd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::texture::ManagedTexture::new @ 0x734d43 -> 0x7356b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::managed::texture::ManagedTexture::view @ 0x7352b8 -> 0x7356b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::impl$0::dispose
// 信息 ID: 9dfe16a5147bb605f9773965
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::managed::resources::impl$5::dispose<rhapsody::raw::managed::texture::ManagedTexture> @ 0x1178029 -> 0x734860 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::view_desc_impl
// 信息 ID: f24cc3deaa6e4ddcbccc2e4a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=566, inline_record=9940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::managed::texture::view_desc_impl::closure$0
// 信息 ID: d5a74be806fc4d39601a7469
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::managed::texture::view_desc_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::texture::create_vk_image_view; module=566, inline_record=10040; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
