// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/rg/resource_info.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2fc677178fa0a1874fed4c8050aacd4179c00b5e8ea93c9b89230d0a1e997f17
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: resource_info.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 12, 21, 52, 73, 195, 467, 558, 563
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use vk_sync_fork; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
// 记录形式: compiled_procedure; instances=1
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":12,"record_offset":18060,"rva":103568,"va":5368812688}
// 编译实例 RVAs: 0x19490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14049d3c0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4734107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e89b3c4800","size":5,"soft_fail":false,"target_va":5373547456,"va":5368813344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f520
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7430053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8a55f7100","size":5,"soft_fail":false,"target_va":5376242976,"va":5368812918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072f560
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7429593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8d95d7100","size":5,"soft_fail":false,"target_va":5376243040,"va":5368813442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: resource_info.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34780373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8d5b41202","size":5,"soft_fail":false,"target_va":5403593392,"va":5368813014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t34106819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8c36d0802","size":5,"soft_fail":false,"target_va":5402920704,"va":5368813880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2491920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e810062600","size":5,"soft_fail":false,"target_va":5371305408,"va":5368813483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t34165487","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8ef520902","size":5,"soft_fail":false,"target_va":5402979344,"va":5368813852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource::ResourceOrigin::ty
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7403599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e84ff87000","size":5,"soft_fail":false,"target_va":5376217248,"va":5368813644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::clear
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7428859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8fb5a7100","size":5,"soft_fail":false,"target_va":5376241600,"va":5368812736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7428624","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8105a7100","size":5,"soft_fail":false,"target_va":5376241824,"va":5368813195,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7428459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e86b597100","size":5,"soft_fail":false,"target_va":5376241648,"va":5368813184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7429532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e89c5d7100","size":5,"soft_fail":false,"target_va":5376242912,"va":5368813375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_info::image_access_mask_to_usage_flags
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7428896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e8205b7100","size":5,"soft_fail":false,"target_va":5376242608,"va":5368813707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7549223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5368812688,"raw_hex":"e827317300","size":5,"soft_fail":false,"target_va":5376362832,"va":5368813604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t7428622","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5368812688,"raw_hex":"e90e5a7100","size":5,"soft_fail":false,"target_va":5376242464,"va":5368813837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceInfo::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":18268,"parent_record_offset":18060,"ancestor_record_offset":18060,"inlinee":5426,"source_line":51,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":20440,"parent_record_offset":18060,"ancestor_record_offset":18060,"inlinee":5441,"source_line":121,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18596,"parent_record_offset":18568,"ancestor_record_offset":18060,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":18060,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":18060,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18532,"parent_record_offset":18504,"ancestor_record_offset":18060,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19252,"parent_record_offset":18452,"ancestor_record_offset":18060,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18452,"parent_record_offset":18416,"ancestor_record_offset":18060,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19408,"parent_record_offset":19348,"ancestor_record_offset":18060,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19380,"parent_record_offset":19348,"ancestor_record_offset":18060,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20696,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18504,"parent_record_offset":18452,"ancestor_record_offset":18060,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19348,"parent_record_offset":18268,"ancestor_record_offset":18060,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":21520,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20932,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20824,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18380,"parent_record_offset":18268,"ancestor_record_offset":18060,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18896,"parent_record_offset":18872,"ancestor_record_offset":18060,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19856,"parent_record_offset":19408,"ancestor_record_offset":18060,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19308,"parent_record_offset":18452,"ancestor_record_offset":18060,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":18060,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":12,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":18060,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":18060,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":19756,"parent_record_offset":19724,"ancestor_record_offset":18060,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":18840,"parent_record_offset":18812,"ancestor_record_offset":18060,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":12,"record_offset":20584,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":18676,"parent_record_offset":18644,"ancestor_record_offset":18060,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":18788,"parent_record_offset":18708,"ancestor_record_offset":18060,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":12,"record_offset":19724,"parent_record_offset":19692,"ancestor_record_offset":18060,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":12,"record_offset":19692,"parent_record_offset":19408,"ancestor_record_offset":18060,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":18644,"parent_record_offset":18452,"ancestor_record_offset":18060,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":18872,"parent_record_offset":18788,"ancestor_record_offset":18060,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":18324,"parent_record_offset":18268,"ancestor_record_offset":18060,"inlinee":5427,"source_line":604,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":18348,"parent_record_offset":18324,"ancestor_record_offset":18060,"inlinee":5428,"source_line":2074,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":12,"record_offset":21376,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":12,"record_offset":21340,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":18960,"parent_record_offset":18708,"ancestor_record_offset":18060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":18760,"parent_record_offset":18708,"ancestor_record_offset":18060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":18060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21548,"parent_record_offset":21520,"ancestor_record_offset":18060,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":20968,"parent_record_offset":20932,"ancestor_record_offset":18060,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21576,"parent_record_offset":21548,"ancestor_record_offset":18060,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21004,"parent_record_offset":20968,"ancestor_record_offset":18060,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":18060,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":20612,"parent_record_offset":20584,"ancestor_record_offset":18060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":21436,"parent_record_offset":20440,"ancestor_record_offset":18060,"inlinee":5446,"source_line":203,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$0::allocate::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":18812,"parent_record_offset":18788,"ancestor_record_offset":18060,"inlinee":5433,"source_line":55,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":18060,"inlinee":5448,"source_line":227,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::clear
// 记录形式: compiled_procedure; instances=1
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":612,"rva":7532480,"va":5376241600}
// 编译实例 RVAs: 0x72efc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":744,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":4125,"source_line":2175,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::allocate
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1532
// 首条位置: {"module":12,"inlinee":5426}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19348,"parent_record_offset":18268,"ancestor_record_offset":18268,"inlinee":5438,"source_line":2506,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18380,"parent_record_offset":18268,"ancestor_record_offset":18268,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":18324,"parent_record_offset":18268,"ancestor_record_offset":18268,"inlinee":5427,"source_line":604,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18596,"parent_record_offset":18568,"ancestor_record_offset":18268,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18568,"parent_record_offset":18532,"ancestor_record_offset":18268,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":19276,"parent_record_offset":19252,"ancestor_record_offset":18268,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":18532,"parent_record_offset":18504,"ancestor_record_offset":18268,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19252,"parent_record_offset":18452,"ancestor_record_offset":18268,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18452,"parent_record_offset":18416,"ancestor_record_offset":18268,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19408,"parent_record_offset":19348,"ancestor_record_offset":18268,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":19380,"parent_record_offset":19348,"ancestor_record_offset":18268,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18504,"parent_record_offset":18452,"ancestor_record_offset":18268,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":18896,"parent_record_offset":18872,"ancestor_record_offset":18268,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19856,"parent_record_offset":19408,"ancestor_record_offset":18268,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19308,"parent_record_offset":18452,"ancestor_record_offset":18268,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":12,"record_offset":19056,"parent_record_offset":19032,"ancestor_record_offset":18268,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":12,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":18268,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":19756,"parent_record_offset":19724,"ancestor_record_offset":18268,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":18840,"parent_record_offset":18812,"ancestor_record_offset":18268,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":18676,"parent_record_offset":18644,"ancestor_record_offset":18268,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":12,"record_offset":18788,"parent_record_offset":18708,"ancestor_record_offset":18268,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":12,"record_offset":19724,"parent_record_offset":19692,"ancestor_record_offset":18268,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":12,"record_offset":19692,"parent_record_offset":19408,"ancestor_record_offset":18268,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":18644,"parent_record_offset":18452,"ancestor_record_offset":18268,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":12,"record_offset":18872,"parent_record_offset":18788,"ancestor_record_offset":18268,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":12,"record_offset":18348,"parent_record_offset":18324,"ancestor_record_offset":18268,"inlinee":5428,"source_line":2074,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":18960,"parent_record_offset":18708,"ancestor_record_offset":18268,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":18760,"parent_record_offset":18708,"ancestor_record_offset":18268,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":18920,"parent_record_offset":18896,"ancestor_record_offset":18268,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":18708,"parent_record_offset":18676,"ancestor_record_offset":18268,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$0::allocate::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":18812,"parent_record_offset":18788,"ancestor_record_offset":18268,"inlinee":5433,"source_line":55,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$0::allocate::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1539
// 首条位置: {"module":12,"inlinee":5433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":18840,"parent_record_offset":18812,"ancestor_record_offset":18812,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":888,"rva":7532528,"va":5376241648}
// 编译实例 RVAs: 0x72eff0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26737546","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241648,"raw_hex":"e88afb9701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376241793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":1148,"parent_record_offset":888,"ancestor_record_offset":888,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":558,"record_offset":1044,"parent_record_offset":888,"ancestor_record_offset":888,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":558,"record_offset":1112,"parent_record_offset":1072,"ancestor_record_offset":888,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1176,"parent_record_offset":1148,"ancestor_record_offset":888,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":888,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":558,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":888,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":1332,"rva":7532704,"va":5376241824}
// 编译实例 RVAs: 0x72f0a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26737019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241824,"raw_hex":"e87bf99701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376242320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26736493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241824,"raw_hex":"e86df79701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376242446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] vk_sync_fork::get_access_info
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t120817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241824,"raw_hex":"e8f1d70100","size":5,"soft_fail":false,"target_va":5376362832,"va":5376242010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":1840,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":1936,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":1528,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":2132,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":30634,"source_line":45,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$354::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":1700,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":11121,"source_line":45,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":558,"record_offset":1784,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":558,"record_offset":1556,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::graph::ExportableResource::index
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":558,"record_offset":1728,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":7661,"source_line":37,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":558,"record_offset":2032,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":30629,"source_line":287,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::image_access_mask_to_usage_flags
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":558,"record_offset":1632,"parent_record_offset":1332,"ancestor_record_offset":1332,"inlinee":30628,"source_line":248,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":2072,"parent_record_offset":2032,"ancestor_record_offset":1332,"inlinee":30631,"source_line":27,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":1332,"inlinee":30632,"source_line":23,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":558,"record_offset":1668,"parent_record_offset":1632,"ancestor_record_offset":1332,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":558,"record_offset":1596,"parent_record_offset":1556,"ancestor_record_offset":1332,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1868,"parent_record_offset":1840,"ancestor_record_offset":1332,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1964,"parent_record_offset":1936,"ancestor_record_offset":1332,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1896,"parent_record_offset":1868,"ancestor_record_offset":1332,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":1992,"parent_record_offset":1964,"ancestor_record_offset":1332,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::graph::ExportableResource::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/graph.rs
//     evidence={"module":558,"record_offset":1752,"parent_record_offset":1728,"ancestor_record_offset":1332,"inlinee":7662,"source_line":29,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] vk_sync_fork::impl$5::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/vk-sync-rs/src/lib.rs
//     evidence={"module":558,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":1332,"inlinee":12595,"source_line":21,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 121
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1541
// 首条位置: {"module":12,"inlinee":5441}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20696,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":21520,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20932,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20824,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":12,"record_offset":21148,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":12,"record_offset":20584,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":12,"record_offset":21376,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":12,"record_offset":21340,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":5444,"source_line":1603,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":20852,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":21436,"parent_record_offset":20440,"ancestor_record_offset":20440,"inlinee":5446,"source_line":203,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":12,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":20440,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":12,"record_offset":20736,"parent_record_offset":20696,"ancestor_record_offset":20440,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":20440,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":12,"record_offset":20656,"parent_record_offset":20612,"ancestor_record_offset":20440,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":12,"record_offset":20792,"parent_record_offset":20696,"ancestor_record_offset":20440,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21548,"parent_record_offset":21520,"ancestor_record_offset":20440,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":20968,"parent_record_offset":20932,"ancestor_record_offset":20440,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21576,"parent_record_offset":21548,"ancestor_record_offset":20440,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":12,"record_offset":21004,"parent_record_offset":20968,"ancestor_record_offset":20440,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":12,"record_offset":20612,"parent_record_offset":20584,"ancestor_record_offset":20440,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$14::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":20440,"inlinee":5448,"source_line":227,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
// 记录形式: compiled_procedure; instances=1
// 源码行: 174
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":2260,"rva":7533344,"va":5376242464}
// 编译实例 RVAs: 0x72f320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26736786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376242464,"raw_hex":"e892f89701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376242553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26737529","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376242464,"raw_hex":"e879fb9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376242594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":558,"record_offset":2652,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":558,"record_offset":2500,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":558,"record_offset":2424,"parent_record_offset":2260,"ancestor_record_offset":2260,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":558,"record_offset":2464,"parent_record_offset":2444,"ancestor_record_offset":2260,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":558,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":2260,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":558,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2260,"inlinee":29273,"source_line":248,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":558,"record_offset":2444,"parent_record_offset":2424,"ancestor_record_offset":2260,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":558,"record_offset":2612,"parent_record_offset":2528,"ancestor_record_offset":2260,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":2260,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":558,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2260,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$20::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":558,"record_offset":2528,"parent_record_offset":2500,"ancestor_record_offset":2260,"inlinee":30636,"source_line":216,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::index
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 189
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7823
// 首条位置: {"module":563,"inlinee":30755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::try_index
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DDA
// 首条位置: {"module":52,"inlinee":7642}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::resource_info::ResourceAccessIndex::valid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":73,"record_offset":116660,"parent_record_offset":116636,"ancestor_record_offset":116636,"inlinee":5446,"source_line":203,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":73,"record_offset":116684,"parent_record_offset":116660,"ancestor_record_offset":116636,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$14::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":73,"record_offset":116708,"parent_record_offset":116684,"ancestor_record_offset":116636,"inlinee":5448,"source_line":227,"archive":"_pdb/A/modules/00073.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::ResourceAccessIndex::valid
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1546
// 首条位置: {"module":12,"inlinee":5446}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":12,"record_offset":21460,"parent_record_offset":21436,"ancestor_record_offset":21436,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource_info::impl$14::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource_info.rs
//     evidence={"module":12,"record_offset":21484,"parent_record_offset":21460,"ancestor_record_offset":21436,"inlinee":5448,"source_line":227,"archive":"_pdb/A/modules/00012.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$10::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":558,"record_offset":3140,"rva":7533856,"va":5376242976}
// 编译实例 RVAs: 0x72f520
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26737091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376242976,"raw_hex":"e8c3f99701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376243032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":558,"record_offset":3272,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":558,"record_offset":3320,"parent_record_offset":3140,"ancestor_record_offset":3140,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":558,"record_offset":3292,"parent_record_offset":3272,"ancestor_record_offset":3140,"inlinee":29273,"source_line":248,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$20::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77AC
// 首条位置: {"module":558,"inlinee":30636}
// 编译实例 RVAs: 0x72f560
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26737027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376243040,"raw_hex":"e883f99701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376243096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":558,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":558,"record_offset":2612,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$2::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":558,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2528,"inlinee":29273,"source_line":248,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$5::eq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 223
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3131
// 首条位置: {"module":195,"inlinee":12593}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::impl$14::eq
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1548
// 首条位置: {"module":12,"inlinee":5448}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::image_access_mask_to_usage_flags
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 248
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77A4
// 首条位置: {"module":558,"inlinee":30628}
// 编译实例 RVAs: 0x72f3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26736044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376242608,"raw_hex":"e8acf59701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376242895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":558,"record_offset":1668,"parent_record_offset":1632,"ancestor_record_offset":1632,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 287, 290
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77A5
// 首条位置: {"module":558,"inlinee":30629}
// 编译实例 RVAs: 0x72f4e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::AccessFlags::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":2072,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":30631,"source_line":27,"archive":"_pdb/A/modules/00558.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::AccessFlags::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":558,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":2032,"inlinee":30632,"source_line":23,"archive":"_pdb/A/modules/00558.jsonl.gz"}
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
// 完整结构及原始类型记录: resource_info.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceAccessIndex
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceAccessInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::rg::resource_info::ResourceAccessInfo", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>", "constructor": "rhapsody::rg::resource_info::ResourceInfo::calculate", "arguments": ["country_core::render::pass_builder::pass_resources::PassResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::rg::resource_info::ResourceInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::rg::resource_info::ResourceInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::rg::resource_info::ResourceLifetime
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >", "constructor": "core::iter::adapters::filter_map::FilterMap", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >", "rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::rg::resource::ResourceHandle", "core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<rhapsody::rg::resource_info::ResourceLifetime,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["rhapsody::rg::resource_info::ResourceLifetime", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: resource_info.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: resource_info.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 11; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>,bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>", "bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::wall::snapping::SnappingMode>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::in_progress_wall::InProgressWallResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<rhapsody::raw::physical_device::PhysicalDevice,alloc::alloc::Global> (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 候选参数类型: ["rhapsody::raw::physical_device::PhysicalDevice"]
// [文件级接口候选，具体函数归属 unknown] country_core::render::render_config::VideoMode (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] std::ffi::os_str::OsString (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] alloc::string::String (*)(ref$<winit::monitor::VideoMode>)
// 候选参数类型: ["ref$<winit::monitor::VideoMode>"]
// [文件级接口候选，具体函数归属 unknown] iced_core::rectangle::Rectangle<f32> (*)(ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >)
// 候选参数类型: ["ref$<enum2$<iced_graphics::primitive::Primitive<enum2$<iced_tiny_skia::primitive::Custom> > > >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(ref$<glam::f32::vec2::Vec2>)
// 候选参数类型: ["ref$<glam::f32::vec2::Vec2>"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::index
// 信息 ID: d8e13622a52e354d9740b911
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::compiled::pass_access_range_indices; module=563, inline_record=1140; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::compiled::pass_access_range_indices; module=563, inline_record=1164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::try_index
// 信息 ID: 4918bd89239b8cfca8a7f41d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=116636; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=117620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceAccessIndex::valid
// 信息 ID: d530e30f8d4cd5441b5554c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=12, inline_record=21436; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=116660; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=117644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::allocate
// 信息 ID: f93a755bfd2b85ee3d844149
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=12, inline_record=18268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>
// 信息 ID: 6c32e3486059d6270d001092
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] rhapsody::rg::resource_info::ResourceInfo::calculate -> ["country_core::render::pass_builder::pass_resources::PassResources"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::graph::RenderGraph::compile<country_core::render::pass_builder::pass_resources::PassResources> @ 0x669c0 -> 0x19490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::clear
// 信息 ID: f5a2016bdfa455a4bd2230f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x194c0 -> 0x72efc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports
// 信息 ID: cf87303ba3d868befa6d089e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x1968b -> 0x72f0a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extract_creation_flags
// 信息 ID: b85079e30677fb5e5a655f4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x19680 -> 0x72eff0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::extract_pass_resource_access
// 信息 ID: 6a0c8a18880908c3e485220d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=12, inline_record=20440; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains
// 信息 ID: 6943d03c24416424e48320ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x1990d -> 0x72f320 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::buffer_access_mask_to_usage_flags
// 信息 ID: 2e98ab12dcdc2528dd81d961
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x1973f -> 0x72f4e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=558, inline_record=2032; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::image_access_mask_to_usage_flags
// 信息 ID: 4d0d7dbcaca49ad1e5a43ee2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x1988b -> 0x72f3b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::extend_usage_flags_for_exports; module=558, inline_record=1632; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$0::allocate::closure$0
// 信息 ID: df9f50f770cd6ec4b12fdbf4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::rg::resource_info::impl$0::allocate", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=12, inline_record=18812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$10::from
// 信息 ID: 4a995a04aebaff9fa61fb804
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 27 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x19576 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x197e6 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x19801 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74eb4 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15bcb0 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1a70e6 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<alloc::string::String> > @ 0x1a7919 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x553485 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608c64 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x60a776 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists @ 0x611629 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2 @ 0x612643 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::new_impl @ 0x660732 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0x692a2f -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x6af164 -> 0x72f520 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::clear_buffer<country_core::render::nani::buffers::NaniNextVisibilityBitmaskSsbo> @ 0x6af3a4 -> 0x72f520 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$14::eq
// 信息 ID: a9643f5ab12ec3eccf4721e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources>; module=12, inline_record=21484; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=116708; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::rg::resource::ResourceHandle,core::iter::adapters::filter_map::FilterMap<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rhapsody::rg::resource_info::ResourceLifetime> >,rhapsody::rg::compiled::impl$1::new::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources> > >; module=73, inline_record=117692; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$20::from
// 信息 ID: e74dc7eca0900d74549f84f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x19782 -> 0x72f560 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x198c4 -> 0x72f560 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_info::ResourceInfo::calculate<country_core::render::pass_builder::pass_resources::PassResources> @ 0x198cf -> 0x72f560 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::resource_info::ResourceInfo::link_up_next_access_chains; module=558, inline_record=2528; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::rg::resource_info::impl$5::eq
// 信息 ID: c6d290929854343520acb0ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=195, inline_record=18968; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_builder::PassBuilder::add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>; module=467, inline_record=10668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
