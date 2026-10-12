// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/decorator/src/collision_detection.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: f0e77fb26ea7c7ea5a8673c21fac8fbaaad5e078f149d4ce9a08723b7794311b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: collision_detection.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1286, 2774, 2775
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2775,"record_offset":5920,"rva":25003600,"va":5393712720}
// 编译实例 RVAs: 0x17d8650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d7d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712720,"raw_hex":"e86ff4ffff","size":5,"soft_fail":false,"target_va":5393710352,"va":5393713308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d7f70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712720,"raw_hex":"e83ff8ffff","size":5,"soft_fail":false,"target_va":5393710960,"va":5393712940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9206655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393712720,"raw_hex":"e87f7b8c00","size":5,"soft_fail":false,"target_va":5402919776,"va":5393713116,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_decorator::collision_detection::intersect_wall_decorators_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":2775,"record_offset":6180,"parent_record_offset":5920,"ancestor_record_offset":5920,"inlinee":89961,"source_line":52,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":7612,"parent_record_offset":7580,"ancestor_record_offset":5920,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":5920,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7580,"parent_record_offset":6684,"ancestor_record_offset":5920,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":5920,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7552,"parent_record_offset":6684,"ancestor_record_offset":5920,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7672,"parent_record_offset":6684,"ancestor_record_offset":5920,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7644,"parent_record_offset":6684,"ancestor_record_offset":5920,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":6608,"parent_record_offset":6180,"ancestor_record_offset":5920,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2775,"record_offset":6660,"parent_record_offset":6608,"ancestor_record_offset":5920,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":5920,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":5920,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2775,"record_offset":6728,"parent_record_offset":6684,"ancestor_record_offset":5920,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2775,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5920,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5920,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":7512,"parent_record_offset":6776,"ancestor_record_offset":5920,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":5920,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2775,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":5920,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2775,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":5920,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":5920,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":5920,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":5920,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":5920,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":7168,"parent_record_offset":6920,"ancestor_record_offset":5920,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6376,"parent_record_offset":6288,"ancestor_record_offset":5920,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6920,"parent_record_offset":6872,"ancestor_record_offset":5920,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":5920,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6348,"parent_record_offset":6288,"ancestor_record_offset":5920,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6320,"parent_record_offset":6288,"ancestor_record_offset":5920,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6288,"parent_record_offset":6260,"ancestor_record_offset":5920,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7068,"parent_record_offset":6984,"ancestor_record_offset":5920,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7104,"parent_record_offset":7068,"ancestor_record_offset":5920,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7032,"parent_record_offset":6984,"ancestor_record_offset":5920,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":6984,"parent_record_offset":6920,"ancestor_record_offset":5920,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6872,"parent_record_offset":6840,"ancestor_record_offset":5920,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":6436,"parent_record_offset":6376,"ancestor_record_offset":5920,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":6492,"parent_record_offset":6436,"ancestor_record_offset":5920,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":6208,"parent_record_offset":6180,"ancestor_record_offset":5920,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":6808,"parent_record_offset":6776,"ancestor_record_offset":5920,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2775,"record_offset":9316,"rva":25004400,"va":5393713520}
// 编译实例 RVAs: 0x17d8970
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d81e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2177","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713520,"raw_hex":"e87ff7ffff","size":5,"soft_fail":false,"target_va":5393711584,"va":5393713756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417d8420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: collision_detection.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713520,"raw_hex":"e84ff8ffff","size":5,"soft_fail":false,"target_va":5393712160,"va":5393714124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9205839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393713520,"raw_hex":"e84f788c00","size":5,"soft_fail":false,"target_va":5402919776,"va":5393713932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] system_decorator::collision_detection::intersect_wall_decorators_impl
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":2775,"record_offset":9584,"parent_record_offset":9316,"ancestor_record_offset":9316,"inlinee":89961,"source_line":52,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":11016,"parent_record_offset":10984,"ancestor_record_offset":9316,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":11100,"parent_record_offset":11076,"ancestor_record_offset":9316,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":10984,"parent_record_offset":10088,"ancestor_record_offset":9316,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":10088,"parent_record_offset":10064,"ancestor_record_offset":9316,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":10956,"parent_record_offset":10088,"ancestor_record_offset":9316,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":11076,"parent_record_offset":10088,"ancestor_record_offset":9316,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":11048,"parent_record_offset":10088,"ancestor_record_offset":9316,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":10012,"parent_record_offset":9584,"ancestor_record_offset":9316,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2775,"record_offset":10064,"parent_record_offset":10012,"ancestor_record_offset":9316,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":9868,"parent_record_offset":9840,"ancestor_record_offset":9316,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":9928,"parent_record_offset":9896,"ancestor_record_offset":9316,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2775,"record_offset":10132,"parent_record_offset":10088,"ancestor_record_offset":9316,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2775,"record_offset":10036,"parent_record_offset":10012,"ancestor_record_offset":9316,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":10156,"parent_record_offset":10132,"ancestor_record_offset":9316,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":10916,"parent_record_offset":10180,"ancestor_record_offset":9316,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":10180,"parent_record_offset":10156,"ancestor_record_offset":9316,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2775,"record_offset":10532,"parent_record_offset":10508,"ancestor_record_offset":9316,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2775,"record_offset":9812,"parent_record_offset":9780,"ancestor_record_offset":9316,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":10620,"parent_record_offset":10596,"ancestor_record_offset":9316,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":10596,"parent_record_offset":10572,"ancestor_record_offset":9316,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":9636,"parent_record_offset":9612,"ancestor_record_offset":9316,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":10244,"parent_record_offset":10212,"ancestor_record_offset":9316,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":10572,"parent_record_offset":10324,"ancestor_record_offset":9316,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":9780,"parent_record_offset":9692,"ancestor_record_offset":9316,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":10324,"parent_record_offset":10276,"ancestor_record_offset":9316,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":9664,"parent_record_offset":9636,"ancestor_record_offset":9316,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":9752,"parent_record_offset":9692,"ancestor_record_offset":9316,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":9724,"parent_record_offset":9692,"ancestor_record_offset":9316,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":9692,"parent_record_offset":9664,"ancestor_record_offset":9316,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":10472,"parent_record_offset":10388,"ancestor_record_offset":9316,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":10508,"parent_record_offset":10472,"ancestor_record_offset":9316,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":10436,"parent_record_offset":10388,"ancestor_record_offset":9316,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":10388,"parent_record_offset":10324,"ancestor_record_offset":9316,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":10276,"parent_record_offset":10244,"ancestor_record_offset":9316,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":9840,"parent_record_offset":9780,"ancestor_record_offset":9316,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":9896,"parent_record_offset":9840,"ancestor_record_offset":9316,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":9612,"parent_record_offset":9584,"ancestor_record_offset":9316,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":10212,"parent_record_offset":10180,"ancestor_record_offset":9316,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators_impl
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F69
// 首条位置: {"module":2775,"inlinee":89961}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":6608,"parent_record_offset":6180,"ancestor_record_offset":6180,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":6208,"parent_record_offset":6180,"ancestor_record_offset":6180,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":7612,"parent_record_offset":7580,"ancestor_record_offset":6180,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2775,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6180,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7580,"parent_record_offset":6684,"ancestor_record_offset":6180,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":6180,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7552,"parent_record_offset":6684,"ancestor_record_offset":6180,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7672,"parent_record_offset":6684,"ancestor_record_offset":6180,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2775,"record_offset":7644,"parent_record_offset":6684,"ancestor_record_offset":6180,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":2775,"record_offset":6660,"parent_record_offset":6608,"ancestor_record_offset":6180,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":6464,"parent_record_offset":6436,"ancestor_record_offset":6180,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2775,"record_offset":6524,"parent_record_offset":6492,"ancestor_record_offset":6180,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":2775,"record_offset":6728,"parent_record_offset":6684,"ancestor_record_offset":6180,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":2775,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":6180,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":6180,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":7512,"parent_record_offset":6776,"ancestor_record_offset":6180,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2775,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":6180,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":2775,"record_offset":7128,"parent_record_offset":7104,"ancestor_record_offset":6180,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2775,"record_offset":6408,"parent_record_offset":6376,"ancestor_record_offset":6180,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":7216,"parent_record_offset":7192,"ancestor_record_offset":6180,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2775,"record_offset":7192,"parent_record_offset":7168,"ancestor_record_offset":6180,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":6180,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":2775,"record_offset":6840,"parent_record_offset":6808,"ancestor_record_offset":6180,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":7168,"parent_record_offset":6920,"ancestor_record_offset":6180,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6376,"parent_record_offset":6288,"ancestor_record_offset":6180,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6920,"parent_record_offset":6872,"ancestor_record_offset":6180,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6260,"parent_record_offset":6232,"ancestor_record_offset":6180,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6348,"parent_record_offset":6288,"ancestor_record_offset":6180,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6320,"parent_record_offset":6288,"ancestor_record_offset":6180,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6288,"parent_record_offset":6260,"ancestor_record_offset":6180,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7068,"parent_record_offset":6984,"ancestor_record_offset":6180,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7104,"parent_record_offset":7068,"ancestor_record_offset":6180,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":7032,"parent_record_offset":6984,"ancestor_record_offset":6180,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":2775,"record_offset":6984,"parent_record_offset":6920,"ancestor_record_offset":6180,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":2775,"record_offset":6872,"parent_record_offset":6840,"ancestor_record_offset":6180,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":6436,"parent_record_offset":6376,"ancestor_record_offset":6180,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=8
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":2775,"record_offset":6492,"parent_record_offset":6436,"ancestor_record_offset":6180,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":2775,"record_offset":6808,"parent_record_offset":6776,"ancestor_record_offset":6180,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::intersect_wall_decorators_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=8
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15F62
// 首条位置: {"module":2775,"inlinee":89954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2775,"record_offset":1192,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":21730,"source_line":261,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2775,"record_offset":1728,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$46::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":2775,"record_offset":1444,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":5676,"source_line":18,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::abs
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2775,"record_offset":1472,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::impl$1::snapping_direction
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/lib.rs
//     evidence={"module":2775,"record_offset":1416,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":89958,"source_line":277,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure$1
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/merge_proposal_wall_decorators.rs
//     evidence={"module":2775,"record_offset":852,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":89956,"source_line":197,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":2775,"record_offset":3940,"parent_record_offset":3868,"ancestor_record_offset":3868,"inlinee":89959,"source_line":370,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceRegion::intersects
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":2775,"record_offset":1500,"parent_record_offset":780,"ancestor_record_offset":780,"inlinee":89933,"source_line":61,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":2775,"record_offset":876,"parent_record_offset":852,"ancestor_record_offset":780,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":2775,"record_offset":1316,"parent_record_offset":1292,"ancestor_record_offset":780,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":2775,"record_offset":1760,"parent_record_offset":1728,"ancestor_record_offset":780,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2775,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":780,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2775,"record_offset":1092,"parent_record_offset":1064,"ancestor_record_offset":780,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2775,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":780,"inlinee":4965,"source_line":215,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$10::slice_contains::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":2775,"record_offset":1124,"parent_record_offset":1036,"ancestor_record_offset":780,"inlinee":4968,"source_line":216,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":2775,"record_offset":988,"parent_record_offset":852,"ancestor_record_offset":780,"inlinee":4963,"source_line":2524,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2775,"record_offset":1036,"parent_record_offset":1012,"ancestor_record_offset":780,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2775,"record_offset":1064,"parent_record_offset":1036,"ancestor_record_offset":780,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$25::eq
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":2775,"record_offset":900,"parent_record_offset":876,"ancestor_record_offset":780,"inlinee":21651,"source_line":84,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2775,"record_offset":1352,"parent_record_offset":1232,"ancestor_record_offset":780,"inlinee":21733,"source_line":102,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2775,"record_offset":1232,"parent_record_offset":1192,"ancestor_record_offset":780,"inlinee":21731,"source_line":217,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$3::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":2775,"record_offset":1292,"parent_record_offset":1260,"ancestor_record_offset":780,"inlinee":21753,"source_line":220,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":2775,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":780,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$0::regions_intersect
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2775,"record_offset":1552,"parent_record_offset":1500,"ancestor_record_offset":780,"inlinee":89935,"source_line":39,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$1::distance
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2775,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":780,"inlinee":89937,"source_line":58,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] utils::comparative_space::impl$2::distance
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/comparative_space.rs
//     evidence={"module":2775,"record_offset":1664,"parent_record_offset":1640,"ancestor_record_offset":780,"inlinee":60012,"source_line":91,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":2775,"record_offset":1376,"parent_record_offset":1352,"ancestor_record_offset":780,"inlinee":21736,"source_line":65,"archive":"_pdb/A/modules/02775.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEB72
// 首条位置: {"module":1286,"inlinee":60274}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1286,"record_offset":3320,"parent_record_offset":3288,"ancestor_record_offset":3288,"inlinee":21347,"source_line":205,"archive":"_pdb/A/modules/01286.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1286,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":3288,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01286.jsonl.gz"}
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
// 完整结构及原始类型记录: collision_detection.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::collision_detection::DecoratorIntersection
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "ref$<system_decorator::collision_detection::DecoratorIntersection>", "constructor": "ref$", "arguments": ["system_decorator::collision_detection::DecoratorIntersection"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >", "constructor": "tuple$", "arguments": ["ref$<system_decorator::collision_detection::DecoratorIntersection>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0>", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<system_decorator::collision_detection::DecoratorIntersection> >", "system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::collision_detection::DecoratorWorldSpace
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: collision_detection.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: collision_detection.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
// 信息 ID: a079c907350b29ee9646a9b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1286, inline_record=3288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>
// 信息 ID: 8f62dafe361b773f3c5c1ee9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::collision_detection::intersect_wall_decorators -> ["system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner @ 0x17d96f6 -> 0x17d8650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>
// 信息 ID: ce2e31b0c0f1815eb0ecf544
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] system_decorator::collision_detection::intersect_wall_decorators -> ["system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0xd0a462 -> 0x17d8970 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators_impl
// 信息 ID: 81fcc55b0266e7b23978f394
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; module=2775, inline_record=6180; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1>; module=2775, inline_record=7744; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; module=2775, inline_record=9584; 不是独立调用边。
// [PDB 内联宿主] system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; module=2775, inline_record=11148; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::collision_detection::intersect_wall_decorators_impl::closure$0
// 信息 ID: bcdcbb613b87911e98f81b55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::collision_detection::intersect_wall_decorators_impl", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=780; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::merge_proposal_wall_decorators::propose_wall_decorator_merge_position_inner::closure_env$1> >; module=2775, inline_record=2348; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=3868; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=2775, inline_record=5132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
