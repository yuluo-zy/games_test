// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/resources/history/path_edit.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: e41ae7974331735a4822d100e70e9cc91a92daf8d468f714b0c3c2308f86ce10
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: path_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 181, 559, 593, 1630, 1673
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.200"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde","source":"PDB_registry_path_only","version":"1.0.200"},{"name":"serde_json","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.85"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1673,"record_offset":20820,"rva":24020240,"va":5392729360}
// 编译实例 RVAs: 0x16e8510
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17648073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e837b6f2fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5392729556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17648133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e8fbb5f2fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392729632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14159f130
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1348652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e8d46bebff","size":5,"soft_fail":false,"target_va":5391380784,"va":5392729431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19117565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e8fdb52301","size":5,"soft_fail":false,"target_va":5411847168,"va":5392729598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18811267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e883091f01","size":5,"soft_fail":false,"target_va":5411540992,"va":5392729720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9880632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392729360,"raw_hex":"e8c83b69ff","size":5,"soft_fail":false,"target_va":5382848896,"va":5392729523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":21036,"parent_record_offset":20820,"ancestor_record_offset":20820,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1673,"record_offset":21012,"parent_record_offset":20820,"ancestor_record_offset":20820,"inlinee":76306,"source_line":19,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":20820,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21404,"parent_record_offset":21376,"ancestor_record_offset":20820,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":20820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21352,"parent_record_offset":21328,"ancestor_record_offset":20820,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21904,"parent_record_offset":21880,"ancestor_record_offset":20820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21820,"parent_record_offset":21792,"ancestor_record_offset":20820,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":20820,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":20820,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21288,"parent_record_offset":21200,"ancestor_record_offset":20820,"inlinee":7968,"source_line":1945,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21200,"parent_record_offset":21164,"ancestor_record_offset":20820,"inlinee":29461,"source_line":2113,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21540,"parent_record_offset":21288,"ancestor_record_offset":20820,"inlinee":7971,"source_line":1977,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":20820,"inlinee":7975,"source_line":1952,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$25::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21328,"parent_record_offset":21288,"ancestor_record_offset":20820,"inlinee":29463,"source_line":2117,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$69::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21164,"parent_record_offset":21136,"ancestor_record_offset":20820,"inlinee":29460,"source_line":3746,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\mod.rs
//     evidence={"module":1673,"record_offset":21880,"parent_record_offset":21792,"ancestor_record_offset":20820,"inlinee":9414,"source_line":384,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1673,"record_offset":21232,"parent_record_offset":21200,"ancestor_record_offset":20820,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1673,"record_offset":21256,"parent_record_offset":21232,"ancestor_record_offset":20820,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1673,"record_offset":21136,"parent_record_offset":21100,"ancestor_record_offset":20820,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1673,"record_offset":21692,"parent_record_offset":21164,"ancestor_record_offset":20820,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1673,"record_offset":21848,"parent_record_offset":21820,"ancestor_record_offset":20820,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1673,"record_offset":21072,"parent_record_offset":21036,"ancestor_record_offset":20820,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21720,"parent_record_offset":21164,"ancestor_record_offset":20820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1673,"record_offset":21568,"parent_record_offset":21540,"ancestor_record_offset":20820,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21432,"parent_record_offset":21404,"ancestor_record_offset":20820,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21592,"parent_record_offset":21568,"ancestor_record_offset":20820,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":21476,"parent_record_offset":21288,"ancestor_record_offset":20820,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1673,"record_offset":21100,"parent_record_offset":21072,"ancestor_record_offset":20820,"inlinee":32050,"source_line":35,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 记录形式: compiled_procedure; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":181,"record_offset":60376,"rva":4757232,"va":5373466352}
// 编译实例 RVAs: 0x4896f0, 0x16e8340
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b5870
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1916853","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e84bc0e2ff","size":5,"soft_fail":false,"target_va":5371549808,"va":5373466656,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bbdb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t206359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e817260300","size":5,"soft_fail":false,"target_va":5373672880,"va":5373466516,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bc390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t207828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e8d42b0300","size":5,"soft_fail":false,"target_va":5373674384,"va":5373466551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404bdc80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t214181","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e8a5440300","size":5,"soft_fail":false,"target_va":5373680768,"va":5373466582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404c0820
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t225318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e826700300","size":5,"soft_fail":false,"target_va":5373691936,"va":5373466613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1614806","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e8d6a31800","size":5,"soft_fail":false,"target_va":5375081504,"va":5373466693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a976b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6348632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e858df6000","size":5,"soft_fail":false,"target_va":5379815088,"va":5373466451,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f4520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4144981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392728896,"raw_hex":"e8abc0c0ff","size":5,"soft_fail":false,"target_va":5388584224,"va":5392729200,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141586090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1450907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392728896,"raw_hex":"e865dce9ff","size":5,"soft_fail":false,"target_va":5391278224,"va":5392729126,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1442492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392728896,"raw_hex":"e844fde9ff","size":5,"soft_fail":false,"target_va":5391286608,"va":5392729095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141588e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1439178","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392728896,"raw_hex":"e8360aeaff","size":5,"soft_fail":false,"target_va":5391289984,"va":5392729157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14158c570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1425017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392728896,"raw_hex":"e88741eaff","size":5,"soft_fail":false,"target_va":5391304048,"va":5392729060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37851499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373466352,"raw_hex":"e86b914102","size":5,"soft_fail":false,"target_va":5411318240,"va":5373466736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":181,"record_offset":60848,"parent_record_offset":60376,"ancestor_record_offset":60376,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":181,"record_offset":61712,"parent_record_offset":60376,"ancestor_record_offset":60376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":181,"record_offset":60876,"parent_record_offset":60376,"ancestor_record_offset":60376,"inlinee":14119,"source_line":19,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":181,"record_offset":60668,"parent_record_offset":60376,"ancestor_record_offset":60376,"inlinee":14115,"source_line":8,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":61924,"parent_record_offset":61900,"ancestor_record_offset":60376,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":181,"record_offset":61900,"parent_record_offset":61808,"ancestor_record_offset":60376,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61300,"parent_record_offset":61268,"ancestor_record_offset":60376,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61168,"parent_record_offset":61140,"ancestor_record_offset":60376,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61836,"parent_record_offset":61808,"ancestor_record_offset":60376,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61808,"parent_record_offset":61784,"ancestor_record_offset":60376,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61228,"parent_record_offset":61196,"ancestor_record_offset":60376,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61348,"parent_record_offset":61324,"ancestor_record_offset":60376,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61324,"parent_record_offset":61300,"ancestor_record_offset":60376,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61196,"parent_record_offset":61168,"ancestor_record_offset":60376,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61784,"parent_record_offset":61760,"ancestor_record_offset":60376,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":60744,"parent_record_offset":60708,"ancestor_record_offset":60376,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61100,"parent_record_offset":61072,"ancestor_record_offset":60376,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61268,"parent_record_offset":61100,"ancestor_record_offset":60376,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61044,"parent_record_offset":61016,"ancestor_record_offset":60376,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61388,"parent_record_offset":61100,"ancestor_record_offset":60376,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61140,"parent_record_offset":61100,"ancestor_record_offset":60376,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":181,"record_offset":61072,"parent_record_offset":61044,"ancestor_record_offset":60376,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":181,"record_offset":61868,"parent_record_offset":61836,"ancestor_record_offset":60376,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":181,"record_offset":61736,"parent_record_offset":61712,"ancestor_record_offset":60376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60780,"parent_record_offset":60744,"ancestor_record_offset":60376,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60804,"parent_record_offset":60780,"ancestor_record_offset":60376,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":181,"record_offset":60708,"parent_record_offset":60668,"ancestor_record_offset":60376,"inlinee":14117,"source_line":11,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60960,"parent_record_offset":60932,"ancestor_record_offset":60376,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61616,"parent_record_offset":61588,"ancestor_record_offset":60376,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60932,"parent_record_offset":60904,"ancestor_record_offset":60376,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60904,"parent_record_offset":60876,"ancestor_record_offset":60376,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61588,"parent_record_offset":61560,"ancestor_record_offset":60376,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61560,"parent_record_offset":60876,"ancestor_record_offset":60376,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61448,"parent_record_offset":60876,"ancestor_record_offset":60376,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":181,"record_offset":61016,"parent_record_offset":60988,"ancestor_record_offset":60376,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":181,"record_offset":60988,"parent_record_offset":60960,"ancestor_record_offset":60376,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8349
// 首条位置: {"module":593,"inlinee":33609}
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
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3723
// 首条位置: {"module":181,"inlinee":14115}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::raster_stroke::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":181,"record_offset":60708,"parent_record_offset":60668,"ancestor_record_offset":60668,"inlinee":14117,"source_line":11,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":181,"record_offset":60744,"parent_record_offset":60708,"ancestor_record_offset":60668,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60780,"parent_record_offset":60744,"ancestor_record_offset":60668,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":181,"record_offset":60804,"parent_record_offset":60780,"ancestor_record_offset":60668,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12A12
// 首条位置: {"module":1673,"inlinee":76306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12255
// 首条位置: {"module":1630,"inlinee":74325}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":383836,"parent_record_offset":383808,"ancestor_record_offset":383808,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":383860,"parent_record_offset":383836,"ancestor_record_offset":383808,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383884,"parent_record_offset":383860,"ancestor_record_offset":383808,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383908,"parent_record_offset":383884,"ancestor_record_offset":383808,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12253
// 首条位置: {"module":1630,"inlinee":74323}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":383676,"parent_record_offset":383648,"ancestor_record_offset":383648,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":384468,"parent_record_offset":384444,"ancestor_record_offset":383648,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":384444,"parent_record_offset":384388,"ancestor_record_offset":383648,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":384416,"parent_record_offset":384388,"ancestor_record_offset":383648,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":384388,"parent_record_offset":384364,"ancestor_record_offset":383648,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":384364,"parent_record_offset":384340,"ancestor_record_offset":383648,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":383860,"parent_record_offset":383836,"ancestor_record_offset":383648,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":384292,"parent_record_offset":383772,"ancestor_record_offset":383648,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383884,"parent_record_offset":383860,"ancestor_record_offset":383648,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383908,"parent_record_offset":383884,"ancestor_record_offset":383648,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":383836,"parent_record_offset":383808,"ancestor_record_offset":383648,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1630,"record_offset":383808,"parent_record_offset":383772,"ancestor_record_offset":383648,"inlinee":74325,"source_line":19,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":383772,"parent_record_offset":383740,"ancestor_record_offset":383648,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":383708,"parent_record_offset":383676,"ancestor_record_offset":383648,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":383740,"parent_record_offset":383708,"ancestor_record_offset":383648,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D48
// 首条位置: {"module":1673,"inlinee":32072}
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
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12251
// 首条位置: {"module":1630,"inlinee":74321}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":385060,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":386604,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":385364,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::__private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":386380,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":6608,"source_line":23,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":382136,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":11498,"source_line":1936,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":384784,"parent_record_offset":382096,"ancestor_record_offset":382096,"inlinee":11499,"source_line":1944,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":382952,"parent_record_offset":382928,"ancestor_record_offset":382096,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":382928,"parent_record_offset":382872,"ancestor_record_offset":382096,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":382296,"parent_record_offset":382272,"ancestor_record_offset":382096,"inlinee":9910,"source_line":1759,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::impl$36::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\map.rs
//     evidence={"module":1630,"record_offset":382348,"parent_record_offset":382320,"ancestor_record_offset":382096,"inlinee":9912,"source_line":1761,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::into_key_val
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\collections\btree\node.rs
//     evidence={"module":1630,"record_offset":382372,"parent_record_offset":382348,"ancestor_record_offset":382096,"inlinee":9913,"source_line":1171,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":383308,"parent_record_offset":383284,"ancestor_record_offset":382096,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":382900,"parent_record_offset":382872,"ancestor_record_offset":382096,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":382872,"parent_record_offset":382848,"ancestor_record_offset":382096,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":383356,"parent_record_offset":383332,"ancestor_record_offset":382096,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":383332,"parent_record_offset":383308,"ancestor_record_offset":382096,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":382848,"parent_record_offset":382824,"ancestor_record_offset":382096,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":383284,"parent_record_offset":383260,"ancestor_record_offset":382096,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1630,"record_offset":383260,"parent_record_offset":383004,"ancestor_record_offset":382096,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1630,"record_offset":383860,"parent_record_offset":383836,"ancestor_record_offset":382096,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1630,"record_offset":384900,"parent_record_offset":384876,"ancestor_record_offset":382096,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":383216,"parent_record_offset":383188,"ancestor_record_offset":382096,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":382320,"parent_record_offset":382296,"ancestor_record_offset":382096,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1630,"record_offset":384876,"parent_record_offset":384840,"ancestor_record_offset":382096,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1630,"record_offset":382488,"parent_record_offset":382400,"ancestor_record_offset":382096,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":382716,"parent_record_offset":382216,"ancestor_record_offset":382096,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":382512,"parent_record_offset":382488,"ancestor_record_offset":382096,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":384956,"parent_record_offset":384900,"ancestor_record_offset":382096,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383884,"parent_record_offset":383860,"ancestor_record_offset":382096,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1630,"record_offset":383908,"parent_record_offset":383884,"ancestor_record_offset":382096,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1630,"record_offset":382400,"parent_record_offset":382372,"ancestor_record_offset":382096,"inlinee":9914,"source_line":698,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":382456,"parent_record_offset":382432,"ancestor_record_offset":382096,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1630,"record_offset":382432,"parent_record_offset":382400,"ancestor_record_offset":382096,"inlinee":5468,"source_line":261,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1630,"record_offset":383836,"parent_record_offset":383808,"ancestor_record_offset":382096,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1630,"record_offset":383808,"parent_record_offset":383772,"ancestor_record_offset":382096,"inlinee":74325,"source_line":19,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1630,"record_offset":383648,"parent_record_offset":383616,"ancestor_record_offset":382096,"inlinee":74323,"source_line":19,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/private/de.rs
//     evidence={"module":1630,"record_offset":386452,"parent_record_offset":386428,"ancestor_record_offset":382096,"inlinee":5158,"source_line":36,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":386540,"parent_record_offset":386516,"ancestor_record_offset":382096,"inlinee":6619,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::__private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":386428,"parent_record_offset":386404,"ancestor_record_offset":382096,"inlinee":6672,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":382176,"parent_record_offset":382136,"ancestor_record_offset":382096,"inlinee":6600,"source_line":1846,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":384812,"parent_record_offset":384784,"ancestor_record_offset":382096,"inlinee":6605,"source_line":1863,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":383772,"parent_record_offset":383740,"ancestor_record_offset":382096,"inlinee":11493,"source_line":1529,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/ignored_any.rs
//     evidence={"module":1630,"record_offset":385016,"parent_record_offset":384992,"ancestor_record_offset":382096,"inlinee":6518,"source_line":232,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":383616,"parent_record_offset":382216,"ancestor_record_offset":382096,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":386304,"parent_record_offset":386280,"ancestor_record_offset":382096,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":385320,"parent_record_offset":385296,"ancestor_record_offset":382096,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":385660,"parent_record_offset":385636,"ancestor_record_offset":382096,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::map::impl$26::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/map.rs
//     evidence={"module":1630,"record_offset":382272,"parent_record_offset":382216,"ancestor_record_offset":382096,"inlinee":26608,"source_line":458,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":383708,"parent_record_offset":383676,"ancestor_record_offset":382096,"inlinee":61149,"source_line":1148,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$17::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/macros.rs
//     evidence={"module":1630,"record_offset":383676,"parent_record_offset":383648,"ancestor_record_offset":382096,"inlinee":70669,"source_line":126,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$22::deserialize_any
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":383740,"parent_record_offset":383708,"ancestor_record_offset":382096,"inlinee":61151,"source_line":1301,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_key_seed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":382216,"parent_record_offset":382176,"ancestor_record_offset":382096,"inlinee":61155,"source_line":610,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$9::next_value_seed
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":384840,"parent_record_offset":384812,"ancestor_record_offset":382096,"inlinee":33409,"source_line":626,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12219
// 首条位置: {"module":1630,"inlinee":74265}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":137676,"parent_record_offset":136940,"ancestor_record_offset":136940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_direct] serde::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":136980,"parent_record_offset":136940,"ancestor_record_offset":136940,"inlinee":11489,"source_line":1753,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":137912,"parent_record_offset":137888,"ancestor_record_offset":136940,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1630,"record_offset":137888,"parent_record_offset":137776,"ancestor_record_offset":136940,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":137816,"parent_record_offset":137776,"ancestor_record_offset":136940,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":137776,"parent_record_offset":137740,"ancestor_record_offset":136940,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1630,"record_offset":137740,"parent_record_offset":137708,"ancestor_record_offset":136940,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1630,"record_offset":137056,"parent_record_offset":137028,"ancestor_record_offset":136940,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1630,"record_offset":137856,"parent_record_offset":137816,"ancestor_record_offset":136940,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":137708,"parent_record_offset":137676,"ancestor_record_offset":136940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":137128,"parent_record_offset":137056,"ancestor_record_offset":136940,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":137156,"parent_record_offset":137056,"ancestor_record_offset":136940,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1630,"record_offset":137096,"parent_record_offset":137056,"ancestor_record_offset":136940,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1630,"record_offset":137180,"parent_record_offset":137156,"ancestor_record_offset":136940,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1630,"record_offset":137272,"parent_record_offset":137028,"ancestor_record_offset":136940,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":137004,"parent_record_offset":136980,"ancestor_record_offset":136940,"inlinee":6504,"source_line":1724,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/mod.rs
//     evidence={"module":1630,"record_offset":137216,"parent_record_offset":137028,"ancestor_record_offset":136940,"inlinee":5767,"source_line":790,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":137240,"parent_record_offset":137216,"ancestor_record_offset":136940,"inlinee":6469,"source_line":1150,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":137596,"parent_record_offset":137572,"ancestor_record_offset":136940,"inlinee":6537,"source_line":72,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde::de::impls::impl$76::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde","version":"1.0.200","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.200/src/de/impls.rs
//     evidence={"module":1630,"record_offset":138232,"parent_record_offset":138208,"ancestor_record_offset":136940,"inlinee":5592,"source_line":134,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$7::next_element_seed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/value/de.rs
//     evidence={"module":1630,"record_offset":137028,"parent_record_offset":137004,"ancestor_record_offset":136940,"inlinee":29591,"source_line":575,"archive":"_pdb/B/modules/01630.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3727
// 首条位置: {"module":181,"inlinee":14119}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60904,"parent_record_offset":60876,"ancestor_record_offset":60876,"inlinee":5727,"source_line":409,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61560,"parent_record_offset":60876,"ancestor_record_offset":60876,"inlinee":5728,"source_line":774,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61448,"parent_record_offset":60876,"ancestor_record_offset":60876,"inlinee":5447,"source_line":746,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61300,"parent_record_offset":61268,"ancestor_record_offset":60876,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61168,"parent_record_offset":61140,"ancestor_record_offset":60876,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61228,"parent_record_offset":61196,"ancestor_record_offset":60876,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61348,"parent_record_offset":61324,"ancestor_record_offset":60876,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61324,"parent_record_offset":61300,"ancestor_record_offset":60876,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":181,"record_offset":61196,"parent_record_offset":61168,"ancestor_record_offset":60876,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61100,"parent_record_offset":61072,"ancestor_record_offset":60876,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61268,"parent_record_offset":61100,"ancestor_record_offset":60876,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61044,"parent_record_offset":61016,"ancestor_record_offset":60876,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61388,"parent_record_offset":61100,"ancestor_record_offset":60876,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":181,"record_offset":61140,"parent_record_offset":61100,"ancestor_record_offset":60876,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":181,"record_offset":61072,"parent_record_offset":61044,"ancestor_record_offset":60876,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60960,"parent_record_offset":60932,"ancestor_record_offset":60876,"inlinee":5420,"source_line":1849,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61616,"parent_record_offset":61588,"ancestor_record_offset":60876,"inlinee":5435,"source_line":1859,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":60932,"parent_record_offset":60904,"ancestor_record_offset":60876,"inlinee":5434,"source_line":386,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.85","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.85/src/ser.rs
//     evidence={"module":181,"record_offset":61588,"parent_record_offset":61560,"ancestor_record_offset":60876,"inlinee":5730,"source_line":720,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":181,"record_offset":61016,"parent_record_offset":60988,"ancestor_record_offset":60876,"inlinee":5424,"source_line":493,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\impls.rs
//     evidence={"module":181,"record_offset":60988,"parent_record_offset":60960,"ancestor_record_offset":60876,"inlinee":5422,"source_line":77,"archive":"_pdb/B/modules/00181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7D32
// 首条位置: {"module":1673,"inlinee":32050}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1673,"record_offset":21136,"parent_record_offset":21100,"ancestor_record_offset":21100,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21376,"parent_record_offset":21352,"ancestor_record_offset":21100,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21404,"parent_record_offset":21376,"ancestor_record_offset":21100,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21928,"parent_record_offset":21904,"ancestor_record_offset":21100,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21352,"parent_record_offset":21328,"ancestor_record_offset":21100,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1673,"record_offset":21904,"parent_record_offset":21880,"ancestor_record_offset":21100,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21820,"parent_record_offset":21792,"ancestor_record_offset":21100,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21792,"parent_record_offset":21768,"ancestor_record_offset":21100,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1673,"record_offset":21768,"parent_record_offset":21744,"ancestor_record_offset":21100,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21288,"parent_record_offset":21200,"ancestor_record_offset":21100,"inlinee":7968,"source_line":1945,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21200,"parent_record_offset":21164,"ancestor_record_offset":21100,"inlinee":29461,"source_line":2113,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21540,"parent_record_offset":21288,"ancestor_record_offset":21100,"inlinee":7971,"source_line":1977,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$22::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":21100,"inlinee":7975,"source_line":1952,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$25::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21328,"parent_record_offset":21288,"ancestor_record_offset":21100,"inlinee":29463,"source_line":2117,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$69::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1673,"record_offset":21164,"parent_record_offset":21136,"ancestor_record_offset":21100,"inlinee":29460,"source_line":3746,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\mod.rs
//     evidence={"module":1673,"record_offset":21880,"parent_record_offset":21792,"ancestor_record_offset":21100,"inlinee":9414,"source_line":384,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1673,"record_offset":21232,"parent_record_offset":21200,"ancestor_record_offset":21100,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1673,"record_offset":21256,"parent_record_offset":21232,"ancestor_record_offset":21100,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1673,"record_offset":21692,"parent_record_offset":21164,"ancestor_record_offset":21100,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1673,"record_offset":21848,"parent_record_offset":21820,"ancestor_record_offset":21100,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21720,"parent_record_offset":21164,"ancestor_record_offset":21100,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1673,"record_offset":21568,"parent_record_offset":21540,"ancestor_record_offset":21100,"inlinee":5670,"source_line":1505,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21432,"parent_record_offset":21404,"ancestor_record_offset":21100,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1673,"record_offset":21592,"parent_record_offset":21568,"ancestor_record_offset":21100,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1673,"record_offset":21476,"parent_record_offset":21288,"ancestor_record_offset":21100,"inlinee":5122,"source_line":1460,"archive":"_pdb/B/modules/01673.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$1::from
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":559,"record_offset":33828,"rva":11105968,"va":5379815088}
// 编译实例 RVAs: 0xa976b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4733709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379815088,"raw_hex":"e8f3c4b7ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379815192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d130
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t809296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379815088,"raw_hex":"e850590c00","size":5,"soft_fail":false,"target_va":5380624688,"va":5379815387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31725526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379815088,"raw_hex":"e8d617e401","size":5,"soft_fail":false,"target_va":5411540963,"va":5379815432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":33976,"parent_record_offset":33828,"ancestor_record_offset":33828,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":35040,"parent_record_offset":33828,"ancestor_record_offset":33828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":34580,"parent_record_offset":34556,"ancestor_record_offset":33828,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":34604,"parent_record_offset":34580,"ancestor_record_offset":33828,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":559,"record_offset":34556,"parent_record_offset":34376,"ancestor_record_offset":33828,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":34308,"parent_record_offset":34276,"ancestor_record_offset":33828,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":34376,"parent_record_offset":34340,"ancestor_record_offset":33828,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":34340,"parent_record_offset":34308,"ancestor_record_offset":33828,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":559,"record_offset":34416,"parent_record_offset":34376,"ancestor_record_offset":33828,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":35164,"parent_record_offset":35124,"ancestor_record_offset":33828,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":559,"record_offset":35124,"parent_record_offset":35096,"ancestor_record_offset":33828,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":34688,"parent_record_offset":34660,"ancestor_record_offset":33828,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":34244,"parent_record_offset":34060,"ancestor_record_offset":33828,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":34276,"parent_record_offset":34244,"ancestor_record_offset":33828,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":34004,"parent_record_offset":33976,"ancestor_record_offset":33828,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":559,"record_offset":34868,"parent_record_offset":34840,"ancestor_record_offset":33828,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":559,"record_offset":34936,"parent_record_offset":34868,"ancestor_record_offset":33828,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":559,"record_offset":34660,"parent_record_offset":34060,"ancestor_record_offset":33828,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":559,"record_offset":34032,"parent_record_offset":34004,"ancestor_record_offset":33828,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":559,"record_offset":34060,"parent_record_offset":34032,"ancestor_record_offset":33828,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":34440,"parent_record_offset":34416,"ancestor_record_offset":33828,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":559,"record_offset":34464,"parent_record_offset":34440,"ancestor_record_offset":33828,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":559,"record_offset":34512,"parent_record_offset":34488,"ancestor_record_offset":33828,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":559,"record_offset":34812,"parent_record_offset":34772,"ancestor_record_offset":33828,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":559,"record_offset":34744,"parent_record_offset":34716,"ancestor_record_offset":33828,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":559,"record_offset":34104,"parent_record_offset":34060,"ancestor_record_offset":33828,"inlinee":11428,"source_line":58,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":34716,"parent_record_offset":34688,"ancestor_record_offset":33828,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":559,"record_offset":34840,"parent_record_offset":34812,"ancestor_record_offset":33828,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":559,"record_offset":34488,"parent_record_offset":34464,"ancestor_record_offset":33828,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":559,"record_offset":34200,"parent_record_offset":34176,"ancestor_record_offset":33828,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":35068,"parent_record_offset":35040,"ancestor_record_offset":33828,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":559,"record_offset":34176,"parent_record_offset":34152,"ancestor_record_offset":33828,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":34976,"parent_record_offset":34772,"ancestor_record_offset":33828,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":35200,"parent_record_offset":35164,"ancestor_record_offset":33828,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":559,"record_offset":34152,"parent_record_offset":34128,"ancestor_record_offset":33828,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":559,"record_offset":34904,"parent_record_offset":34868,"ancestor_record_offset":33828,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":34772,"parent_record_offset":34744,"ancestor_record_offset":33828,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":559,"record_offset":34128,"parent_record_offset":34104,"ancestor_record_offset":33828,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":35228,"parent_record_offset":35164,"ancestor_record_offset":33828,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00559.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":559,"record_offset":35260,"parent_record_offset":35124,"ancestor_record_offset":33828,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00559.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$216::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$54::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::impl$54::fmt", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::marker::PhantomData<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::marker::PhantomData", "arguments": ["alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::value::de::impl$5::newtype_variant_seed<core::marker::PhantomData<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> > >", "constructor": "serde_json::value::de::impl$5::newtype_variant_seed", "arguments": ["core::marker::PhantomData<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde::de::impls::impl$211::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: path_edit.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [资源/历史 JSON schema，不是内存布局] Path history serialized observations
// 全部实际键和数据形态: {"/commands/Path": {"observed_types": {"object": 234}, "object_instances": 234, "field_occurrences": {"brush_size": 234, "is_additive": 234, "points": 234, "blend_mode": 228}, "present_in_every_observed_object": ["brush_size", "is_additive", "points"], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/2/edits/0/Path"}]}, "/commands/Path/blend_mode": {"observed_types": {"string": 228}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": ["Max"], "sources": [{"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/49/edits/0/Path/blend_mode"}, {"file": "assets/starting-builds/full/01-cottage/history.json", "pointer": "/History/edit_chain/50/edits/0/Path/blend_mode"}]}, "/commands/Path/brush_size": {"observed_types": {"number": 234}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [0.32000002, 1.68], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/brush_size"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/2/edits/0/Path/brush_size"}]}, "/commands/Path/is_additive": {"observed_types": {"boolean": 234}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/is_additive"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/2/edits/0/Path/is_additive"}]}, "/commands/Path/points": {"observed_types": {"array": 234}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"2": 64, "3": 28, "4": 27, "5": 9, "6": 2, "7": 3, "8": 3, "9": 1, "10": 2, "12": 3, "13": 4, "14": 4, "15": 3, "16": 3, "17": 2, "18": 1, "20": 3, "21": 3, "22": 1, "24": 2, "26": 3, "28": 1, "29": 1, "31": 2, "32": 2, "33": 2, "34": 3, "36": 1, "37": 2, "38": 2, "40": 1, "42": 2, "43": 1, "44": 1, "45": 1, "46": 1, "47": 1, "48": 1, "49": 1, "50": 1, "51": 1, "53": 2, "54": 2, "61": 1, "63": 2, "69": 1, "71": 1, "72": 2, "77": 1, "79": 1, "83": 2, "84": 1, "91": 1, "96": 1, "98": 1, "101": 1, "102": 1, "108": 1, "116": 1, "121": 1, "125": 1, "127": 1, "128": 1, "144": 1, "148": 1, "149": 2, "156": 1, "163": 1, "177": 1, "295": 1}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/points"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/2/edits/0/Path/points"}]}, "/commands/Path/points/[]": {"observed_types": {"array": 5780}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {"3": 5780}, "numeric_range": null, "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/points/0"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/points/1"}]}, "/commands/Path/points/[]/[]": {"observed_types": {"number": 17340}, "object_instances": 0, "field_occurrences": {}, "present_in_every_observed_object": [], "array_length_counts": {}, "numeric_range": [-37.76115036010742, 30.47100067138672], "observed_string_values": [], "sources": [{"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/points/0/0"}, {"file": "assets/starting-builds/demo/00-demo2/history.json", "pointer": "/History/edit_chain/1/edits/0/Path/points/0/1"}]}}
// offsets / Rust numeric types / missing defaults = unknown。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: path_edit.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 14; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_watch::TextureWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::draw_passes::final_post::FinalPostResources>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::configwatch::ConfigWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::commands::Commands"]
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize
// 信息 ID: cd2130088f531125e1817508
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=21012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: ee5b06b2e1b0ee26c7709813
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=383808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: f86a915f9daecedd59c5df36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=383648; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: e3c9d1382e6a4d976b24886c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde::de::impl$1::fmt<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1673, inline_record=2104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 713357258dceae7c8b2ce319
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_object<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=382096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 4e368e5799552ede86735c1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1630, inline_record=136940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 6acfdd4ab173d0f1295ad4cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::path_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> > @ 0x1697049 -> 0x16e8510 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize
// 信息 ID: 6048942bcc7795f2e7094cd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=181, inline_record=60876; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=19452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: 1c19ee8b801adb0c3e1b03c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["u8", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["alloc::vec::Vec<u8,alloc::alloc::Global>"]; 字段布局 = unknown。
// [类型关系] serde_json::ser::Serializer -> ["ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >", "serde_json::ser::CompactFormatter"]; 字段布局 = unknown。
// [类型关系] ref_mut$ -> ["serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter>"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::path_edit::_::impl$0::serialize -> ["ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$0::from
// 信息 ID: 3816a61120222aebb6e46e07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=1673, inline_record=21100; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$1::from
// 信息 ID: ae5794e8e033e05c02e90b94
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x489753 -> 0xa976b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x16e83a3 -> 0xa976b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$2::fmt
// 信息 ID: 963f4015ec3d46eb744468ee
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$54::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>; module=593, inline_record=56788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$3::clone
// 信息 ID: f32506101770f6c3c48726bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=181, inline_record=60668; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=1673, inline_record=19244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
