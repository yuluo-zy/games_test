// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/rt/bvh.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 79105ef2b0377b6c1bb17245cd4750601cd4922553322218570a4ba17ac36ce0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: bvh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 468, 1308, 1321
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
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":468,"record_offset":10028,"rva":6627008,"va":5375336128}
// 编译实例 RVAs: 0x651ec0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2586532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e85c88d8ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375336223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2587277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e87385d8ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375336984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e1ff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t589021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8ddfc0800","size":5,"soft_fail":false,"target_va":5375926256,"va":5375337230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140854520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2106415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e82f242000","size":5,"soft_fail":false,"target_va":5377443104,"va":5375336684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d789e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7497788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e83c687200","size":5,"soft_fail":false,"target_va":5382834656,"va":5375336863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d81590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: bvh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7533504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8c0f37200","size":5,"soft_fail":false,"target_va":5382870416,"va":5375336907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27641035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8cbc4a501","size":5,"soft_fail":false,"target_va":5402978144,"va":5375337104,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2190024","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8c86a2100","size":5,"soft_fail":false,"target_va":5377527120,"va":5375337091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2016964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8c4c61e00","size":5,"soft_fail":false,"target_va":5377353776,"va":5375336807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] obvhs::cwbvh::builder::build_cwbvh_from_tris
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t11317758","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e8feb1ac00","size":5,"soft_fail":false,"target_va":5386654400,"va":5375336637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2106421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375336128,"raw_hex":"e835242000","size":5,"soft_fail":false,"target_va":5377442944,"va":5375336518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":468,"record_offset":11576,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":468,"record_offset":11604,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":10188,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":11912,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":468,"record_offset":11660,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::rt::bvh::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":468,"record_offset":11468,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":26661,"source_line":132,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":468,"record_offset":11632,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":468,"record_offset":11492,"parent_record_offset":10028,"ancestor_record_offset":10028,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":10528,"parent_record_offset":10504,"ancestor_record_offset":10028,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":10552,"parent_record_offset":10528,"ancestor_record_offset":10028,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":12076,"parent_record_offset":12052,"ancestor_record_offset":10028,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":10504,"parent_record_offset":10412,"ancestor_record_offset":10028,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":468,"record_offset":12052,"parent_record_offset":11960,"ancestor_record_offset":10028,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":468,"record_offset":11988,"parent_record_offset":11960,"ancestor_record_offset":10028,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":468,"record_offset":10412,"parent_record_offset":10384,"ancestor_record_offset":10028,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":468,"record_offset":10384,"parent_record_offset":10356,"ancestor_record_offset":10028,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":468,"record_offset":10624,"parent_record_offset":10384,"ancestor_record_offset":10028,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":468,"record_offset":11960,"parent_record_offset":11936,"ancestor_record_offset":10028,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10696,"parent_record_offset":10668,"ancestor_record_offset":10028,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10328,"parent_record_offset":10284,"ancestor_record_offset":10028,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10356,"parent_record_offset":10328,"ancestor_record_offset":10028,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":10220,"parent_record_offset":10188,"ancestor_record_offset":10028,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":468,"record_offset":11272,"parent_record_offset":11240,"ancestor_record_offset":10028,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":468,"record_offset":11364,"parent_record_offset":11272,"ancestor_record_offset":10028,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":468,"record_offset":10668,"parent_record_offset":10284,"ancestor_record_offset":10028,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":468,"record_offset":10252,"parent_record_offset":10220,"ancestor_record_offset":10028,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":468,"record_offset":10284,"parent_record_offset":10252,"ancestor_record_offset":10028,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":468,"record_offset":10448,"parent_record_offset":10412,"ancestor_record_offset":10028,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":468,"record_offset":10472,"parent_record_offset":10448,"ancestor_record_offset":10028,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":468,"record_offset":10912,"parent_record_offset":10848,"ancestor_record_offset":10028,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":468,"record_offset":10752,"parent_record_offset":10724,"ancestor_record_offset":10028,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":468,"record_offset":10820,"parent_record_offset":10780,"ancestor_record_offset":10028,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":468,"record_offset":11792,"parent_record_offset":11768,"ancestor_record_offset":10028,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":10724,"parent_record_offset":10696,"ancestor_record_offset":10028,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":468,"record_offset":11240,"parent_record_offset":10820,"ancestor_record_offset":10028,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":468,"record_offset":12016,"parent_record_offset":11988,"ancestor_record_offset":10028,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":11936,"parent_record_offset":11912,"ancestor_record_offset":10028,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":468,"record_offset":11708,"parent_record_offset":11684,"ancestor_record_offset":10028,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":468,"record_offset":11404,"parent_record_offset":10780,"ancestor_record_offset":10028,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":10580,"parent_record_offset":10552,"ancestor_record_offset":10028,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":468,"record_offset":11312,"parent_record_offset":11272,"ancestor_record_offset":10028,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":468,"record_offset":11684,"parent_record_offset":11660,"ancestor_record_offset":10028,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":468,"record_offset":10780,"parent_record_offset":10752,"ancestor_record_offset":10028,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":11516,"parent_record_offset":11492,"ancestor_record_offset":10028,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":468,"record_offset":11540,"parent_record_offset":11516,"ancestor_record_offset":10028,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::rt::bvh::build_gpu_bvh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/rt/bvh.rs
//     evidence={"module":468,"record_offset":10848,"parent_record_offset":10820,"ancestor_record_offset":10028,"inlinee":26663,"source_line":53,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":10028,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10028,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6827
// 首条位置: {"module":468,"inlinee":26663}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":468,"record_offset":10912,"parent_record_offset":10848,"ancestor_record_offset":10848,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":10976,"parent_record_offset":10944,"ancestor_record_offset":10848,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$45::from
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":468,"record_offset":10944,"parent_record_offset":10912,"ancestor_record_offset":10848,"inlinee":14678,"source_line":1358,"archive":"_pdb/A/modules/00468.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF054
// 首条位置: {"module":1308,"inlinee":61524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":169892,"parent_record_offset":169856,"ancestor_record_offset":169856,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":169924,"parent_record_offset":169892,"ancestor_record_offset":169856,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":169956,"parent_record_offset":169924,"ancestor_record_offset":169856,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 117
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF07C
// 首条位置: {"module":1308,"inlinee":61564}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::rt::bvh::impl$1::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 132
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6825
// 首条位置: {"module":468,"inlinee":26661}
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
// 完整结构及原始类型记录: bvh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::rt::bvh::GpuBlBvh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::rt::bvh::GpuBlBvh>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::rt::bvh::GpuBlBvh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: bvh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: bvh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh
// 信息 ID: e2b06e4c25c757f983b3332d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::rt::bvh_builder::bvh_builder_worker @ 0x5845ba -> 0x651ec0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$0
// 信息 ID: d727aa8ef66b308fa1ad1ff7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=468, inline_record=10848; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$1
// 信息 ID: c6360f7cdef0e288bf815c77
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::GpuRtTriangle,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::rt::GpuRtTriangle,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$1> >; module=1308, inline_record=169856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::build_gpu_bvh::closure$2
// 信息 ID: 9ae2d36cc3eb83458078f7d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::rt::bvh::build_gpu_bvh", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<u32,core::iter::adapters::map::Map<core::slice::iter::Iter<u32>,country_core::resources::rt::bvh::build_gpu_bvh::closure_env$2> >; module=1308, inline_record=301524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::rt::bvh::impl$1::default
// 信息 ID: 079fc50570c0ab56f3841371
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::rt::bvh::build_gpu_bvh; module=468, inline_record=11468; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
