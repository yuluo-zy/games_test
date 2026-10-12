// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/history/path_edit.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: e41ae7974331735a4822d100e70e9cc91a92daf8d468f714b0c3c2308f86ce10
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: path_edit.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 203, 540, 585, 611, 1803, 1804
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
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
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
// 首条位置: {"module":540,"record_offset":25576,"rva":10952032,"va":5379661152}
// 编译实例 RVAs: 0xa71d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3982663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e8b93ac3ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379661346,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-3982723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e87d3ac3ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379661422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10939148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e8f41459ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379661335,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415102e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11134260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e834e5a900","size":5,"soft_fail":false,"target_va":5390795488,"va":5379661223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37395663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e8cf9c3a02","size":5,"soft_fail":false,"target_va":5417057056,"va":5379661388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36959254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e816f43302","size":5,"soft_fail":false,"target_va":5416620769,"va":5379661510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::arcinner_layout_for_value_layout
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5133416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379661152,"raw_hex":"e868544e00","size":5,"soft_fail":false,"target_va":5384794736,"va":5379661315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":25792,"parent_record_offset":25576,"ancestor_record_offset":25576,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":540,"record_offset":25768,"parent_record_offset":25576,"ancestor_record_offset":25576,"inlinee":33280,"source_line":19,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26132,"parent_record_offset":26108,"ancestor_record_offset":25576,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":25576,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":25576,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":25576,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26184,"parent_record_offset":26156,"ancestor_record_offset":25576,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":25576,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26108,"parent_record_offset":26084,"ancestor_record_offset":25576,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":25576,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26576,"parent_record_offset":26548,"ancestor_record_offset":25576,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26548,"parent_record_offset":26524,"ancestor_record_offset":25576,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26524,"parent_record_offset":26500,"ancestor_record_offset":25576,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26044,"parent_record_offset":25956,"ancestor_record_offset":25576,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":25956,"parent_record_offset":25920,"ancestor_record_offset":25576,"inlinee":33285,"source_line":2324,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26296,"parent_record_offset":26044,"ancestor_record_offset":25576,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26264,"parent_record_offset":26232,"ancestor_record_offset":25576,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26084,"parent_record_offset":26044,"ancestor_record_offset":25576,"inlinee":33287,"source_line":2328,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$76::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":25920,"parent_record_offset":25892,"ancestor_record_offset":25576,"inlinee":33284,"source_line":4019,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\mod.rs
//     evidence={"module":540,"record_offset":26636,"parent_record_offset":26548,"ancestor_record_offset":25576,"inlinee":4776,"source_line":388,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":540,"record_offset":25988,"parent_record_offset":25956,"ancestor_record_offset":25576,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":540,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":25576,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":25892,"parent_record_offset":25856,"ancestor_record_offset":25576,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":540,"record_offset":26604,"parent_record_offset":26576,"ancestor_record_offset":25576,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":540,"record_offset":25828,"parent_record_offset":25792,"ancestor_record_offset":25576,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26448,"parent_record_offset":25920,"ancestor_record_offset":25576,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26476,"parent_record_offset":25920,"ancestor_record_offset":25576,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":540,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":25576,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":25576,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":26232,"parent_record_offset":26044,"ancestor_record_offset":25576,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::impl$0::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":540,"record_offset":25856,"parent_record_offset":25828,"ancestor_record_offset":25576,"inlinee":33282,"source_line":35,"archive":"_pdb/C/modules/00540.jsonl.gz"}
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
// 首条位置: {"module":203,"record_offset":36616,"rva":5714688,"va":5374423808}
// 编译实例 RVAs: 0x573300, 0xa71a90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1254638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e8ee241300","size":5,"soft_fail":false,"target_va":5375678704,"va":5374424061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402f9710
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2596227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e87d62d8ff","size":5,"soft_fail":false,"target_va":5371827984,"va":5374424206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045d020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1139770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e8c69beeff","size":5,"soft_fail":false,"target_va":5373284384,"va":5374424149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14045f200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1130969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e827beeeff","size":5,"soft_fail":false,"target_va":5373293056,"va":5374424020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404618f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1120955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e845e5eeff","size":5,"soft_fail":false,"target_va":5373303024,"va":5374423974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140464a60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1108432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e83016efff","size":5,"soft_fail":false,"target_va":5373315680,"va":5374424107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bf22f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6811528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e888ef6700","size":5,"soft_fail":false,"target_va":5381235440,"va":5374423907,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2229312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e840042200","size":5,"soft_fail":false,"target_va":5381889920,"va":5379660603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160e950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12176964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e844ceb900","size":5,"soft_fail":false,"target_va":5391837520,"va":5379660551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14954917","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e8a531e400","size":5,"soft_fail":false,"target_va":5394615776,"va":5379660854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b95c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14973455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e80f7ae400","size":5,"soft_fail":false,"target_va":5394634176,"va":5379660716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418b9f80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14975855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e86f83e400","size":5,"soft_fail":false,"target_va":5394636672,"va":5379660812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418bcca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14987549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379660432,"raw_hex":"e81db1e400","size":5,"soft_fail":false,"target_va":5394648224,"va":5379660670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41928792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e858c87f02","size":5,"soft_fail":false,"target_va":5416353040,"va":5374424243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142da1cb0
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t42133748","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374423808,"raw_hex":"e8f4e88202","size":5,"soft_fail":false,"target_va":5416557744,"va":5374423991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":203,"record_offset":37088,"parent_record_offset":36616,"ancestor_record_offset":36616,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":37960,"parent_record_offset":36616,"ancestor_record_offset":36616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":203,"record_offset":37116,"parent_record_offset":36616,"ancestor_record_offset":36616,"inlinee":16216,"source_line":19,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::path_edit::impl$3::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":203,"record_offset":36908,"parent_record_offset":36616,"ancestor_record_offset":36616,"inlinee":16212,"source_line":8,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":38172,"parent_record_offset":38148,"ancestor_record_offset":36616,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":38196,"parent_record_offset":38172,"ancestor_record_offset":36616,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":38220,"parent_record_offset":38196,"ancestor_record_offset":36616,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":203,"record_offset":38148,"parent_record_offset":38056,"ancestor_record_offset":36616,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37540,"parent_record_offset":37508,"ancestor_record_offset":36616,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37408,"parent_record_offset":37380,"ancestor_record_offset":36616,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":38084,"parent_record_offset":38056,"ancestor_record_offset":36616,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":38056,"parent_record_offset":38032,"ancestor_record_offset":36616,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37468,"parent_record_offset":37436,"ancestor_record_offset":36616,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37588,"parent_record_offset":37564,"ancestor_record_offset":36616,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37564,"parent_record_offset":37540,"ancestor_record_offset":36616,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37436,"parent_record_offset":37408,"ancestor_record_offset":36616,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":38032,"parent_record_offset":38008,"ancestor_record_offset":36616,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":36984,"parent_record_offset":36948,"ancestor_record_offset":36616,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":23472,"parent_record_offset":23432,"ancestor_record_offset":22932,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":22932,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37340,"parent_record_offset":37312,"ancestor_record_offset":36616,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37508,"parent_record_offset":37340,"ancestor_record_offset":36616,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37284,"parent_record_offset":37256,"ancestor_record_offset":36616,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37628,"parent_record_offset":37340,"ancestor_record_offset":36616,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37380,"parent_record_offset":37340,"ancestor_record_offset":36616,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":540,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":22932,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":203,"record_offset":37312,"parent_record_offset":37284,"ancestor_record_offset":36616,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":540,"record_offset":23584,"parent_record_offset":23432,"ancestor_record_offset":22932,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":203,"record_offset":38116,"parent_record_offset":38084,"ancestor_record_offset":36616,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":203,"record_offset":37984,"parent_record_offset":37960,"ancestor_record_offset":36616,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":540,"record_offset":23548,"parent_record_offset":23524,"ancestor_record_offset":22932,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":540,"record_offset":23500,"parent_record_offset":23432,"ancestor_record_offset":22932,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":540,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":22932,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":37020,"parent_record_offset":36984,"ancestor_record_offset":36616,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23748,"parent_record_offset":23712,"ancestor_record_offset":22932,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":36616,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":22932,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23804,"parent_record_offset":23712,"ancestor_record_offset":22932,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::impl$1::from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":540,"record_offset":23432,"parent_record_offset":23404,"ancestor_record_offset":22932,"inlinee":33279,"source_line":48,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::raster_stroke::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":203,"record_offset":36948,"parent_record_offset":36908,"ancestor_record_offset":36616,"inlinee":16214,"source_line":11,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37200,"parent_record_offset":37172,"ancestor_record_offset":36616,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37864,"parent_record_offset":37840,"ancestor_record_offset":36616,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":36616,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":36616,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37840,"parent_record_offset":37812,"ancestor_record_offset":36616,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37812,"parent_record_offset":37116,"ancestor_record_offset":36616,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37688,"parent_record_offset":37116,"ancestor_record_offset":36616,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":203,"record_offset":37256,"parent_record_offset":37228,"ancestor_record_offset":36616,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":203,"record_offset":37228,"parent_record_offset":37200,"ancestor_record_offset":36616,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$2::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x97EB
// 首条位置: {"module":611,"inlinee":38891}
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
// IPI IDs: 0x3F54
// 首条位置: {"module":203,"inlinee":16212}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::raster_stroke::impl$5::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/raster_stroke.rs
//     evidence={"module":203,"record_offset":36948,"parent_record_offset":36908,"ancestor_record_offset":36908,"inlinee":16214,"source_line":11,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":203,"record_offset":36984,"parent_record_offset":36948,"ancestor_record_offset":36908,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":37020,"parent_record_offset":36984,"ancestor_record_offset":36908,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":203,"record_offset":37044,"parent_record_offset":37020,"ancestor_record_offset":36908,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8200
// 首条位置: {"module":540,"inlinee":33280}
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
// IPI IDs: 0x8259
// 首条位置: {"module":1803,"inlinee":33369}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":352476,"parent_record_offset":352428,"ancestor_record_offset":352428,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":352500,"parent_record_offset":352476,"ancestor_record_offset":352428,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":352524,"parent_record_offset":352500,"ancestor_record_offset":352428,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":352552,"parent_record_offset":352524,"ancestor_record_offset":352428,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x156EE
// 首条位置: {"module":1803,"inlinee":87790}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":352280,"parent_record_offset":352248,"ancestor_record_offset":352248,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":353108,"parent_record_offset":353084,"ancestor_record_offset":352248,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":353132,"parent_record_offset":353108,"ancestor_record_offset":352248,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":353156,"parent_record_offset":353132,"ancestor_record_offset":352248,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":353084,"parent_record_offset":353028,"ancestor_record_offset":352248,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":353056,"parent_record_offset":353028,"ancestor_record_offset":352248,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":353028,"parent_record_offset":353004,"ancestor_record_offset":352248,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":353004,"parent_record_offset":352980,"ancestor_record_offset":352248,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1803,"record_offset":352500,"parent_record_offset":352476,"ancestor_record_offset":352248,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":352932,"parent_record_offset":352388,"ancestor_record_offset":352248,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":352524,"parent_record_offset":352500,"ancestor_record_offset":352248,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1803,"record_offset":352552,"parent_record_offset":352524,"ancestor_record_offset":352248,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1803,"record_offset":352476,"parent_record_offset":352428,"ancestor_record_offset":352248,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/path_edit.rs
//     evidence={"module":1803,"record_offset":352428,"parent_record_offset":352388,"ancestor_record_offset":352248,"inlinee":33369,"source_line":19,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":352388,"parent_record_offset":352352,"ancestor_record_offset":352248,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":352316,"parent_record_offset":352280,"ancestor_record_offset":352248,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":352352,"parent_record_offset":352316,"ancestor_record_offset":352248,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8224
// 首条位置: {"module":540,"inlinee":33316}
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
// IPI IDs: 0x1578A
// 首条位置: {"module":1804,"inlinee":87946}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":265028,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":266572,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":264772,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":266264,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":266124,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":266460,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":264716,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":264808,"parent_record_offset":264668,"ancestor_record_offset":264668,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":266756,"parent_record_offset":266732,"ancestor_record_offset":264668,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":266780,"parent_record_offset":266756,"ancestor_record_offset":264668,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":266804,"parent_record_offset":266780,"ancestor_record_offset":264668,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1804,"record_offset":266732,"parent_record_offset":266644,"ancestor_record_offset":264668,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":266672,"parent_record_offset":266644,"ancestor_record_offset":264668,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":266644,"parent_record_offset":266620,"ancestor_record_offset":264668,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1804,"record_offset":266620,"parent_record_offset":266596,"ancestor_record_offset":264668,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":264924,"parent_record_offset":264900,"ancestor_record_offset":264668,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1804,"record_offset":266700,"parent_record_offset":266672,"ancestor_record_offset":264668,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":264900,"parent_record_offset":264864,"ancestor_record_offset":264668,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1804,"record_offset":266596,"parent_record_offset":266572,"ancestor_record_offset":264668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1804,"record_offset":266196,"parent_record_offset":266172,"ancestor_record_offset":264668,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_bool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":266340,"parent_record_offset":266316,"ancestor_record_offset":264668,"inlinee":7736,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1804,"record_offset":266172,"parent_record_offset":266148,"ancestor_record_offset":264668,"inlinee":7805,"source_line":125,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":266220,"parent_record_offset":266196,"ancestor_record_offset":264668,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":264740,"parent_record_offset":264716,"ancestor_record_offset":264668,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":264836,"parent_record_offset":264808,"ancestor_record_offset":264668,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1804,"record_offset":264984,"parent_record_offset":264960,"ancestor_record_offset":264668,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":264960,"parent_record_offset":264864,"ancestor_record_offset":264668,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":266048,"parent_record_offset":266024,"ancestor_record_offset":264668,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":265232,"parent_record_offset":265208,"ancestor_record_offset":264668,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1804,"record_offset":265516,"parent_record_offset":265492,"ancestor_record_offset":264668,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=5
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":264864,"parent_record_offset":264836,"ancestor_record_offset":264668,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1569D
// 首条位置: {"module":1803,"inlinee":87709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":13096,"parent_record_offset":12380,"ancestor_record_offset":12380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":12412,"parent_record_offset":12380,"ancestor_record_offset":12380,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":13280,"parent_record_offset":13256,"ancestor_record_offset":12380,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":13304,"parent_record_offset":13280,"ancestor_record_offset":12380,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":12380,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":13256,"parent_record_offset":13168,"ancestor_record_offset":12380,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":13196,"parent_record_offset":13168,"ancestor_record_offset":12380,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":12380,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":13144,"parent_record_offset":13120,"ancestor_record_offset":12380,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":12496,"parent_record_offset":12468,"ancestor_record_offset":12380,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1803,"record_offset":13224,"parent_record_offset":13196,"ancestor_record_offset":12380,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":13120,"parent_record_offset":13096,"ancestor_record_offset":12380,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":12552,"parent_record_offset":12496,"ancestor_record_offset":12380,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":12580,"parent_record_offset":12496,"ancestor_record_offset":12380,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":12524,"parent_record_offset":12496,"ancestor_record_offset":12380,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":12380,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":12696,"parent_record_offset":12468,"ancestor_record_offset":12380,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":12440,"parent_record_offset":12412,"ancestor_record_offset":12380,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":12640,"parent_record_offset":12468,"ancestor_record_offset":12380,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$19::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":12664,"parent_record_offset":12640,"ancestor_record_offset":12380,"inlinee":7615,"source_line":1139,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$3::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":13020,"parent_record_offset":12996,"ancestor_record_offset":12380,"inlinee":7727,"source_line":71,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$77::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1803,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":12380,"inlinee":7158,"source_line":132,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":12468,"parent_record_offset":12440,"ancestor_record_offset":12380,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::_::impl$0::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 19
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3F58
// 首条位置: {"module":203,"inlinee":16216}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::ser::impl$3::serialize_struct
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37144,"parent_record_offset":37116,"ancestor_record_offset":37116,"inlinee":4567,"source_line":372,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37812,"parent_record_offset":37116,"ancestor_record_offset":37116,"inlinee":4577,"source_line":718,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_direct] serde_json::ser::impl$9::serialize_field
//     owner=git_dependency_definition; count=8
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37688,"parent_record_offset":37116,"ancestor_record_offset":37116,"inlinee":4576,"source_line":692,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37540,"parent_record_offset":37508,"ancestor_record_offset":37116,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37408,"parent_record_offset":37380,"ancestor_record_offset":37116,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37468,"parent_record_offset":37436,"ancestor_record_offset":37116,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37588,"parent_record_offset":37564,"ancestor_record_offset":37116,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37564,"parent_record_offset":37540,"ancestor_record_offset":37116,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":203,"record_offset":37436,"parent_record_offset":37408,"ancestor_record_offset":37116,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37340,"parent_record_offset":37312,"ancestor_record_offset":37116,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37508,"parent_record_offset":37340,"ancestor_record_offset":37116,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37284,"parent_record_offset":37256,"ancestor_record_offset":37116,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37628,"parent_record_offset":37340,"ancestor_record_offset":37116,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":203,"record_offset":37380,"parent_record_offset":37340,"ancestor_record_offset":37116,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":203,"record_offset":37312,"parent_record_offset":37284,"ancestor_record_offset":37116,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::begin_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37200,"parent_record_offset":37172,"ancestor_record_offset":37116,"inlinee":4570,"source_line":1865,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::Formatter::end_object
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37864,"parent_record_offset":37840,"ancestor_record_offset":37116,"inlinee":4580,"source_line":1875,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$3::serialize_map
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37172,"parent_record_offset":37144,"ancestor_record_offset":37116,"inlinee":4568,"source_line":349,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::ser::impl$8::end
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/ser.rs
//     evidence={"module":203,"record_offset":37840,"parent_record_offset":37812,"ancestor_record_offset":37116,"inlinee":4579,"source_line":669,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$11::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":203,"record_offset":37256,"parent_record_offset":37228,"ancestor_record_offset":37116,"inlinee":7398,"source_line":503,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   [pdb_inline_transitive] std::io::impls::impl$1::write_all
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\io\impls.rs
//     evidence={"module":203,"record_offset":37228,"parent_record_offset":37200,"ancestor_record_offset":37116,"inlinee":4572,"source_line":77,"archive":"_pdb/C/modules/00203.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$0::from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8202
// 首条位置: {"module":540,"inlinee":33282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":540,"record_offset":25892,"parent_record_offset":25856,"ancestor_record_offset":25856,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26132,"parent_record_offset":26108,"ancestor_record_offset":25856,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26156,"parent_record_offset":26132,"ancestor_record_offset":25856,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26684,"parent_record_offset":26660,"ancestor_record_offset":25856,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26708,"parent_record_offset":26684,"ancestor_record_offset":25856,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26184,"parent_record_offset":26156,"ancestor_record_offset":25856,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26732,"parent_record_offset":26708,"ancestor_record_offset":25856,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26108,"parent_record_offset":26084,"ancestor_record_offset":25856,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":540,"record_offset":26660,"parent_record_offset":26636,"ancestor_record_offset":25856,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26576,"parent_record_offset":26548,"ancestor_record_offset":25856,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26548,"parent_record_offset":26524,"ancestor_record_offset":25856,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":540,"record_offset":26524,"parent_record_offset":26500,"ancestor_record_offset":25856,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_layout
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26044,"parent_record_offset":25956,"ancestor_record_offset":25856,"inlinee":7217,"source_line":2155,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::allocate_for_slice_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":25956,"parent_record_offset":25920,"ancestor_record_offset":25856,"inlinee":33285,"source_line":2324,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::initialize_arcinner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26296,"parent_record_offset":26044,"ancestor_record_offset":25856,"inlinee":7220,"source_line":2187,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$26::allocate_for_layout::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26264,"parent_record_offset":26232,"ancestor_record_offset":25856,"inlinee":7288,"source_line":2162,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::allocate_for_slice_in::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":26084,"parent_record_offset":26044,"ancestor_record_offset":25856,"inlinee":33287,"source_line":2328,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$76::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":25920,"parent_record_offset":25892,"ancestor_record_offset":25856,"inlinee":33284,"source_line":4019,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::impl$2::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\mod.rs
//     evidence={"module":540,"record_offset":26636,"parent_record_offset":26548,"ancestor_record_offset":25856,"inlinee":4776,"source_line":388,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":540,"record_offset":25988,"parent_record_offset":25956,"ancestor_record_offset":25856,"inlinee":21027,"source_line":559,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":540,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":25856,"inlinee":5306,"source_line":564,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":540,"record_offset":26604,"parent_record_offset":26576,"ancestor_record_offset":25856,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26448,"parent_record_offset":25920,"ancestor_record_offset":25856,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26476,"parent_record_offset":25920,"ancestor_record_offset":25856,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":540,"record_offset":26324,"parent_record_offset":26296,"ancestor_record_offset":25856,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":26348,"parent_record_offset":26324,"ancestor_record_offset":25856,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":540,"record_offset":26232,"parent_record_offset":26044,"ancestor_record_offset":25856,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::history::path_edit::impl$1::from
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x81FF
// 首条位置: {"module":540,"inlinee":33279}
// 编译实例 RVAs: 0xbf22f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5556832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381235440,"raw_hex":"e8a035abff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381235515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12513315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381235440,"raw_hex":"e8dd0f41ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381235502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: path_edit.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t654209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381235440,"raw_hex":"e881fb0900","size":5,"soft_fail":false,"target_va":5381889920,"va":5381235706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35385048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381235440,"raw_hex":"e8d8ee1b02","size":5,"soft_fail":false,"target_va":5416620794,"va":5381235741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":23472,"parent_record_offset":23432,"ancestor_record_offset":23432,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":540,"record_offset":23584,"parent_record_offset":23432,"ancestor_record_offset":23432,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":23640,"parent_record_offset":23432,"ancestor_record_offset":23432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":540,"record_offset":23500,"parent_record_offset":23432,"ancestor_record_offset":23432,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":99372,"parent_record_offset":99348,"ancestor_record_offset":98908,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":99396,"parent_record_offset":99372,"ancestor_record_offset":98908,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":99420,"parent_record_offset":99396,"ancestor_record_offset":98908,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":585,"record_offset":99348,"parent_record_offset":99312,"ancestor_record_offset":98908,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":585,"record_offset":99244,"parent_record_offset":99212,"ancestor_record_offset":98908,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":585,"record_offset":99312,"parent_record_offset":99276,"ancestor_record_offset":98908,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":585,"record_offset":99276,"parent_record_offset":99244,"ancestor_record_offset":98908,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":585,"record_offset":99984,"parent_record_offset":99944,"ancestor_record_offset":98908,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":540,"record_offset":23712,"parent_record_offset":23688,"ancestor_record_offset":23432,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":99508,"parent_record_offset":99480,"ancestor_record_offset":98908,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":99180,"parent_record_offset":99140,"ancestor_record_offset":98908,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":99212,"parent_record_offset":99180,"ancestor_record_offset":98908,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":540,"record_offset":23608,"parent_record_offset":23584,"ancestor_record_offset":23432,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":585,"record_offset":99688,"parent_record_offset":99660,"ancestor_record_offset":98908,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":585,"record_offset":99756,"parent_record_offset":99688,"ancestor_record_offset":98908,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":585,"record_offset":99480,"parent_record_offset":99140,"ancestor_record_offset":98908,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":585,"record_offset":99112,"parent_record_offset":99084,"ancestor_record_offset":98908,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":585,"record_offset":99140,"parent_record_offset":99112,"ancestor_record_offset":98908,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":585,"record_offset":99632,"parent_record_offset":99592,"ancestor_record_offset":98908,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":585,"record_offset":99564,"parent_record_offset":99536,"ancestor_record_offset":98908,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":585,"record_offset":99536,"parent_record_offset":99508,"ancestor_record_offset":98908,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":585,"record_offset":99660,"parent_record_offset":99632,"ancestor_record_offset":98908,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":540,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":23432,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":540,"record_offset":23548,"parent_record_offset":23524,"ancestor_record_offset":23432,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":585,"record_offset":99796,"parent_record_offset":99592,"ancestor_record_offset":98908,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":585,"record_offset":100020,"parent_record_offset":99984,"ancestor_record_offset":98908,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":585,"record_offset":99724,"parent_record_offset":99688,"ancestor_record_offset":98908,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":540,"record_offset":23524,"parent_record_offset":23500,"ancestor_record_offset":23432,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":585,"record_offset":99592,"parent_record_offset":99564,"ancestor_record_offset":98908,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00585.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23748,"parent_record_offset":23712,"ancestor_record_offset":23432,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23772,"parent_record_offset":23748,"ancestor_record_offset":23432,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00540.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":540,"record_offset":23804,"parent_record_offset":23712,"ancestor_record_offset":23432,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/00540.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::Arc<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$211::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_core::de::impls::impl$216::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> >", "constructor": "serde_core::de::impls::impl$216::deserialize", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "enum2$<serde_json::value::Value>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::impl$60::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>", "constructor": "alloc::sync::impl$60::fmt", "arguments": ["country_core::resources::history::path_edit::RasterEdit", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize
// 信息 ID: 93317be5a88510d3e743bc66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=540, inline_record=25768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$0::visit_str
// 信息 ID: 62cbd4585f00674c88d9bba4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::path_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=352428; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 9b90c95b6b06cd90a593afcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::resources::history::path_edit::_::impl$0::deserialize::__Field> >; module=1803, inline_record=352248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: a35a8092162c79e1abb344e8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=540, inline_record=46952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: 922047d43f6de14675f94f63
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=264668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 59d8b1948739878e324abe57
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::resources::history::path_edit::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=12380; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
// 信息 ID: 0c609c624744dc635d42b0b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["serde_json::value::Value"]; 字段布局 = unknown。
// [类型关系] country_core::resources::history::path_edit::_::impl$0::deserialize -> ["enum2$<serde_json::value::Value>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_core::de::impls::impl$211::deserialize<country_core::resources::history::path_edit::RasterEdit,enum2$<serde_json::value::Value> > @ 0xbd1839 -> 0xa71d60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize
// 信息 ID: b36ec7e55323d2e4fc332c04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=203, inline_record=37116; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=24064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >
// 信息 ID: dd1fcb62951eed5abecc0814
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
// 信息 ID: cbd556b430480f5721fd950f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::deserialize<enum2$<serde_json::value::Value> >; module=540, inline_record=25856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$1::from
// 信息 ID: 3d6af9daf4edf781b6d7f64d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > @ 0x573363 -> 0xbf22f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=23432; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$2::fmt
// 信息 ID: 62e5d88dd5c810cc155599cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::sync::impl$60::fmt<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>; module=611, inline_record=86352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::history::path_edit::impl$3::clone
// 信息 ID: 18fb3b4b52db73ae1e3ef867
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=203, inline_record=36908; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::history::path_edit::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >; module=540, inline_record=23224; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
