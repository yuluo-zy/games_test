// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/utils/geo.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 027f4248d612f8abd841feafa3a3928d8ce0b3e7ef1764835dde18d7a2d3c286
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: geo.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 551, 1585, 1610, 1624
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":18384,"rva":22538944,"va":5391248064}
// 编译实例 RVAs: 0x157eac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16166991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e8b14f09ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391248474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16167131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e8254f09ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391248630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11837282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e89e604bff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391248365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2652609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e83f86d7ff","size":5,"soft_fail":false,"target_va":5388595808,"va":5391248412,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20292250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e89aa23501","size":5,"soft_fail":false,"target_va":5411540963,"va":5391248708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20293121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e801a63501","size":5,"soft_fail":false,"target_va":5411541808,"va":5391248682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12916752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248064,"raw_hex":"e8f0e73aff","size":5,"soft_fail":false,"target_va":5378331392,"va":5391248139,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1624,"record_offset":18608,"parent_record_offset":18384,"ancestor_record_offset":18384,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":18784,"parent_record_offset":18384,"ancestor_record_offset":18384,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":18748,"parent_record_offset":18384,"ancestor_record_offset":18384,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":20736,"parent_record_offset":18384,"ancestor_record_offset":18384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1624,"record_offset":20472,"parent_record_offset":18384,"ancestor_record_offset":18384,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":19816,"parent_record_offset":19792,"ancestor_record_offset":18384,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":19840,"parent_record_offset":19816,"ancestor_record_offset":18384,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":20684,"parent_record_offset":20660,"ancestor_record_offset":18384,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":19792,"parent_record_offset":19752,"ancestor_record_offset":18384,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":20660,"parent_record_offset":20568,"ancestor_record_offset":18384,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":19684,"parent_record_offset":19652,"ancestor_record_offset":18384,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":20596,"parent_record_offset":20568,"ancestor_record_offset":18384,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":20568,"parent_record_offset":20544,"ancestor_record_offset":18384,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":19752,"parent_record_offset":19716,"ancestor_record_offset":18384,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":19716,"parent_record_offset":19684,"ancestor_record_offset":18384,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":20544,"parent_record_offset":20520,"ancestor_record_offset":18384,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":19924,"parent_record_offset":19896,"ancestor_record_offset":18384,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_parts
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":19416,"parent_record_offset":18896,"ancestor_record_offset":18384,"inlinee":11710,"source_line":717,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":19440,"parent_record_offset":19416,"ancestor_record_offset":18384,"inlinee":11711,"source_line":1050,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":19620,"parent_record_offset":19576,"ancestor_record_offset":18384,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":19652,"parent_record_offset":19620,"ancestor_record_offset":18384,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":18812,"parent_record_offset":18784,"ancestor_record_offset":18384,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":20332,"parent_record_offset":20304,"ancestor_record_offset":18384,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1624,"record_offset":18868,"parent_record_offset":18840,"ancestor_record_offset":18384,"inlinee":11705,"source_line":251,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1624,"record_offset":18840,"parent_record_offset":18812,"ancestor_record_offset":18384,"inlinee":11928,"source_line":233,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1624,"record_offset":18932,"parent_record_offset":18896,"ancestor_record_offset":18384,"inlinee":11713,"source_line":391,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1624,"record_offset":20760,"parent_record_offset":20736,"ancestor_record_offset":18384,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1624,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":18384,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1624,"record_offset":19248,"parent_record_offset":19216,"ancestor_record_offset":18384,"inlinee":11715,"source_line":352,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1624,"record_offset":18632,"parent_record_offset":18608,"ancestor_record_offset":18384,"inlinee":4934,"source_line":229,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1624,"record_offset":20396,"parent_record_offset":20332,"ancestor_record_offset":18384,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1624,"record_offset":19896,"parent_record_offset":19576,"ancestor_record_offset":18384,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1624,"record_offset":19548,"parent_record_offset":19520,"ancestor_record_offset":18384,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1624,"record_offset":19576,"parent_record_offset":19548,"ancestor_record_offset":18384,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1624,"record_offset":19056,"parent_record_offset":19016,"ancestor_record_offset":18384,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":19152,"parent_record_offset":18932,"ancestor_record_offset":18384,"inlinee":11714,"source_line":132,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":19980,"parent_record_offset":19952,"ancestor_record_offset":18384,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":20240,"parent_record_offset":20008,"ancestor_record_offset":18384,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1624,"record_offset":19216,"parent_record_offset":19184,"ancestor_record_offset":18384,"inlinee":9837,"source_line":637,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":1624,"record_offset":19184,"parent_record_offset":19152,"ancestor_record_offset":18384,"inlinee":9835,"source_line":614,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":19088,"parent_record_offset":19016,"ancestor_record_offset":18384,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":19016,"parent_record_offset":18980,"ancestor_record_offset":18384,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":18980,"parent_record_offset":18932,"ancestor_record_offset":18384,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":20008,"parent_record_offset":19980,"ancestor_record_offset":18384,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":19952,"parent_record_offset":19924,"ancestor_record_offset":18384,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":20304,"parent_record_offset":20240,"ancestor_record_offset":18384,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":19112,"parent_record_offset":19088,"ancestor_record_offset":18384,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":20628,"parent_record_offset":20596,"ancestor_record_offset":18384,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1624,"record_offset":18704,"parent_record_offset":18680,"ancestor_record_offset":18384,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":18384,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1624,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":18384,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1624,"record_offset":19280,"parent_record_offset":19248,"ancestor_record_offset":18384,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1624,"record_offset":18656,"parent_record_offset":18632,"ancestor_record_offset":18384,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":19320,"parent_record_offset":19280,"ancestor_record_offset":18384,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":19376,"parent_record_offset":18932,"ancestor_record_offset":18384,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1624,"record_offset":20272,"parent_record_offset":20240,"ancestor_record_offset":18384,"inlinee":72367,"source_line":22,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":24472,"rva":22540592,"va":5391249712}
// 编译实例 RVAs: 0x157f130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16168452","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e8fc4909ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391249935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16168602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e8664909ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391250101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11838745","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e8e75a4bff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391249828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412612d0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3268289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e83f21ceff","size":5,"soft_fail":false,"target_va":5387981520,"va":5391249804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2654075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e88580d7ff","size":5,"soft_fail":false,"target_va":5388595808,"va":5391249878,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20290788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e8e49c3501","size":5,"soft_fail":false,"target_va":5411540963,"va":5391250170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20291653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e845a03501","size":5,"soft_fail":false,"target_va":5411541808,"va":5391250150,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12918375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249712,"raw_hex":"e899e13aff","size":5,"soft_fail":false,"target_va":5378331392,"va":5391249762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1624,"record_offset":24700,"parent_record_offset":24472,"ancestor_record_offset":24472,"inlinee":62294,"source_line":143,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":24876,"parent_record_offset":24472,"ancestor_record_offset":24472,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":24840,"parent_record_offset":24472,"ancestor_record_offset":24472,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1624,"record_offset":25912,"parent_record_offset":24472,"ancestor_record_offset":24472,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":25256,"parent_record_offset":25232,"ancestor_record_offset":24472,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":25280,"parent_record_offset":25256,"ancestor_record_offset":24472,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":26124,"parent_record_offset":26100,"ancestor_record_offset":24472,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":25232,"parent_record_offset":25192,"ancestor_record_offset":24472,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":26100,"parent_record_offset":26008,"ancestor_record_offset":24472,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":25124,"parent_record_offset":25092,"ancestor_record_offset":24472,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":26036,"parent_record_offset":26008,"ancestor_record_offset":24472,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":26008,"parent_record_offset":25984,"ancestor_record_offset":24472,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":25192,"parent_record_offset":25156,"ancestor_record_offset":24472,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":25156,"parent_record_offset":25124,"ancestor_record_offset":24472,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":25984,"parent_record_offset":25960,"ancestor_record_offset":24472,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":25364,"parent_record_offset":25336,"ancestor_record_offset":24472,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":25060,"parent_record_offset":25016,"ancestor_record_offset":24472,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":25092,"parent_record_offset":25060,"ancestor_record_offset":24472,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":24900,"parent_record_offset":24876,"ancestor_record_offset":24472,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":25772,"parent_record_offset":25744,"ancestor_record_offset":24472,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1624,"record_offset":25836,"parent_record_offset":25772,"ancestor_record_offset":24472,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1624,"record_offset":25336,"parent_record_offset":25016,"ancestor_record_offset":24472,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1624,"record_offset":24988,"parent_record_offset":24960,"ancestor_record_offset":24472,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1624,"record_offset":25016,"parent_record_offset":24988,"ancestor_record_offset":24472,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1624,"record_offset":25644,"parent_record_offset":25536,"ancestor_record_offset":24472,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":25420,"parent_record_offset":25392,"ancestor_record_offset":24472,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":25680,"parent_record_offset":25448,"ancestor_record_offset":24472,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":25580,"parent_record_offset":25536,"ancestor_record_offset":24472,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":25536,"parent_record_offset":25496,"ancestor_record_offset":24472,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":25496,"parent_record_offset":25448,"ancestor_record_offset":24472,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":25448,"parent_record_offset":25420,"ancestor_record_offset":24472,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":25392,"parent_record_offset":25364,"ancestor_record_offset":24472,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":25744,"parent_record_offset":25680,"ancestor_record_offset":24472,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":25608,"parent_record_offset":25580,"ancestor_record_offset":24472,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":26068,"parent_record_offset":26036,"ancestor_record_offset":24472,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1624,"record_offset":24796,"parent_record_offset":24772,"ancestor_record_offset":24472,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":25936,"parent_record_offset":25912,"ancestor_record_offset":24472,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1624,"record_offset":24772,"parent_record_offset":24748,"ancestor_record_offset":24472,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1624,"record_offset":24748,"parent_record_offset":24724,"ancestor_record_offset":24472,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":25804,"parent_record_offset":25772,"ancestor_record_offset":24472,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1624,"record_offset":24724,"parent_record_offset":24700,"ancestor_record_offset":24472,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1624,"record_offset":25712,"parent_record_offset":25680,"ancestor_record_offset":24472,"inlinee":72367,"source_line":22,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":21072,"rva":22539680,"va":5391248800}
// 编译实例 RVAs: 0x157eda0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16167525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e89b4d09ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391249008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16167674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e8064d09ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391249173,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11837820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e8845e4bff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391248903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2653150","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e82284d7ff","size":5,"soft_fail":false,"target_va":5388595808,"va":5391248953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14130d430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2562500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e83ce6d8ff","size":5,"soft_fail":false,"target_va":5388686384,"va":5391248879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20291714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e882a03501","size":5,"soft_fail":false,"target_va":5411540963,"va":5391249244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20292579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e8e3a33501","size":5,"soft_fail":false,"target_va":5411541808,"va":5391249224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12917444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391248800,"raw_hex":"e83ce53aff","size":5,"soft_fail":false,"target_va":5378331392,"va":5391248831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":21404,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":21500,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":21464,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1624,"record_offset":22536,"parent_record_offset":21072,"ancestor_record_offset":21072,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":21880,"parent_record_offset":21856,"ancestor_record_offset":21072,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":21904,"parent_record_offset":21880,"ancestor_record_offset":21072,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":22748,"parent_record_offset":22724,"ancestor_record_offset":21072,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":21856,"parent_record_offset":21816,"ancestor_record_offset":21072,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":22724,"parent_record_offset":22632,"ancestor_record_offset":21072,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":21748,"parent_record_offset":21716,"ancestor_record_offset":21072,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":22660,"parent_record_offset":22632,"ancestor_record_offset":21072,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":22632,"parent_record_offset":22608,"ancestor_record_offset":21072,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":21816,"parent_record_offset":21780,"ancestor_record_offset":21072,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":21780,"parent_record_offset":21748,"ancestor_record_offset":21072,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":21072,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":21988,"parent_record_offset":21960,"ancestor_record_offset":21072,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":21684,"parent_record_offset":21640,"ancestor_record_offset":21072,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":21716,"parent_record_offset":21684,"ancestor_record_offset":21072,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":21524,"parent_record_offset":21500,"ancestor_record_offset":21072,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":22396,"parent_record_offset":22368,"ancestor_record_offset":21072,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1624,"record_offset":22460,"parent_record_offset":22396,"ancestor_record_offset":21072,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1624,"record_offset":21960,"parent_record_offset":21640,"ancestor_record_offset":21072,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1624,"record_offset":21612,"parent_record_offset":21584,"ancestor_record_offset":21072,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1624,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":21072,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1624,"record_offset":22268,"parent_record_offset":22160,"ancestor_record_offset":21072,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":22044,"parent_record_offset":22016,"ancestor_record_offset":21072,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":22304,"parent_record_offset":22072,"ancestor_record_offset":21072,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":22204,"parent_record_offset":22160,"ancestor_record_offset":21072,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":22160,"parent_record_offset":22120,"ancestor_record_offset":21072,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":22120,"parent_record_offset":22072,"ancestor_record_offset":21072,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":22072,"parent_record_offset":22044,"ancestor_record_offset":21072,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":22016,"parent_record_offset":21988,"ancestor_record_offset":21072,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":22368,"parent_record_offset":22304,"ancestor_record_offset":21072,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":22232,"parent_record_offset":22204,"ancestor_record_offset":21072,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":22692,"parent_record_offset":22660,"ancestor_record_offset":21072,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":21072,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":22428,"parent_record_offset":22396,"ancestor_record_offset":21072,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1624,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":21072,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1624,"record_offset":22336,"parent_record_offset":22304,"ancestor_record_offset":21072,"inlinee":72367,"source_line":22,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":22804,"rva":22540144,"va":5391249264}
// 编译实例 RVAs: 0x157ef70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16167986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e8ce4b09ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5391249469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16168138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e8364b09ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5391249637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11838281","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e8b75c4bff","size":5,"soft_fail":false,"target_va":5379411088,"va":5391249364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2653611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e85582d7ff","size":5,"soft_fail":false,"target_va":5388595808,"va":5391249414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1413014d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2611953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e80f25d8ff","size":5,"soft_fail":false,"target_va":5388637392,"va":5391249340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20291252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e8b49e3501","size":5,"soft_fail":false,"target_va":5411540963,"va":5391249706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20292117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e815a23501","size":5,"soft_fail":false,"target_va":5411541808,"va":5391249686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-12917902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391249264,"raw_hex":"e872e33aff","size":5,"soft_fail":false,"target_va":5378331392,"va":5391249289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":23168,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":23132,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1624,"record_offset":24204,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":23548,"parent_record_offset":23524,"ancestor_record_offset":22804,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":23572,"parent_record_offset":23548,"ancestor_record_offset":22804,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":24416,"parent_record_offset":24392,"ancestor_record_offset":22804,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":23524,"parent_record_offset":23484,"ancestor_record_offset":22804,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1624,"record_offset":24392,"parent_record_offset":24300,"ancestor_record_offset":22804,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":23416,"parent_record_offset":23384,"ancestor_record_offset":22804,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":24328,"parent_record_offset":24300,"ancestor_record_offset":22804,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":22804,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":23484,"parent_record_offset":23448,"ancestor_record_offset":22804,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":23448,"parent_record_offset":23416,"ancestor_record_offset":22804,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1624,"record_offset":24276,"parent_record_offset":24252,"ancestor_record_offset":22804,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":23656,"parent_record_offset":23628,"ancestor_record_offset":22804,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":23352,"parent_record_offset":23308,"ancestor_record_offset":22804,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":23384,"parent_record_offset":23352,"ancestor_record_offset":22804,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":23192,"parent_record_offset":23168,"ancestor_record_offset":22804,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1624,"record_offset":24064,"parent_record_offset":24036,"ancestor_record_offset":22804,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1624,"record_offset":24128,"parent_record_offset":24064,"ancestor_record_offset":22804,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1624,"record_offset":23628,"parent_record_offset":23308,"ancestor_record_offset":22804,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1624,"record_offset":23280,"parent_record_offset":23252,"ancestor_record_offset":22804,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1624,"record_offset":23308,"parent_record_offset":23280,"ancestor_record_offset":22804,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1624,"record_offset":23936,"parent_record_offset":23828,"ancestor_record_offset":22804,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":23712,"parent_record_offset":23684,"ancestor_record_offset":22804,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":23972,"parent_record_offset":23740,"ancestor_record_offset":22804,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":23872,"parent_record_offset":23828,"ancestor_record_offset":22804,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":23828,"parent_record_offset":23788,"ancestor_record_offset":22804,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":23788,"parent_record_offset":23740,"ancestor_record_offset":22804,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":23740,"parent_record_offset":23712,"ancestor_record_offset":22804,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":23684,"parent_record_offset":23656,"ancestor_record_offset":22804,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1624,"record_offset":24036,"parent_record_offset":23972,"ancestor_record_offset":22804,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":23900,"parent_record_offset":23872,"ancestor_record_offset":22804,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1624,"record_offset":24360,"parent_record_offset":24328,"ancestor_record_offset":22804,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":24228,"parent_record_offset":24204,"ancestor_record_offset":22804,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":24096,"parent_record_offset":24064,"ancestor_record_offset":22804,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1624,"record_offset":24004,"parent_record_offset":23972,"ancestor_record_offset":22804,"inlinee":72367,"source_line":22,"archive":"_pdb/B/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11AAF
// 首条位置: {"module":1585,"inlinee":72367}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::rewire_enclosed_curve
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":56640,"rva":10926672,"va":5379635792}
// 编译实例 RVAs: 0xa6ba50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4554642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e86e80baff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379636141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141466bb0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10465522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8f2b09f00","size":5,"soft_fail":false,"target_va":5390101424,"va":5379635897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141469430
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10475298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e822d79f00","size":5,"soft_fail":false,"target_va":5390111792,"va":5379636489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32210753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8417feb01","size":5,"soft_fail":false,"target_va":5411847168,"va":5379636410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32210900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8d47feb01","size":5,"soft_fail":false,"target_va":5411847184,"va":5379636279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31710387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8b3dce301","size":5,"soft_fail":false,"target_va":5411347040,"va":5379636648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31904953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8b9d4e601","size":5,"soft_fail":false,"target_va":5411541776,"va":5379636818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31905589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e835d7e601","size":5,"soft_fail":false,"target_va":5411542296,"va":5379636702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31905979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379635792,"raw_hex":"e8bbd8e601","size":5,"soft_fail":false,"target_va":5411542880,"va":5379636896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":57152,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":58288,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":56808,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":58656,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":551,"record_offset":58376,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":57180,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":58348,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":551,"record_offset":58412,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":57000,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":9564,"source_line":422,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":551,"record_offset":59740,"parent_record_offset":56640,"ancestor_record_offset":56640,"inlinee":24512,"source_line":264,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":57392,"parent_record_offset":57368,"ancestor_record_offset":56640,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":57368,"parent_record_offset":57276,"ancestor_record_offset":56640,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":58980,"parent_record_offset":58936,"ancestor_record_offset":56640,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":58808,"parent_record_offset":58780,"ancestor_record_offset":56640,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":57304,"parent_record_offset":57276,"ancestor_record_offset":56640,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":57276,"parent_record_offset":57252,"ancestor_record_offset":56640,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":58868,"parent_record_offset":58836,"ancestor_record_offset":56640,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":59028,"parent_record_offset":59004,"ancestor_record_offset":56640,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":59004,"parent_record_offset":58980,"ancestor_record_offset":56640,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":58836,"parent_record_offset":58808,"ancestor_record_offset":56640,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":57252,"parent_record_offset":57228,"ancestor_record_offset":56640,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":58736,"parent_record_offset":58692,"ancestor_record_offset":56640,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":59068,"parent_record_offset":58736,"ancestor_record_offset":56640,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":58780,"parent_record_offset":58736,"ancestor_record_offset":56640,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":551,"record_offset":59124,"parent_record_offset":59100,"ancestor_record_offset":56640,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":551,"record_offset":59172,"parent_record_offset":59148,"ancestor_record_offset":56640,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":551,"record_offset":58692,"parent_record_offset":58656,"ancestor_record_offset":56640,"inlinee":8579,"source_line":35,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":58312,"parent_record_offset":58288,"ancestor_record_offset":56640,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":57336,"parent_record_offset":57304,"ancestor_record_offset":56640,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":58892,"parent_record_offset":58868,"ancestor_record_offset":56640,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":551,"record_offset":57052,"parent_record_offset":57024,"ancestor_record_offset":56640,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":57204,"parent_record_offset":57180,"ancestor_record_offset":56640,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":58936,"parent_record_offset":58736,"ancestor_record_offset":56640,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":551,"record_offset":59768,"parent_record_offset":59740,"ancestor_record_offset":56640,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":56836,"parent_record_offset":56808,"ancestor_record_offset":56640,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":551,"record_offset":56864,"parent_record_offset":56836,"ancestor_record_offset":56640,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":57080,"parent_record_offset":57024,"ancestor_record_offset":56640,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":551,"record_offset":57024,"parent_record_offset":57000,"ancestor_record_offset":56640,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":551,"record_offset":57112,"parent_record_offset":57080,"ancestor_record_offset":56640,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts
// 记录形式: compiled_procedure; instances=1
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":61396,"rva":10928416,"va":5379637536}
// 编译实例 RVAs: 0xa6c120
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400d9b20
//     owner=unresolved_namespace; count=1
//     target_aliases=35 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10036826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e8a6d966ff","size":5,"soft_fail":false,"target_va":5369600800,"va":5379637621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4556274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e80e7abaff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379637757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4556716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e85478baff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379638215,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140888b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1980803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e87dc6e1ff","size":5,"soft_fail":false,"target_va":5377657648,"va":5379638446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a34c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-226879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e8c189fcff","size":5,"soft_fail":false,"target_va":5379411088,"va":5379637962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412f7260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8957798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e866af8800","size":5,"soft_fail":false,"target_va":5388595808,"va":5379638005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31902574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e86ecbe601","size":5,"soft_fail":false,"target_va":5411540963,"va":5379638384,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31903708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e8dccfe601","size":5,"soft_fail":false,"target_va":5411542064,"va":5379638351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31898736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379637536,"raw_hex":"e870bce601","size":5,"soft_fail":false,"target_va":5411537104,"va":5379638363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":62044,"parent_record_offset":61396,"ancestor_record_offset":61396,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":61532,"parent_record_offset":61396,"ancestor_record_offset":61396,"inlinee":9857,"source_line":141,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":551,"record_offset":61644,"parent_record_offset":61396,"ancestor_record_offset":61396,"inlinee":26960,"source_line":26,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":551,"record_offset":64052,"parent_record_offset":61396,"ancestor_record_offset":61396,"inlinee":30837,"source_line":55,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":62492,"parent_record_offset":62468,"ancestor_record_offset":61396,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":62516,"parent_record_offset":62492,"ancestor_record_offset":61396,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":64264,"parent_record_offset":64240,"ancestor_record_offset":61396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":62468,"parent_record_offset":62296,"ancestor_record_offset":61396,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":64240,"parent_record_offset":64148,"ancestor_record_offset":61396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":62236,"parent_record_offset":62208,"ancestor_record_offset":61396,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":64176,"parent_record_offset":64148,"ancestor_record_offset":61396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":64148,"parent_record_offset":64124,"ancestor_record_offset":61396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":62296,"parent_record_offset":62264,"ancestor_record_offset":61396,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":62264,"parent_record_offset":62236,"ancestor_record_offset":61396,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":64124,"parent_record_offset":64100,"ancestor_record_offset":61396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":551,"record_offset":62328,"parent_record_offset":62296,"ancestor_record_offset":61396,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":62596,"parent_record_offset":62572,"ancestor_record_offset":61396,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":62180,"parent_record_offset":62140,"ancestor_record_offset":61396,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":62208,"parent_record_offset":62180,"ancestor_record_offset":61396,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":62076,"parent_record_offset":62044,"ancestor_record_offset":61396,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":62800,"parent_record_offset":62768,"ancestor_record_offset":61396,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":551,"record_offset":62876,"parent_record_offset":62800,"ancestor_record_offset":61396,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":551,"record_offset":62572,"parent_record_offset":62140,"ancestor_record_offset":61396,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":551,"record_offset":62108,"parent_record_offset":62076,"ancestor_record_offset":61396,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":551,"record_offset":62140,"parent_record_offset":62108,"ancestor_record_offset":61396,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":62352,"parent_record_offset":62328,"ancestor_record_offset":61396,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":62376,"parent_record_offset":62352,"ancestor_record_offset":61396,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":61876,"parent_record_offset":61852,"ancestor_record_offset":61396,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":61928,"parent_record_offset":61904,"ancestor_record_offset":61396,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":551,"record_offset":61904,"parent_record_offset":61852,"ancestor_record_offset":61396,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":63784,"parent_record_offset":63680,"ancestor_record_offset":61396,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":62424,"parent_record_offset":62400,"ancestor_record_offset":61396,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":551,"record_offset":62644,"parent_record_offset":62620,"ancestor_record_offset":61396,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":551,"record_offset":62704,"parent_record_offset":62668,"ancestor_record_offset":61396,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":551,"record_offset":63720,"parent_record_offset":63680,"ancestor_record_offset":61396,"inlinee":4764,"source_line":203,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":551,"record_offset":63680,"parent_record_offset":63644,"ancestor_record_offset":61396,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":551,"record_offset":63644,"parent_record_offset":63600,"ancestor_record_offset":61396,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":63600,"parent_record_offset":63576,"ancestor_record_offset":61396,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":62620,"parent_record_offset":62596,"ancestor_record_offset":61396,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":551,"record_offset":62768,"parent_record_offset":62704,"ancestor_record_offset":61396,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":551,"record_offset":61952,"parent_record_offset":61928,"ancestor_record_offset":61396,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":62400,"parent_record_offset":62376,"ancestor_record_offset":61396,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":63356,"parent_record_offset":63332,"ancestor_record_offset":61396,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":63748,"parent_record_offset":63720,"ancestor_record_offset":61396,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":64208,"parent_record_offset":64176,"ancestor_record_offset":61396,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":61824,"parent_record_offset":61796,"ancestor_record_offset":61396,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":61604,"parent_record_offset":61580,"ancestor_record_offset":61396,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":64076,"parent_record_offset":64052,"ancestor_record_offset":61396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":61580,"parent_record_offset":61556,"ancestor_record_offset":61396,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":62916,"parent_record_offset":62668,"ancestor_record_offset":61396,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":551,"record_offset":61556,"parent_record_offset":61532,"ancestor_record_offset":61396,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":61976,"parent_record_offset":61952,"ancestor_record_offset":61396,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":551,"record_offset":62668,"parent_record_offset":62644,"ancestor_record_offset":61396,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":551,"record_offset":62732,"parent_record_offset":62704,"ancestor_record_offset":61396,"inlinee":31561,"source_line":101,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":551,"record_offset":63852,"parent_record_offset":63820,"ancestor_record_offset":61396,"inlinee":31562,"source_line":104,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":61680,"parent_record_offset":61644,"ancestor_record_offset":61396,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":551,"record_offset":61736,"parent_record_offset":61708,"ancestor_record_offset":61396,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":551,"record_offset":61852,"parent_record_offset":61796,"ancestor_record_offset":61396,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":551,"record_offset":61708,"parent_record_offset":61680,"ancestor_record_offset":61396,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":61796,"parent_record_offset":61764,"ancestor_record_offset":61396,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":551,"record_offset":61764,"parent_record_offset":61736,"ancestor_record_offset":61396,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B49
// 首条位置: {"module":551,"inlinee":31561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B4A
// 首条位置: {"module":551,"inlinee":31562}
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
// 完整结构及原始类型记录: geo.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: geo.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: geo.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd> > > (*)(enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)
// 候选参数类型: ["enum2$<country_core::resources::history::replay_cmd_buffer::ReplayCmd>"]
// [文件级接口候选，具体函数归属 unknown] array$<f32,3> (*)(glam::f32::sse2::vec3a::Vec3A)
// 候选参数类型: ["glam::f32::sse2::vec3a::Vec3A"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts
// 信息 ID: b9116c17a3f2c15999b81cbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x121272d -> 0xa6c120 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$0
// 信息 ID: 14567a2a4f1216faef40b45d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=551, inline_record=62732; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$1
// 信息 ID: 87dc1385a2a81ad76747c5fc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=551, inline_record=63852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::rewire_enclosed_curve
// 信息 ID: 8166b1cffd7ad495fe8c4b3c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xacd7fb -> 0xa6ba50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh::closure$1
// 信息 ID: ad8ed157d5c2d8517e325d6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::update_curve_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >; module=1624, inline_record=20272; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >; module=1624, inline_record=22336; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >; module=1624, inline_record=24004; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >; module=1624, inline_record=25712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >
// 信息 ID: 8c99a93729e7bca606f40c4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["glam::f32::vec3::Vec3", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x86bdb2 -> 0x157eac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >
// 信息 ID: d12da61c2fc885a7df2eaa7d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation @ 0x85c253 -> 0x157f130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >
// 信息 ID: 130eadabebe41b3372e48ee8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xa650cf -> 0x157eda0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >
// 信息 ID: 044f73577acd71f35c68ee4b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims @ 0x8de32e -> 0x157ef70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
