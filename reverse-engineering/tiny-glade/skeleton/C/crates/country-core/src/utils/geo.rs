// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/geo.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 572f9f54a8d25cccb3dc27c8fabf1cdd51c44e78ea54deb43b48abdf44e1159a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: geo.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 600, 1643, 1682, 1700
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":1700,"record_offset":131156,"rva":26703072,"va":5395412192}
// 编译实例 RVAs: 0x19774e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19733913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e867e2d2fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395412596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19734113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e89fe1d2fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395412812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26690396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e8a4bc68fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395412583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15397794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e85e0c15ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5395412493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3674683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e8c5edc7ff","size":5,"soft_fail":false,"target_va":5391737856,"va":5395412534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e8d39b4301","size":5,"soft_fail":false,"target_va":5416620794,"va":5395412770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208781","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e8cd9e4301","size":5,"soft_fail":false,"target_va":5416621536,"va":5395412750,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e82c9f4301","size":5,"soft_fail":false,"target_va":5416621600,"va":5395412719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16136683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412192,"raw_hex":"e815c609ff","size":5,"soft_fail":false,"target_va":5379275600,"va":5395412278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1700,"record_offset":131384,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":131596,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":131560,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":131524,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":133600,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1700,"record_offset":133280,"parent_record_offset":131156,"ancestor_record_offset":131156,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":132624,"parent_record_offset":132600,"ancestor_record_offset":131156,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":132648,"parent_record_offset":132624,"ancestor_record_offset":131156,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":133492,"parent_record_offset":133468,"ancestor_record_offset":131156,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":133516,"parent_record_offset":133492,"ancestor_record_offset":131156,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":132672,"parent_record_offset":132648,"ancestor_record_offset":131156,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":133540,"parent_record_offset":133516,"ancestor_record_offset":131156,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":132600,"parent_record_offset":132560,"ancestor_record_offset":131156,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":133468,"parent_record_offset":133376,"ancestor_record_offset":131156,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":132492,"parent_record_offset":132460,"ancestor_record_offset":131156,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":133404,"parent_record_offset":133376,"ancestor_record_offset":131156,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":133376,"parent_record_offset":133352,"ancestor_record_offset":131156,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":132560,"parent_record_offset":132524,"ancestor_record_offset":131156,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":132524,"parent_record_offset":132492,"ancestor_record_offset":131156,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":133352,"parent_record_offset":133328,"ancestor_record_offset":131156,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":132764,"parent_record_offset":132732,"ancestor_record_offset":131156,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_parts
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":132224,"parent_record_offset":131680,"ancestor_record_offset":131156,"inlinee":10801,"source_line":745,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":132248,"parent_record_offset":132224,"ancestor_record_offset":131156,"inlinee":10758,"source_line":1291,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":132428,"parent_record_offset":132380,"ancestor_record_offset":131156,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":132460,"parent_record_offset":132428,"ancestor_record_offset":131156,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":131624,"parent_record_offset":131596,"ancestor_record_offset":131156,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":133140,"parent_record_offset":133112,"ancestor_record_offset":131156,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::from_iter_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1700,"record_offset":131680,"parent_record_offset":131652,"ancestor_record_offset":131156,"inlinee":10798,"source_line":249,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1700,"record_offset":131652,"parent_record_offset":131624,"ancestor_record_offset":131156,"inlinee":8793,"source_line":232,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":1700,"record_offset":131716,"parent_record_offset":131680,"ancestor_record_offset":131156,"inlinee":10803,"source_line":389,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1700,"record_offset":133624,"parent_record_offset":133600,"ancestor_record_offset":131156,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1700,"record_offset":133672,"parent_record_offset":133648,"ancestor_record_offset":131156,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1700,"record_offset":132044,"parent_record_offset":132008,"ancestor_record_offset":131156,"inlinee":10805,"source_line":362,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1700,"record_offset":131408,"parent_record_offset":131384,"ancestor_record_offset":131156,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1700,"record_offset":133204,"parent_record_offset":133140,"ancestor_record_offset":131156,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1700,"record_offset":132732,"parent_record_offset":132380,"ancestor_record_offset":131156,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1700,"record_offset":132352,"parent_record_offset":132324,"ancestor_record_offset":131156,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1700,"record_offset":132380,"parent_record_offset":132352,"ancestor_record_offset":131156,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1700,"record_offset":131840,"parent_record_offset":131800,"ancestor_record_offset":131156,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":131936,"parent_record_offset":131716,"ancestor_record_offset":131156,"inlinee":10804,"source_line":132,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":132828,"parent_record_offset":132796,"ancestor_record_offset":131156,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":133048,"parent_record_offset":132860,"ancestor_record_offset":131156,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1700,"record_offset":132008,"parent_record_offset":131972,"ancestor_record_offset":131156,"inlinee":6629,"source_line":629,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1700,"record_offset":131972,"parent_record_offset":131936,"ancestor_record_offset":131156,"inlinee":6627,"source_line":606,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":131872,"parent_record_offset":131800,"ancestor_record_offset":131156,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":131800,"parent_record_offset":131764,"ancestor_record_offset":131156,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":131764,"parent_record_offset":131716,"ancestor_record_offset":131156,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":132860,"parent_record_offset":132828,"ancestor_record_offset":131156,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":132796,"parent_record_offset":132764,"ancestor_record_offset":131156,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":133112,"parent_record_offset":133048,"ancestor_record_offset":131156,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":131896,"parent_record_offset":131872,"ancestor_record_offset":131156,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":133436,"parent_record_offset":133404,"ancestor_record_offset":131156,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1700,"record_offset":131480,"parent_record_offset":131456,"ancestor_record_offset":131156,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":133304,"parent_record_offset":133280,"ancestor_record_offset":131156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1700,"record_offset":131456,"parent_record_offset":131432,"ancestor_record_offset":131156,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1700,"record_offset":131432,"parent_record_offset":131408,"ancestor_record_offset":131156,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1700,"record_offset":132080,"parent_record_offset":132044,"ancestor_record_offset":131156,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":132124,"parent_record_offset":132080,"ancestor_record_offset":131156,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":132184,"parent_record_offset":131716,"ancestor_record_offset":131156,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1700,"record_offset":133080,"parent_record_offset":133048,"ancestor_record_offset":131156,"inlinee":81296,"source_line":24,"archive":"_pdb/C/modules/01700.jsonl.gz"}
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
// 首条位置: {"module":1700,"record_offset":133992,"rva":26703808,"va":5395412928}
// 编译实例 RVAs: 0x19777c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19734387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e88de0d2fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395413070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19734873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8a7ded2fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395413572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26690864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8d0ba68fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395413051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15398581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e84b0915ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5395413280,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3675472","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8b0eac7ff","size":5,"soft_fail":false,"target_va":5391737856,"va":5395413323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t21207757","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8cd9a4301","size":5,"soft_fail":false,"target_va":5416620794,"va":5395413032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8ca9b4301","size":5,"soft_fail":false,"target_va":5416621536,"va":5395413521,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e8229c4301","size":5,"soft_fail":false,"target_va":5416621600,"va":5395413497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16137396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395412928,"raw_hex":"e84cc309ff","size":5,"soft_fail":false,"target_va":5379275600,"va":5395412991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1700,"record_offset":134224,"parent_record_offset":133992,"ancestor_record_offset":133992,"inlinee":70693,"source_line":149,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":134436,"parent_record_offset":133992,"ancestor_record_offset":133992,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":134400,"parent_record_offset":133992,"ancestor_record_offset":133992,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":134364,"parent_record_offset":133992,"ancestor_record_offset":133992,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1700,"record_offset":136440,"parent_record_offset":133992,"ancestor_record_offset":133992,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":134888,"parent_record_offset":134864,"ancestor_record_offset":133992,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":134912,"parent_record_offset":134888,"ancestor_record_offset":133992,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":136652,"parent_record_offset":136628,"ancestor_record_offset":133992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":136676,"parent_record_offset":136652,"ancestor_record_offset":133992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":134936,"parent_record_offset":134912,"ancestor_record_offset":133992,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":136700,"parent_record_offset":136676,"ancestor_record_offset":133992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":134864,"parent_record_offset":134704,"ancestor_record_offset":133992,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":136628,"parent_record_offset":136536,"ancestor_record_offset":133992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":134632,"parent_record_offset":134600,"ancestor_record_offset":133992,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":136564,"parent_record_offset":136536,"ancestor_record_offset":133992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":136536,"parent_record_offset":136512,"ancestor_record_offset":133992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":134704,"parent_record_offset":134664,"ancestor_record_offset":133992,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":134664,"parent_record_offset":134632,"ancestor_record_offset":133992,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":136512,"parent_record_offset":136488,"ancestor_record_offset":133992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":134752,"parent_record_offset":134704,"ancestor_record_offset":133992,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":135024,"parent_record_offset":134996,"ancestor_record_offset":133992,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":134568,"parent_record_offset":134520,"ancestor_record_offset":133992,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":134600,"parent_record_offset":134568,"ancestor_record_offset":133992,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":134464,"parent_record_offset":134436,"ancestor_record_offset":133992,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":135268,"parent_record_offset":135236,"ancestor_record_offset":133992,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1700,"record_offset":135344,"parent_record_offset":135268,"ancestor_record_offset":133992,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1700,"record_offset":134996,"parent_record_offset":134520,"ancestor_record_offset":133992,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1700,"record_offset":134492,"parent_record_offset":134464,"ancestor_record_offset":133992,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1700,"record_offset":134520,"parent_record_offset":134492,"ancestor_record_offset":133992,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1700,"record_offset":134776,"parent_record_offset":134752,"ancestor_record_offset":133992,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1700,"record_offset":134824,"parent_record_offset":134800,"ancestor_record_offset":133992,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1700,"record_offset":135176,"parent_record_offset":135136,"ancestor_record_offset":133992,"inlinee":10907,"source_line":36,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":1700,"record_offset":135108,"parent_record_offset":135080,"ancestor_record_offset":133992,"inlinee":10905,"source_line":77,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":135080,"parent_record_offset":135052,"ancestor_record_offset":133992,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":135204,"parent_record_offset":135176,"ancestor_record_offset":133992,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":136144,"parent_record_offset":136104,"ancestor_record_offset":133992,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":136104,"parent_record_offset":136068,"ancestor_record_offset":133992,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":136068,"parent_record_offset":136020,"ancestor_record_offset":133992,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":136020,"parent_record_offset":135988,"ancestor_record_offset":133992,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":135052,"parent_record_offset":135024,"ancestor_record_offset":133992,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":135236,"parent_record_offset":135204,"ancestor_record_offset":133992,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":134800,"parent_record_offset":134776,"ancestor_record_offset":133992,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":136168,"parent_record_offset":136144,"ancestor_record_offset":133992,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":136596,"parent_record_offset":136564,"ancestor_record_offset":133992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1700,"record_offset":134320,"parent_record_offset":134296,"ancestor_record_offset":133992,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":136464,"parent_record_offset":136440,"ancestor_record_offset":133992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1700,"record_offset":134296,"parent_record_offset":134272,"ancestor_record_offset":133992,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1700,"record_offset":135388,"parent_record_offset":135136,"ancestor_record_offset":133992,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1700,"record_offset":134272,"parent_record_offset":134248,"ancestor_record_offset":133992,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":135308,"parent_record_offset":135268,"ancestor_record_offset":133992,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1700,"record_offset":134248,"parent_record_offset":134224,"ancestor_record_offset":133992,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1700,"record_offset":135136,"parent_record_offset":135108,"ancestor_record_offset":133992,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1700,"record_offset":136240,"parent_record_offset":136208,"ancestor_record_offset":133992,"inlinee":81296,"source_line":24,"archive":"_pdb/C/modules/01700.jsonl.gz"}
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
// 首条位置: {"module":1700,"record_offset":136764,"rva":26704496,"va":5395413616}
// 编译实例 RVAs: 0x1977a70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19735135","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8a1ddd2fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395413818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19735321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8e7dcd2fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395414020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26691618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8deb768fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395413805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15399023","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8910715ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5395413722,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3675914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8f6e8c7ff","size":5,"soft_fail":false,"target_va":5391737856,"va":5395413765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14160bbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3587799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e82941c9ff","size":5,"soft_fail":false,"target_va":5391825904,"va":5395413698,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21206807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e817974301","size":5,"soft_fail":false,"target_va":5416620794,"va":5395413982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21207562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e80a9a4301","size":5,"soft_fail":false,"target_va":5416621536,"va":5395413969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21207650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8629a4301","size":5,"soft_fail":false,"target_va":5416621600,"va":5395413945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16138062","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395413616,"raw_hex":"e8b2c009ff","size":5,"soft_fail":false,"target_va":5379275600,"va":5395413657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":137164,"parent_record_offset":136764,"ancestor_record_offset":136764,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":137128,"parent_record_offset":136764,"ancestor_record_offset":136764,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":137096,"parent_record_offset":136764,"ancestor_record_offset":136764,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1700,"record_offset":138204,"parent_record_offset":136764,"ancestor_record_offset":136764,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":137548,"parent_record_offset":137524,"ancestor_record_offset":136764,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":137572,"parent_record_offset":137548,"ancestor_record_offset":136764,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":138416,"parent_record_offset":138392,"ancestor_record_offset":136764,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":138440,"parent_record_offset":138416,"ancestor_record_offset":136764,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":137596,"parent_record_offset":137572,"ancestor_record_offset":136764,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":138464,"parent_record_offset":138440,"ancestor_record_offset":136764,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":137524,"parent_record_offset":137484,"ancestor_record_offset":136764,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":138392,"parent_record_offset":138300,"ancestor_record_offset":136764,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":137416,"parent_record_offset":137384,"ancestor_record_offset":136764,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":138328,"parent_record_offset":138300,"ancestor_record_offset":136764,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":138300,"parent_record_offset":138276,"ancestor_record_offset":136764,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":137484,"parent_record_offset":137448,"ancestor_record_offset":136764,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":137448,"parent_record_offset":137416,"ancestor_record_offset":136764,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":138276,"parent_record_offset":138252,"ancestor_record_offset":136764,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":137688,"parent_record_offset":137656,"ancestor_record_offset":136764,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":137352,"parent_record_offset":137304,"ancestor_record_offset":136764,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":137384,"parent_record_offset":137352,"ancestor_record_offset":136764,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":137188,"parent_record_offset":137164,"ancestor_record_offset":136764,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":138064,"parent_record_offset":138036,"ancestor_record_offset":136764,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1700,"record_offset":138128,"parent_record_offset":138064,"ancestor_record_offset":136764,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1700,"record_offset":137656,"parent_record_offset":137304,"ancestor_record_offset":136764,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1700,"record_offset":137276,"parent_record_offset":137248,"ancestor_record_offset":136764,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1700,"record_offset":137304,"parent_record_offset":137276,"ancestor_record_offset":136764,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":137752,"parent_record_offset":137720,"ancestor_record_offset":136764,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":137972,"parent_record_offset":137784,"ancestor_record_offset":136764,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":137908,"parent_record_offset":137868,"ancestor_record_offset":136764,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":137868,"parent_record_offset":137832,"ancestor_record_offset":136764,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":137832,"parent_record_offset":137784,"ancestor_record_offset":136764,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":137784,"parent_record_offset":137752,"ancestor_record_offset":136764,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":137720,"parent_record_offset":137688,"ancestor_record_offset":136764,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":138036,"parent_record_offset":137972,"ancestor_record_offset":136764,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":137932,"parent_record_offset":137908,"ancestor_record_offset":136764,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":138360,"parent_record_offset":138328,"ancestor_record_offset":136764,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":138228,"parent_record_offset":138204,"ancestor_record_offset":136764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":138096,"parent_record_offset":138064,"ancestor_record_offset":136764,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1700,"record_offset":138004,"parent_record_offset":137972,"ancestor_record_offset":136764,"inlinee":81296,"source_line":24,"archive":"_pdb/C/modules/01700.jsonl.gz"}
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
// 首条位置: {"module":1700,"record_offset":129392,"rva":26702624,"va":5395411744}
// 编译实例 RVAs: 0x1977320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19733261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e8f3e4d2fe","size":5,"soft_fail":false,"target_va":5375678688,"va":5395411944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19733449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e837e4d2fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5395412148,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26689744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e830bf68fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5395411931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15397149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e8e30e15ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5395411848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3674040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e848f0c7ff","size":5,"soft_fail":false,"target_va":5391737856,"va":5395411891,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608be0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3598229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e86b18c9ff","size":5,"soft_fail":false,"target_va":5391813600,"va":5395411824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21208679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e8679e4301","size":5,"soft_fail":false,"target_va":5416620794,"va":5395412110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21209434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e85aa14301","size":5,"soft_fail":false,"target_va":5416621536,"va":5395412097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t21209522","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e8b2a14301","size":5,"soft_fail":false,"target_va":5416621600,"va":5395412073,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-16136185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5395411744,"raw_hex":"e807c809ff","size":5,"soft_fail":false,"target_va":5379275600,"va":5395411780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":129792,"parent_record_offset":129392,"ancestor_record_offset":129392,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":129756,"parent_record_offset":129392,"ancestor_record_offset":129392,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1700,"record_offset":129724,"parent_record_offset":129392,"ancestor_record_offset":129392,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":1700,"record_offset":130832,"parent_record_offset":129392,"ancestor_record_offset":129392,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":130176,"parent_record_offset":130152,"ancestor_record_offset":129392,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":130200,"parent_record_offset":130176,"ancestor_record_offset":129392,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":131044,"parent_record_offset":131020,"ancestor_record_offset":129392,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":131068,"parent_record_offset":131044,"ancestor_record_offset":129392,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":130224,"parent_record_offset":130200,"ancestor_record_offset":129392,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":131092,"parent_record_offset":131068,"ancestor_record_offset":129392,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":130152,"parent_record_offset":130112,"ancestor_record_offset":129392,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1700,"record_offset":131020,"parent_record_offset":130928,"ancestor_record_offset":129392,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130044,"parent_record_offset":130012,"ancestor_record_offset":129392,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130956,"parent_record_offset":130928,"ancestor_record_offset":129392,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130928,"parent_record_offset":130904,"ancestor_record_offset":129392,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130112,"parent_record_offset":130076,"ancestor_record_offset":129392,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130076,"parent_record_offset":130044,"ancestor_record_offset":129392,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1700,"record_offset":130904,"parent_record_offset":130880,"ancestor_record_offset":129392,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":130316,"parent_record_offset":130284,"ancestor_record_offset":129392,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":129980,"parent_record_offset":129932,"ancestor_record_offset":129392,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":130012,"parent_record_offset":129980,"ancestor_record_offset":129392,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":129816,"parent_record_offset":129792,"ancestor_record_offset":129392,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1700,"record_offset":130692,"parent_record_offset":130664,"ancestor_record_offset":129392,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1700,"record_offset":130756,"parent_record_offset":130692,"ancestor_record_offset":129392,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1700,"record_offset":130284,"parent_record_offset":129932,"ancestor_record_offset":129392,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1700,"record_offset":129904,"parent_record_offset":129876,"ancestor_record_offset":129392,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1700,"record_offset":129932,"parent_record_offset":129904,"ancestor_record_offset":129392,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":130380,"parent_record_offset":130348,"ancestor_record_offset":129392,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":1700,"record_offset":130600,"parent_record_offset":130412,"ancestor_record_offset":129392,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":130536,"parent_record_offset":130496,"ancestor_record_offset":129392,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":130496,"parent_record_offset":130460,"ancestor_record_offset":129392,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":1700,"record_offset":130460,"parent_record_offset":130412,"ancestor_record_offset":129392,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":130412,"parent_record_offset":130380,"ancestor_record_offset":129392,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":130348,"parent_record_offset":130316,"ancestor_record_offset":129392,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1700,"record_offset":130664,"parent_record_offset":130600,"ancestor_record_offset":129392,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":130560,"parent_record_offset":130536,"ancestor_record_offset":129392,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1700,"record_offset":130988,"parent_record_offset":130956,"ancestor_record_offset":129392,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":130856,"parent_record_offset":130832,"ancestor_record_offset":129392,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1700,"record_offset":130724,"parent_record_offset":130692,"ancestor_record_offset":129392,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":1700,"record_offset":130632,"parent_record_offset":130600,"ancestor_record_offset":129392,"inlinee":81296,"source_line":24,"archive":"_pdb/C/modules/01700.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D90
// 首条位置: {"module":1643,"inlinee":81296}
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
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":50784,"rva":12889632,"va":5381598752}
// 编译实例 RVAs: 0xc4ae20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=12
//     evidence={"assembly":"\tcallq\t-5920403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e86da9a5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381599102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404a41d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8023820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8f49085ff","size":5,"soft_fail":false,"target_va":5373575632,"va":5381599447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14196dd90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13774594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8022fd200","size":5,"soft_fail":false,"target_va":5395373456,"va":5381598857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34753378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8624b1202","size":5,"soft_fail":false,"target_va":5416353040,"va":5381599657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35457683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8930a1d02","size":5,"soft_fail":false,"target_va":5417057056,"va":5381599368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35457827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8230b1d02","size":5,"soft_fail":false,"target_va":5417057072,"va":5381599240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35021999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8af641602","size":5,"soft_fail":false,"target_va":5416621600,"va":5381599596,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35021987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e8a3641602","size":5,"soft_fail":false,"target_va":5416621762,"va":5381599770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35021884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381598752,"raw_hex":"e83c641602","size":5,"soft_fail":false,"target_va":5416621632,"va":5381599743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":51324,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":5450,"source_line":2790,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":52656,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":6391,"source_line":2341,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":50952,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$20::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":53080,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":6148,"source_line":3932,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":600,"record_offset":52772,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":51352,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":600,"record_offset":52744,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":12093,"source_line":155,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":600,"record_offset":52808,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":5756,"source_line":281,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":600,"record_offset":51144,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::crashy_new_from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":600,"record_offset":54276,"parent_record_offset":50784,"ancestor_record_offset":50784,"inlinee":26085,"source_line":264,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":51564,"parent_record_offset":51540,"ancestor_record_offset":50784,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":51588,"parent_record_offset":51564,"ancestor_record_offset":50784,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":51612,"parent_record_offset":51588,"ancestor_record_offset":50784,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":51540,"parent_record_offset":51448,"ancestor_record_offset":50784,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53404,"parent_record_offset":53360,"ancestor_record_offset":50784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53232,"parent_record_offset":53204,"ancestor_record_offset":50784,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":51476,"parent_record_offset":51448,"ancestor_record_offset":50784,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":51448,"parent_record_offset":51424,"ancestor_record_offset":50784,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53292,"parent_record_offset":53260,"ancestor_record_offset":50784,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53452,"parent_record_offset":53428,"ancestor_record_offset":50784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53428,"parent_record_offset":53404,"ancestor_record_offset":50784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":53260,"parent_record_offset":53232,"ancestor_record_offset":50784,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":51424,"parent_record_offset":51400,"ancestor_record_offset":50784,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":53160,"parent_record_offset":53116,"ancestor_record_offset":50784,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":53492,"parent_record_offset":53160,"ancestor_record_offset":50784,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":53204,"parent_record_offset":53160,"ancestor_record_offset":50784,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::try_remove
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":52680,"parent_record_offset":52656,"ancestor_record_offset":50784,"inlinee":6392,"source_line":2377,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":600,"record_offset":53548,"parent_record_offset":53524,"ancestor_record_offset":50784,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":600,"record_offset":53596,"parent_record_offset":53572,"ancestor_record_offset":50784,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":53116,"parent_record_offset":53080,"ancestor_record_offset":50784,"inlinee":6150,"source_line":32,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":600,"record_offset":51280,"parent_record_offset":51256,"ancestor_record_offset":50784,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":51508,"parent_record_offset":51476,"ancestor_record_offset":50784,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":53316,"parent_record_offset":53292,"ancestor_record_offset":50784,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":51196,"parent_record_offset":51168,"ancestor_record_offset":50784,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":52704,"parent_record_offset":52680,"ancestor_record_offset":50784,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":51376,"parent_record_offset":51352,"ancestor_record_offset":50784,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":53360,"parent_record_offset":53160,"ancestor_record_offset":50784,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":600,"record_offset":54304,"parent_record_offset":54276,"ancestor_record_offset":50784,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":50980,"parent_record_offset":50952,"ancestor_record_offset":50784,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":600,"record_offset":51008,"parent_record_offset":50980,"ancestor_record_offset":50784,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":600,"record_offset":51224,"parent_record_offset":51168,"ancestor_record_offset":50784,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":600,"record_offset":51168,"parent_record_offset":51144,"ancestor_record_offset":50784,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":600,"record_offset":51256,"parent_record_offset":51224,"ancestor_record_offset":50784,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":40244,"rva":12884096,"va":5381593216}
// 编译实例 RVAs: 0xc49880
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5914772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e86cbfa5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381593455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-5915233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e89fbda5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381593932,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12871249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e8af993bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381593436,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11932938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e8f6ea49ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5381593301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2857683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e82d65d4ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5381594094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1578943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e841e8e7ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5381593642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10144172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e8acc99a00","size":5,"soft_fail":false,"target_va":5391737856,"va":5381593679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t35027364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e8a4791602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381593425,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35028340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e8747d1602","size":5,"soft_fail":false,"target_va":5416622208,"va":5381593863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35025480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381593216,"raw_hex":"e848721602","size":5,"soft_fail":false,"target_va":5416619360,"va":5381593875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":40864,"parent_record_offset":40244,"ancestor_record_offset":40244,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$170::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":600,"record_offset":40380,"parent_record_offset":40244,"ancestor_record_offset":40244,"inlinee":4797,"source_line":142,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":600,"record_offset":40492,"parent_record_offset":40244,"ancestor_record_offset":40244,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":600,"record_offset":42804,"parent_record_offset":40244,"ancestor_record_offset":40244,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":41244,"parent_record_offset":41220,"ancestor_record_offset":40244,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":41268,"parent_record_offset":41244,"ancestor_record_offset":40244,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43016,"parent_record_offset":42992,"ancestor_record_offset":40244,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43040,"parent_record_offset":43016,"ancestor_record_offset":40244,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":41292,"parent_record_offset":41268,"ancestor_record_offset":40244,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43064,"parent_record_offset":43040,"ancestor_record_offset":40244,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":41220,"parent_record_offset":41072,"ancestor_record_offset":40244,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":42992,"parent_record_offset":42900,"ancestor_record_offset":40244,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":41016,"parent_record_offset":40992,"ancestor_record_offset":40244,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":42928,"parent_record_offset":42900,"ancestor_record_offset":40244,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":40244,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":41072,"parent_record_offset":41040,"ancestor_record_offset":40244,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":41040,"parent_record_offset":41016,"ancestor_record_offset":40244,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":42876,"parent_record_offset":42852,"ancestor_record_offset":40244,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":41108,"parent_record_offset":41072,"ancestor_record_offset":40244,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":41376,"parent_record_offset":41352,"ancestor_record_offset":40244,"inlinee":5079,"source_line":3988,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":40968,"parent_record_offset":40936,"ancestor_record_offset":40244,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":40992,"parent_record_offset":40968,"ancestor_record_offset":40244,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":40888,"parent_record_offset":40864,"ancestor_record_offset":40244,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$21::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":41580,"parent_record_offset":41548,"ancestor_record_offset":40244,"inlinee":5113,"source_line":4001,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":600,"record_offset":41656,"parent_record_offset":41580,"ancestor_record_offset":40244,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":41352,"parent_record_offset":40936,"ancestor_record_offset":40244,"inlinee":5078,"source_line":26,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":600,"record_offset":40912,"parent_record_offset":40888,"ancestor_record_offset":40244,"inlinee":6133,"source_line":32,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":600,"record_offset":40936,"parent_record_offset":40912,"ancestor_record_offset":40244,"inlinee":6135,"source_line":50,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":41132,"parent_record_offset":41108,"ancestor_record_offset":40244,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":40724,"parent_record_offset":40700,"ancestor_record_offset":40244,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":40776,"parent_record_offset":40752,"ancestor_record_offset":40244,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":40752,"parent_record_offset":40700,"ancestor_record_offset":40244,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":41180,"parent_record_offset":41156,"ancestor_record_offset":40244,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":41424,"parent_record_offset":41400,"ancestor_record_offset":40244,"inlinee":6631,"source_line":124,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":600,"record_offset":41484,"parent_record_offset":41448,"ancestor_record_offset":40244,"inlinee":5396,"source_line":88,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":42508,"parent_record_offset":42468,"ancestor_record_offset":40244,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":42468,"parent_record_offset":42432,"ancestor_record_offset":40244,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":42432,"parent_record_offset":42388,"ancestor_record_offset":40244,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":42388,"parent_record_offset":42360,"ancestor_record_offset":40244,"inlinee":6141,"source_line":2640,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":41400,"parent_record_offset":41376,"ancestor_record_offset":40244,"inlinee":5042,"source_line":835,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":41548,"parent_record_offset":41484,"ancestor_record_offset":40244,"inlinee":5057,"source_line":842,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":600,"record_offset":40800,"parent_record_offset":40776,"ancestor_record_offset":40244,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":41156,"parent_record_offset":41132,"ancestor_record_offset":40244,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":42104,"parent_record_offset":42080,"ancestor_record_offset":40244,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":42532,"parent_record_offset":42508,"ancestor_record_offset":40244,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":42960,"parent_record_offset":42928,"ancestor_record_offset":40244,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":40672,"parent_record_offset":40644,"ancestor_record_offset":40244,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":40452,"parent_record_offset":40428,"ancestor_record_offset":40244,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":42828,"parent_record_offset":42804,"ancestor_record_offset":40244,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":40428,"parent_record_offset":40404,"ancestor_record_offset":40244,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":600,"record_offset":41696,"parent_record_offset":41448,"ancestor_record_offset":40244,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":600,"record_offset":40404,"parent_record_offset":40380,"ancestor_record_offset":40244,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":41620,"parent_record_offset":41580,"ancestor_record_offset":40244,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":600,"record_offset":41448,"parent_record_offset":41424,"ancestor_record_offset":40244,"inlinee":6636,"source_line":259,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":600,"record_offset":41512,"parent_record_offset":41484,"ancestor_record_offset":40244,"inlinee":38113,"source_line":103,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":600,"record_offset":42604,"parent_record_offset":42572,"ancestor_record_offset":40244,"inlinee":38114,"source_line":106,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":600,"record_offset":40528,"parent_record_offset":40492,"ancestor_record_offset":40244,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":40584,"parent_record_offset":40556,"ancestor_record_offset":40244,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":40700,"parent_record_offset":40644,"ancestor_record_offset":40244,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":40556,"parent_record_offset":40528,"ancestor_record_offset":40244,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":600,"record_offset":40644,"parent_record_offset":40612,"ancestor_record_offset":40244,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":600,"record_offset":40612,"parent_record_offset":40584,"ancestor_record_offset":40244,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 103
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x94E1
// 首条位置: {"module":600,"inlinee":38113}
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
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x94E2
// 首条位置: {"module":600,"inlinee":38114}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::thick_line_mesh
// 记录形式: compiled_procedure; instances=1
// 源码行: 122
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":600,"record_offset":43128,"rva":12885008,"va":5381594128}
// 编译实例 RVAs: 0xc49c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5915642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e806bca5ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5381594325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=7
//     evidence={"assembly":"\tcallq\t-5918210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8feb1a5ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5381596909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=4
//     target_aliases=1699 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12872125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e843963bff","size":5,"soft_fail":false,"target_va":5368722192,"va":5381594312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400e83d0
//     owner=unresolved_namespace; count=1
//     target_aliases=34 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11935858","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e88edf49ff","size":5,"soft_fail":false,"target_va":5369660368,"va":5381596221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990120
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2861640","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8b855d4ff","size":5,"soft_fail":false,"target_va":5378736416,"va":5381598051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1581743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e851dde7ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5381596442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415fdd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10172437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e815389b00","size":5,"soft_fail":false,"target_va":5391768944,"va":5381596502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416070b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10210241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8c1cb9b00","size":5,"soft_fail":false,"target_va":5391806640,"va":5381596394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f940
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34757102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8ee591202","size":5,"soft_fail":false,"target_va":5416352064,"va":5381594957,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34756171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e84b561202","size":5,"soft_fail":false,"target_va":5416353040,"va":5381596864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bd80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35463031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8771f1d02","size":5,"soft_fail":false,"target_va":5417057664,"va":5381594628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t35025347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8c3711602","size":5,"soft_fail":false,"target_va":5416620794,"va":5381595442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed::<usize, usize>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35023932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e83c6c1602","size":5,"soft_fail":false,"target_va":5416621188,"va":5381597251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t35024481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e8616e1602","size":5,"soft_fail":false,"target_va":5416621762,"va":5381597276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35025017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e879701602","size":5,"soft_fail":false,"target_va":5416622240,"va":5381597218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35021909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381594128,"raw_hex":"e855641602","size":5,"soft_fail":false,"target_va":5416619360,"va":5381597446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":48280,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":6757,"source_line":3499,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":44424,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":43316,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":600,"record_offset":44396,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$15::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":46280,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":11646,"source_line":1259,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":43984,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":600,"record_offset":47512,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":49028,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":600,"record_offset":47428,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":600,"record_offset":47040,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":30455,"source_line":40,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":600,"record_offset":48708,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":30734,"source_line":70,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":44128,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46488,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":12105,"source_line":674,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$105::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46796,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":11117,"source_line":20,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46400,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":600,"record_offset":44340,"parent_record_offset":43128,"ancestor_record_offset":43128,"inlinee":11750,"source_line":72,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43600,"parent_record_offset":43576,"ancestor_record_offset":43128,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43624,"parent_record_offset":43600,"ancestor_record_offset":43128,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":48920,"parent_record_offset":48896,"ancestor_record_offset":43128,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":48944,"parent_record_offset":48920,"ancestor_record_offset":43128,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":43128,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":48968,"parent_record_offset":48944,"ancestor_record_offset":43128,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":43576,"parent_record_offset":43432,"ancestor_record_offset":43128,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":600,"record_offset":48896,"parent_record_offset":48804,"ancestor_record_offset":43128,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":44524,"parent_record_offset":44492,"ancestor_record_offset":43128,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48408,"parent_record_offset":48380,"ancestor_record_offset":43128,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":43372,"parent_record_offset":43344,"ancestor_record_offset":43128,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48492,"parent_record_offset":48468,"ancestor_record_offset":43128,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48832,"parent_record_offset":48804,"ancestor_record_offset":43128,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48804,"parent_record_offset":48780,"ancestor_record_offset":43128,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48468,"parent_record_offset":48436,"ancestor_record_offset":43128,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":44572,"parent_record_offset":44548,"ancestor_record_offset":43128,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":44548,"parent_record_offset":44524,"ancestor_record_offset":43128,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48436,"parent_record_offset":48408,"ancestor_record_offset":43128,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":43432,"parent_record_offset":43400,"ancestor_record_offset":43128,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":43400,"parent_record_offset":43372,"ancestor_record_offset":43128,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":48780,"parent_record_offset":48756,"ancestor_record_offset":43128,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":600,"record_offset":43464,"parent_record_offset":43432,"ancestor_record_offset":43128,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":48336,"parent_record_offset":48308,"ancestor_record_offset":43128,"inlinee":6151,"source_line":2877,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":44492,"parent_record_offset":44452,"ancestor_record_offset":43128,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":48668,"parent_record_offset":48336,"ancestor_record_offset":43128,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":44452,"parent_record_offset":44424,"ancestor_record_offset":43128,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":48380,"parent_record_offset":48336,"ancestor_record_offset":43128,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":43344,"parent_record_offset":43316,"ancestor_record_offset":43128,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":600,"record_offset":47536,"parent_record_offset":47512,"ancestor_record_offset":43128,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\spec_extend.rs
//     evidence={"module":600,"record_offset":48308,"parent_record_offset":48280,"ancestor_record_offset":43128,"inlinee":6759,"source_line":54,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":600,"record_offset":43488,"parent_record_offset":43464,"ancestor_record_offset":43128,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":47288,"parent_record_offset":47260,"ancestor_record_offset":43128,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":47340,"parent_record_offset":47316,"ancestor_record_offset":43128,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":600,"record_offset":47316,"parent_record_offset":47260,"ancestor_record_offset":43128,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":44036,"parent_record_offset":44008,"ancestor_record_offset":43128,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$64::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":600,"record_offset":48240,"parent_record_offset":48212,"ancestor_record_offset":43128,"inlinee":5308,"source_line":1916,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":600,"record_offset":44364,"parent_record_offset":44340,"ancestor_record_offset":43128,"inlinee":11751,"source_line":1749,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":600,"record_offset":44296,"parent_record_offset":44272,"ancestor_record_offset":43128,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":600,"record_offset":45888,"parent_record_offset":45832,"ancestor_record_offset":43128,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$14::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":46304,"parent_record_offset":46280,"ancestor_record_offset":43128,"inlinee":11648,"source_line":1164,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":44064,"parent_record_offset":44008,"ancestor_record_offset":43128,"inlinee":5179,"source_line":211,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":600,"record_offset":44008,"parent_record_offset":43984,"ancestor_record_offset":43128,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":600,"record_offset":47560,"parent_record_offset":47536,"ancestor_record_offset":43128,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":600,"record_offset":47364,"parent_record_offset":47340,"ancestor_record_offset":43128,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":43512,"parent_record_offset":43488,"ancestor_record_offset":43128,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":43536,"parent_record_offset":43512,"ancestor_record_offset":43128,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":44088,"parent_record_offset":44064,"ancestor_record_offset":43128,"inlinee":5180,"source_line":832,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":600,"record_offset":48864,"parent_record_offset":48832,"ancestor_record_offset":43128,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":46728,"parent_record_offset":46700,"ancestor_record_offset":43128,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":600,"record_offset":44184,"parent_record_offset":44152,"ancestor_record_offset":43128,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\range.rs
//     evidence={"module":600,"record_offset":46336,"parent_record_offset":46304,"ancestor_record_offset":43128,"inlinee":10957,"source_line":559,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":600,"record_offset":47232,"parent_record_offset":47204,"ancestor_record_offset":43128,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":48544,"parent_record_offset":48336,"ancestor_record_offset":43128,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":43128,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":600,"record_offset":47476,"parent_record_offset":47452,"ancestor_record_offset":43128,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":600,"record_offset":44612,"parent_record_offset":44452,"ancestor_record_offset":43128,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":600,"record_offset":47452,"parent_record_offset":47428,"ancestor_record_offset":43128,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":44240,"parent_record_offset":44152,"ancestor_record_offset":43128,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46568,"parent_record_offset":46544,"ancestor_record_offset":43128,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46544,"parent_record_offset":46512,"ancestor_record_offset":43128,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46512,"parent_record_offset":46488,"ancestor_record_offset":43128,"inlinee":12106,"source_line":657,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":46700,"parent_record_offset":46512,"ancestor_record_offset":43128,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":600,"record_offset":44152,"parent_record_offset":44128,"ancestor_record_offset":43128,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":600,"record_offset":47076,"parent_record_offset":47040,"ancestor_record_offset":43128,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":600,"record_offset":44272,"parent_record_offset":44240,"ancestor_record_offset":43128,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":47140,"parent_record_offset":47108,"ancestor_record_offset":43128,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":47260,"parent_record_offset":47204,"ancestor_record_offset":43128,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":600,"record_offset":47108,"parent_record_offset":47076,"ancestor_record_offset":43128,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":600,"record_offset":47204,"parent_record_offset":47172,"ancestor_record_offset":43128,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":600,"record_offset":47172,"parent_record_offset":47140,"ancestor_record_offset":43128,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00600.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::thick_line_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 181
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DAD
// 首条位置: {"module":1643,"inlinee":81325}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::to_array
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":1643,"record_offset":499864,"parent_record_offset":499828,"ancestor_record_offset":499828,"inlinee":49315,"source_line":140,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::thick_line_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 188
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D92
// 首条位置: {"module":1643,"inlinee":81298}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 28; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]
// [文件级接口候选，具体函数归属 unknown] ref$<str$> (*)(ref$<alloc::string::String>)
// 候选参数类型: ["ref$<alloc::string::String>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<std::fs::DirEntry> > (*)(enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >)
// 候选参数类型: ["enum2$<core::result::Result<std::fs::DirEntry,std::io::error::Error> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>,tuple$<> > > (*)(tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>)
// 候选参数类型: ["tuple$<ref$<country_core::resources::walls::public_wall_state::PublicWallState>,country_core::systems::ui_wall_transform_gizmos::WallWireMetrics>"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)
// 候选参数类型: ["rhapsody::raw::buffer::Buffer"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts
// 信息 ID: 5579bd46aa7b3fcd713e2182
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::closure$0 @ 0x9c33b8 -> 0xc49880 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1432955 -> 0xc49880 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$0
// 信息 ID: 9817b8980119eb7ae29be96d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=600, inline_record=41512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$1
// 信息 ID: 1fff20ff0e60dfc9be3c7901
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=600, inline_record=42604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::rewire_enclosed_curve
// 信息 ID: f25060274857e88b4844b5ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0xb8acd7 -> 0xc4ae20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::thick_line_mesh
// 信息 ID: 4a28c27075a1249751aa40ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::closure$0 @ 0x9c34a3 -> 0xc49c10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::thick_line_mesh::closure$0
// 信息 ID: 14b28a31148083411dbabbc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::geo::thick_line_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::geo::thick_line_mesh::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<glam::f32::vec3::Vec3>,array$<array$<f32,3>,2>,country_core::utils::geo::thick_line_mesh::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<glam::f32::vec3::Vec3>,array$<array$<f32,3>,2>,country_core::utils::geo::thick_line_mesh::closure_env$0> >; module=1643, inline_record=499828; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::thick_line_mesh::closure$1
// 信息 ID: c267b83c529aee26697d34c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::thick_line_mesh", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::geo::thick_line_mesh::closure_env$1 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<array$<f32,3> >,array$<array$<f32,3>,2>,country_core::utils::geo::thick_line_mesh::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<array$<f32,3> >,array$<array$<f32,3>,2>,country_core::utils::geo::thick_line_mesh::closure_env$1> >; module=1643, inline_record=361740; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh::closure$1
// 信息 ID: 544012e4aecb0241e50dfdf9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::update_curve_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >; module=1700, inline_record=130632; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >; module=1700, inline_record=133080; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >; module=1700, inline_record=136240; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >; module=1700, inline_record=138004; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >
// 信息 ID: a231bd1d5af3f63c7d1d5d43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["glam::f32::vec3::Vec3", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x9211bc -> 0x19774e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >
// 信息 ID: 245010e283fbf700037f0314
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation @ 0x92006d -> 0x19777c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >
// 信息 ID: 4bf90cbbdb601929786ed49f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xc0ba57 -> 0x1977a70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >
// 信息 ID: 7372b044fbf0ec9d768fc1bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims @ 0xb7a4bd -> 0x1977320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
