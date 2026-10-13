// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/resources/history/path_edit.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: cc6a7565b378a9d1f9ff992bd98f0880fa7073bf2d4bff1ba4d895a46ddd8a55
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: path_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 167, 432, 490, 1334, 1385, 1440
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"PDB_registry_path_only","version":"1.0.85"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 6, 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6182
// 首条位置: {"module":432,"inlinee":24962}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":14524,"parent_record_offset":14480,"ancestor_record_offset":14480,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1385,"record_offset":14500,"parent_record_offset":14480,"ancestor_record_offset":14480,"inlinee":24962,"source_line":17,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 记录形式: compiled_procedure; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":167,"record_offset":6396,"rva":2934784,"va":5371643904}
// 编译实例 RVAs: 0x2cc800, 0x5cae50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140017e80
//     owner=unresolved_namespace; count=1
//     target_aliases=13 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2837608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e898b3d4ff","size":5,"soft_fail":false,"target_va":5368807040,"va":5371644643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140152290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1549975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e86959e8ff","size":5,"soft_fail":false,"target_va":5370094224,"va":5371644194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140152520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1549389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8b35be8ff","size":5,"soft_fail":false,"target_va":5370094880,"va":5371644264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401565c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1532810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8769ce8ff","size":5,"soft_fail":false,"target_va":5370111424,"va":5371644229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140156ba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1531236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e89ca2e8ff","size":5,"soft_fail":false,"target_va":5370112928,"va":5371644159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140361670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t609442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8a24c0900","size":5,"soft_fail":false,"target_va":5372253808,"va":5371644361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1105689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e819df1000","size":5,"soft_fail":false,"target_va":5372749696,"va":5371644002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1105317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8a5dd1000","size":5,"soft_fail":false,"target_va":5372749712,"va":5371644390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405cac50
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e828fbffff","size":5,"soft_fail":false,"target_va":5374782544,"va":5374783779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ad5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4066551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8f70c3e00","size":5,"soft_fail":false,"target_va":5375710656,"va":5371644100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d99a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1108369","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e891e91000","size":5,"soft_fail":false,"target_va":5375891872,"va":5374783498,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406fa580
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4381496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e838db4200","size":5,"soft_fail":false,"target_va":5376025984,"va":5371644483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141096f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11321269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e8b5bfac00","size":5,"soft_fail":false,"target_va":5386104640,"va":5374783366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109afc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11337851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e87b00ad00","size":5,"soft_fail":false,"target_va":5386121152,"va":5374783296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109d290
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11346728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e82823ad00","size":5,"soft_fail":false,"target_va":5386130064,"va":5374783331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14109d840
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11348114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374783056,"raw_hex":"e89228ad00","size":5,"soft_fail":false,"target_va":5386131520,"va":5374783401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31949316","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e80482e701","size":5,"soft_fail":false,"target_va":5403593360,"va":5371644039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31333636","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e8041dde01","size":5,"soft_fail":false,"target_va":5402978144,"va":5371644503,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5882625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371643904,"raw_hex":"e801c35900","size":5,"soft_fail":false,"target_va":5377527120,"va":5371644490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":167,"record_offset":7180,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7908,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":167,"record_offset":7208,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":11978,"source_line":17,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":167,"record_offset":6596,"parent_record_offset":6396,"ancestor_record_offset":6396,"inlinee":11974,"source_line":6,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6396,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6396,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":6396,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6940,"parent_record_offset":6848,"ancestor_record_offset":6396,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":7628,"parent_record_offset":7572,"ancestor_record_offset":6396,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":167,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":6396,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":167,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":6396,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":6396,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6848,"parent_record_offset":6820,"ancestor_record_offset":6396,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":6396,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7060,"parent_record_offset":6820,"ancestor_record_offset":6396,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":6396,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6396,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":6396,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":6396,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6792,"parent_record_offset":6756,"ancestor_record_offset":6396,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6672,"parent_record_offset":6636,"ancestor_record_offset":6396,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6884,"parent_record_offset":6848,"ancestor_record_offset":6396,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":6396,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":167,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6396,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":167,"record_offset":7432,"parent_record_offset":7344,"ancestor_record_offset":6396,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":167,"record_offset":8048,"parent_record_offset":8016,"ancestor_record_offset":6396,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":167,"record_offset":7100,"parent_record_offset":6756,"ancestor_record_offset":6396,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7268,"parent_record_offset":7208,"ancestor_record_offset":6396,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":6396,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7016,"parent_record_offset":6988,"ancestor_record_offset":6396,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":167,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":6396,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::impl$1::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":432,"record_offset":99236,"parent_record_offset":99212,"ancestor_record_offset":98432,"inlinee":24961,"source_line":46,"archive":"_pdb/A/modules/00432.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":167,"record_offset":6636,"parent_record_offset":6596,"ancestor_record_offset":6396,"inlinee":11976,"source_line":7,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF30E
// 首条位置: {"module":1334,"inlinee":62222}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$3::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EC6
// 首条位置: {"module":167,"inlinee":11974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::raster_stroke::impl$2::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":167,"record_offset":6636,"parent_record_offset":6596,"ancestor_record_offset":6596,"inlinee":11976,"source_line":7,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6964,"parent_record_offset":6940,"ancestor_record_offset":6596,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6988,"parent_record_offset":6964,"ancestor_record_offset":6596,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6940,"parent_record_offset":6848,"ancestor_record_offset":6596,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6848,"parent_record_offset":6820,"ancestor_record_offset":6596,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6820,"parent_record_offset":6792,"ancestor_record_offset":6596,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7060,"parent_record_offset":6820,"ancestor_record_offset":6596,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6756,"parent_record_offset":6728,"ancestor_record_offset":6596,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6728,"parent_record_offset":6700,"ancestor_record_offset":6596,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":6700,"parent_record_offset":6672,"ancestor_record_offset":6596,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6792,"parent_record_offset":6756,"ancestor_record_offset":6596,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6672,"parent_record_offset":6636,"ancestor_record_offset":6596,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6884,"parent_record_offset":6848,"ancestor_record_offset":6596,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6908,"parent_record_offset":6884,"ancestor_record_offset":6596,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":167,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":6596,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":167,"record_offset":7100,"parent_record_offset":6756,"ancestor_record_offset":6596,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7016,"parent_record_offset":6988,"ancestor_record_offset":6596,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x618F
// 首条位置: {"module":1440,"inlinee":24975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":220184,"parent_record_offset":220160,"ancestor_record_offset":220160,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":220208,"parent_record_offset":220184,"ancestor_record_offset":220160,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220232,"parent_record_offset":220208,"ancestor_record_offset":220160,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220256,"parent_record_offset":220232,"ancestor_record_offset":220160,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10445
// 首条位置: {"module":1440,"inlinee":66629}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":220060,"parent_record_offset":220036,"ancestor_record_offset":220036,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":220792,"parent_record_offset":220768,"ancestor_record_offset":220036,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":220768,"parent_record_offset":220712,"ancestor_record_offset":220036,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":220740,"parent_record_offset":220712,"ancestor_record_offset":220036,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":220712,"parent_record_offset":220688,"ancestor_record_offset":220036,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":220208,"parent_record_offset":220184,"ancestor_record_offset":220036,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":220640,"parent_record_offset":220132,"ancestor_record_offset":220036,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220232,"parent_record_offset":220208,"ancestor_record_offset":220036,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220256,"parent_record_offset":220232,"ancestor_record_offset":220036,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":220184,"parent_record_offset":220160,"ancestor_record_offset":220036,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1440,"record_offset":220160,"parent_record_offset":220132,"ancestor_record_offset":220036,"inlinee":24975,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":220132,"parent_record_offset":220108,"ancestor_record_offset":220036,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":220084,"parent_record_offset":220060,"ancestor_record_offset":220036,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":220108,"parent_record_offset":220084,"ancestor_record_offset":220036,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x617A
// 首条位置: {"module":432,"inlinee":24954}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10443
// 首条位置: {"module":1440,"inlinee":66627}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":221352,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":222892,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":221656,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":222668,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":6896,"source_line":23,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":218568,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":9789,"source_line":1936,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":221076,"parent_record_offset":218536,"ancestor_record_offset":218536,"inlinee":9791,"source_line":1944,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":219252,"parent_record_offset":219228,"ancestor_record_offset":218536,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":219228,"parent_record_offset":219172,"ancestor_record_offset":218536,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":219872,"parent_record_offset":219844,"ancestor_record_offset":218536,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":219932,"parent_record_offset":219872,"ancestor_record_offset":218536,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":218728,"parent_record_offset":218704,"ancestor_record_offset":218536,"inlinee":7109,"source_line":1719,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1440,"record_offset":218780,"parent_record_offset":218752,"ancestor_record_offset":218536,"inlinee":7033,"source_line":1721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1440,"record_offset":218804,"parent_record_offset":218780,"ancestor_record_offset":218536,"inlinee":7034,"source_line":1161,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":219200,"parent_record_offset":219172,"ancestor_record_offset":218536,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":219384,"parent_record_offset":219360,"ancestor_record_offset":218536,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":219172,"parent_record_offset":219144,"ancestor_record_offset":218536,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":219360,"parent_record_offset":219332,"ancestor_record_offset":218536,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1440,"record_offset":219332,"parent_record_offset":219300,"ancestor_record_offset":218536,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1440,"record_offset":220208,"parent_record_offset":220184,"ancestor_record_offset":218536,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":219964,"parent_record_offset":219872,"ancestor_record_offset":218536,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1440,"record_offset":221192,"parent_record_offset":221168,"ancestor_record_offset":218536,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":219532,"parent_record_offset":219504,"ancestor_record_offset":218536,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":218752,"parent_record_offset":218728,"ancestor_record_offset":218536,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1440,"record_offset":221168,"parent_record_offset":221132,"ancestor_record_offset":218536,"inlinee":4134,"source_line":1695,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":219020,"parent_record_offset":218648,"ancestor_record_offset":218536,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1440,"record_offset":218880,"parent_record_offset":218856,"ancestor_record_offset":218536,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":219900,"parent_record_offset":219872,"ancestor_record_offset":218536,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":221248,"parent_record_offset":221192,"ancestor_record_offset":218536,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220232,"parent_record_offset":220208,"ancestor_record_offset":218536,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1440,"record_offset":220256,"parent_record_offset":220232,"ancestor_record_offset":218536,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1440,"record_offset":218832,"parent_record_offset":218804,"ancestor_record_offset":218536,"inlinee":7035,"source_line":721,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1440,"record_offset":218856,"parent_record_offset":218832,"ancestor_record_offset":218536,"inlinee":7036,"source_line":251,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1440,"record_offset":220184,"parent_record_offset":220160,"ancestor_record_offset":218536,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1440,"record_offset":220160,"parent_record_offset":220132,"ancestor_record_offset":218536,"inlinee":24975,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1440,"record_offset":220036,"parent_record_offset":220012,"ancestor_record_offset":218536,"inlinee":66629,"source_line":17,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1440,"record_offset":222740,"parent_record_offset":222716,"ancestor_record_offset":218536,"inlinee":6850,"source_line":36,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":222828,"parent_record_offset":222804,"ancestor_record_offset":218536,"inlinee":6849,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":222716,"parent_record_offset":222692,"ancestor_record_offset":218536,"inlinee":6874,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":218608,"parent_record_offset":218568,"ancestor_record_offset":218536,"inlinee":9790,"source_line":1846,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":221104,"parent_record_offset":221076,"ancestor_record_offset":218536,"inlinee":9792,"source_line":1863,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":220132,"parent_record_offset":220108,"ancestor_record_offset":218536,"inlinee":9763,"source_line":1529,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1440,"record_offset":221308,"parent_record_offset":221284,"ancestor_record_offset":218536,"inlinee":7237,"source_line":232,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":220012,"parent_record_offset":218648,"ancestor_record_offset":218536,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":222592,"parent_record_offset":222568,"ancestor_record_offset":218536,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":221612,"parent_record_offset":221588,"ancestor_record_offset":218536,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":221948,"parent_record_offset":221924,"ancestor_record_offset":218536,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1440,"record_offset":218704,"parent_record_offset":218648,"ancestor_record_offset":218536,"inlinee":25385,"source_line":458,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":220084,"parent_record_offset":220060,"ancestor_record_offset":218536,"inlinee":50427,"source_line":1148,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/macros.rs
//     evidence={"module":1440,"record_offset":220060,"parent_record_offset":220036,"ancestor_record_offset":218536,"inlinee":66600,"source_line":126,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":220108,"parent_record_offset":220084,"ancestor_record_offset":218536,"inlinee":50429,"source_line":1301,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":218648,"parent_record_offset":218608,"ancestor_record_offset":218536,"inlinee":50437,"source_line":610,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":221132,"parent_record_offset":221104,"ancestor_record_offset":218536,"inlinee":50438,"source_line":626,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1041A
// 首条位置: {"module":1440,"inlinee":66586}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":48936,"parent_record_offset":48316,"ancestor_record_offset":48316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":48352,"parent_record_offset":48316,"ancestor_record_offset":48316,"inlinee":9782,"source_line":1753,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":49124,"parent_record_offset":49100,"ancestor_record_offset":48316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1440,"record_offset":49100,"parent_record_offset":48992,"ancestor_record_offset":48316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":49028,"parent_record_offset":48992,"ancestor_record_offset":48316,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1440,"record_offset":48992,"parent_record_offset":48964,"ancestor_record_offset":48316,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1440,"record_offset":48440,"parent_record_offset":48408,"ancestor_record_offset":48316,"inlinee":4425,"source_line":202,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1440,"record_offset":49064,"parent_record_offset":49028,"ancestor_record_offset":48316,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1440,"record_offset":48964,"parent_record_offset":48936,"ancestor_record_offset":48316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":48500,"parent_record_offset":48440,"ancestor_record_offset":48316,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1440,"record_offset":48468,"parent_record_offset":48440,"ancestor_record_offset":48316,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1440,"record_offset":48588,"parent_record_offset":48408,"ancestor_record_offset":48316,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":48380,"parent_record_offset":48352,"ancestor_record_offset":48316,"inlinee":9784,"source_line":1724,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1440,"record_offset":48532,"parent_record_offset":48408,"ancestor_record_offset":48316,"inlinee":6888,"source_line":790,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":48556,"parent_record_offset":48532,"ancestor_record_offset":48316,"inlinee":6873,"source_line":1150,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":48856,"parent_record_offset":48832,"ancestor_record_offset":48316,"inlinee":6847,"source_line":72,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1440,"record_offset":49372,"parent_record_offset":49348,"ancestor_record_offset":48316,"inlinee":6870,"source_line":134,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1440,"record_offset":48408,"parent_record_offset":48380,"ancestor_record_offset":48316,"inlinee":24451,"source_line":575,"archive":"_pdb/A/modules/01440.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2ECA
// 首条位置: {"module":167,"inlinee":11978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7268,"parent_record_offset":7208,"ancestor_record_offset":7208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":167,"record_offset":7240,"parent_record_offset":7208,"ancestor_record_offset":7208,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":7652,"parent_record_offset":7628,"ancestor_record_offset":7208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":7628,"parent_record_offset":7572,"ancestor_record_offset":7208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":167,"record_offset":7344,"parent_record_offset":7320,"ancestor_record_offset":7208,"inlinee":5352,"source_line":188,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$33::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\map.rs
//     evidence={"module":167,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":7208,"inlinee":5354,"source_line":1640,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7600,"parent_record_offset":7572,"ancestor_record_offset":7208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":7572,"parent_record_offset":7544,"ancestor_record_offset":7208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":167,"record_offset":7432,"parent_record_offset":7344,"ancestor_record_offset":7208,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":7208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":7208,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$1::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6181
// 首条位置: {"module":432,"inlinee":24961}
// 编译实例 RVAs: 0x6ad5c0
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
// 完整结构及原始类型记录: path_edit.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::history::path_edit::RasterEdit
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "serde_json::value::ser::impl$1::serialize_newtype_variant", "arguments": ["alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$50::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::impl$50::fmt", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$211::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: path_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: path_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] void (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(ref_mut$<bevy_ecs::world::World>)
// 候选参数类型: ["ref_mut$<bevy_ecs::world::World>"]
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music_audio::MusicAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::sheep_audio::PlaySheepReactSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::sheep_audio::SheepAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>,bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>,bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>", "bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>", "bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::wall_audio_fmod::WallAudioState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::CmdSuppressMusic>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::wall::calc_wall_bend::WallBendChangedEvent>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::WallBendAudioParamReset>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize
// 信息 ID: c542d944330d1aa21ec47459
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >; module=1385, inline_record=14480; 不是独立调用边。
// [PDB 内联宿主] serde::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >; module=1385, inline_record=14500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 35b9a0eeab1a3895240342ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=220160; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 741e168dafa8963a53730757
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=220036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 3a0fe110b438a4364afca6de
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=432, inline_record=90524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 415cbe2a1b8d1e4fdeb8a983
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=218536; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: ea6e4d22e5e03072baa20296
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1440, inline_record=48316; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize
// 信息 ID: 31c00cdb19833e870e4e2ae4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=167, inline_record=7208; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=432, inline_record=99268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>
// 信息 ID: cc9ec299370021a85627a841
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::resources::history::path_edit::_::impl$0::serialize -> ["serde_json::value::ser::Serializer"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> > @ 0x14ee6e -> 0x2cc800 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::ser::impl$1::serialize_newtype_variant<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> > @ 0x109549e -> 0x5cae50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$1::from
// 信息 ID: 96bb7674ab3bcd5826fa8c3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer> @ 0x2cc8c4 -> 0x6ad5c0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=432, inline_record=99236; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$2::fmt
// 信息 ID: 0722f8e7f9fd61cbf0896145
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$50::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>; module=1334, inline_record=62900; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$3::clone
// 信息 ID: 45ecd2fa1535fef2c02cec2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=167, inline_record=6596; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<serde_json::value::ser::Serializer>; module=432, inline_record=98632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
