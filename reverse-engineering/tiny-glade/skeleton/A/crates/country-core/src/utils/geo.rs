// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/utils/geo.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 4a8bd1bc8ed072bc0ecc0439cb03de92b56540491a51cd695c318eb133636662
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: geo.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 423, 454, 1308, 1321
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
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":423,"record_offset":41636,"rva":5942400,"va":5374651520}
// 编译实例 RVAs: 0x5aac80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1902247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e859f9e2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374651938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1902414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e8b2f8e2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374652121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t84255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e81f490100","size":5,"soft_fail":false,"target_va":5374736096,"va":5374651836,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8158807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e8577e7c00","size":5,"soft_fail":false,"target_va":5382810688,"va":5374651876,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28325945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e83938b001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374652194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28326732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e84c3bb001","size":5,"soft_fail":false,"target_va":5402978912,"va":5374652175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-83986","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651520,"raw_hex":"e8eeb7feff","size":5,"soft_fail":false,"target_va":5374567616,"va":5374651597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":423,"record_offset":41860,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":42008,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":41972,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":43924,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":423,"record_offset":43656,"parent_record_offset":41636,"ancestor_record_offset":41636,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":42940,"parent_record_offset":42916,"ancestor_record_offset":41636,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":42964,"parent_record_offset":42940,"ancestor_record_offset":41636,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":43876,"parent_record_offset":43852,"ancestor_record_offset":41636,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":42916,"parent_record_offset":42824,"ancestor_record_offset":41636,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":43852,"parent_record_offset":43744,"ancestor_record_offset":41636,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":43780,"parent_record_offset":43744,"ancestor_record_offset":41636,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":42824,"parent_record_offset":42796,"ancestor_record_offset":41636,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":42796,"parent_record_offset":42768,"ancestor_record_offset":41636,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":43036,"parent_record_offset":42796,"ancestor_record_offset":41636,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":43744,"parent_record_offset":43712,"ancestor_record_offset":41636,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":43104,"parent_record_offset":43076,"ancestor_record_offset":41636,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":42740,"parent_record_offset":42700,"ancestor_record_offset":41636,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":42768,"parent_record_offset":42740,"ancestor_record_offset":41636,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":42036,"parent_record_offset":42008,"ancestor_record_offset":41636,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":43516,"parent_record_offset":43488,"ancestor_record_offset":41636,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":42064,"parent_record_offset":42036,"ancestor_record_offset":41636,"inlinee":8657,"source_line":232,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::in_place_collect::impl$3::collect_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\in_place_collect.rs
//     evidence={"module":423,"record_offset":42104,"parent_record_offset":42064,"ancestor_record_offset":41636,"inlinee":8550,"source_line":377,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":43948,"parent_record_offset":43924,"ancestor_record_offset":41636,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":43996,"parent_record_offset":43972,"ancestor_record_offset":41636,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":42424,"parent_record_offset":42392,"ancestor_record_offset":41636,"inlinee":6148,"source_line":292,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":423,"record_offset":41884,"parent_record_offset":41860,"ancestor_record_offset":41636,"inlinee":6045,"source_line":223,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":423,"record_offset":43580,"parent_record_offset":43516,"ancestor_record_offset":41636,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":423,"record_offset":43076,"parent_record_offset":42700,"ancestor_record_offset":41636,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":423,"record_offset":42668,"parent_record_offset":42636,"ancestor_record_offset":41636,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":423,"record_offset":42700,"parent_record_offset":42668,"ancestor_record_offset":41636,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":42860,"parent_record_offset":42824,"ancestor_record_offset":41636,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":42884,"parent_record_offset":42860,"ancestor_record_offset":41636,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":42228,"parent_record_offset":42188,"ancestor_record_offset":41636,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":42328,"parent_record_offset":42104,"ancestor_record_offset":41636,"inlinee":8551,"source_line":133,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":43160,"parent_record_offset":43132,"ancestor_record_offset":41636,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":43424,"parent_record_offset":43188,"ancestor_record_offset":41636,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$19::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":42392,"parent_record_offset":42360,"ancestor_record_offset":41636,"inlinee":8555,"source_line":636,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::try_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":423,"record_offset":42360,"parent_record_offset":42328,"ancestor_record_offset":41636,"inlinee":8553,"source_line":613,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":42260,"parent_record_offset":42188,"ancestor_record_offset":41636,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":42188,"parent_record_offset":42152,"ancestor_record_offset":41636,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":42152,"parent_record_offset":42104,"ancestor_record_offset":41636,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":43188,"parent_record_offset":43160,"ancestor_record_offset":41636,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":43132,"parent_record_offset":43104,"ancestor_record_offset":41636,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":43488,"parent_record_offset":43424,"ancestor_record_offset":41636,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":42284,"parent_record_offset":42260,"ancestor_record_offset":41636,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":43816,"parent_record_offset":43780,"ancestor_record_offset":41636,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":41932,"parent_record_offset":41908,"ancestor_record_offset":41636,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":43684,"parent_record_offset":43656,"ancestor_record_offset":41636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":42456,"parent_record_offset":42424,"ancestor_record_offset":41636,"inlinee":6149,"source_line":971,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":41908,"parent_record_offset":41884,"ancestor_record_offset":41636,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":42496,"parent_record_offset":42456,"ancestor_record_offset":41636,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":42992,"parent_record_offset":42964,"ancestor_record_offset":41636,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":42552,"parent_record_offset":42104,"ancestor_record_offset":41636,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":423,"record_offset":43456,"parent_record_offset":43424,"ancestor_record_offset":41636,"inlinee":24562,"source_line":22,"archive":"_pdb/A/modules/00423.jsonl.gz"}
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
// 首条位置: {"module":423,"record_offset":39880,"rva":5941936,"va":5374651056}
// 编译实例 RVAs: 0x5aaab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1901574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8fafbe2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374651265,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1901741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e853fbe2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374651448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t84926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8be4b0100","size":5,"soft_fail":false,"target_va":5374736096,"va":5374651165,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8159475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8f3807c00","size":5,"soft_fail":false,"target_va":5382810688,"va":5374651208,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76e80
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8176502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e876c37c00","size":5,"soft_fail":false,"target_va":5382827648,"va":5374651141,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28326629","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8e53ab001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374651510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28327410","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8f23db001","size":5,"soft_fail":false,"target_va":5402978912,"va":5374651497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-83495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374651056,"raw_hex":"e8d9b9feff","size":5,"soft_fail":false,"target_va":5374567616,"va":5374651106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::copied::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\copied.rs
//     evidence={"module":423,"record_offset":40108,"parent_record_offset":39880,"ancestor_record_offset":39880,"inlinee":24564,"source_line":145,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":40256,"parent_record_offset":39880,"ancestor_record_offset":39880,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":40220,"parent_record_offset":39880,"ancestor_record_offset":39880,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":423,"record_offset":41364,"parent_record_offset":39880,"ancestor_record_offset":39880,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":40648,"parent_record_offset":40624,"ancestor_record_offset":39880,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":40672,"parent_record_offset":40648,"ancestor_record_offset":39880,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41584,"parent_record_offset":41560,"ancestor_record_offset":39880,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":40624,"parent_record_offset":40532,"ancestor_record_offset":39880,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":41560,"parent_record_offset":41452,"ancestor_record_offset":39880,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":41488,"parent_record_offset":41452,"ancestor_record_offset":39880,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":40532,"parent_record_offset":40504,"ancestor_record_offset":39880,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":40504,"parent_record_offset":40476,"ancestor_record_offset":39880,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":40744,"parent_record_offset":40504,"ancestor_record_offset":39880,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":41452,"parent_record_offset":41420,"ancestor_record_offset":39880,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40812,"parent_record_offset":40784,"ancestor_record_offset":39880,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40448,"parent_record_offset":40408,"ancestor_record_offset":39880,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40476,"parent_record_offset":40448,"ancestor_record_offset":39880,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":40280,"parent_record_offset":40256,"ancestor_record_offset":39880,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":41224,"parent_record_offset":41196,"ancestor_record_offset":39880,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":423,"record_offset":41288,"parent_record_offset":41224,"ancestor_record_offset":39880,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":423,"record_offset":40784,"parent_record_offset":40408,"ancestor_record_offset":39880,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":423,"record_offset":40376,"parent_record_offset":40344,"ancestor_record_offset":39880,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":423,"record_offset":40408,"parent_record_offset":40376,"ancestor_record_offset":39880,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":40568,"parent_record_offset":40532,"ancestor_record_offset":39880,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":40592,"parent_record_offset":40568,"ancestor_record_offset":39880,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":41096,"parent_record_offset":40984,"ancestor_record_offset":39880,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":40868,"parent_record_offset":40840,"ancestor_record_offset":39880,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":41132,"parent_record_offset":40896,"ancestor_record_offset":39880,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":41028,"parent_record_offset":40984,"ancestor_record_offset":39880,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":40984,"parent_record_offset":40944,"ancestor_record_offset":39880,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":40944,"parent_record_offset":40896,"ancestor_record_offset":39880,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":40896,"parent_record_offset":40868,"ancestor_record_offset":39880,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":40840,"parent_record_offset":40812,"ancestor_record_offset":39880,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":41196,"parent_record_offset":41132,"ancestor_record_offset":39880,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":41056,"parent_record_offset":41028,"ancestor_record_offset":39880,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":41524,"parent_record_offset":41488,"ancestor_record_offset":39880,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":423,"record_offset":40180,"parent_record_offset":40156,"ancestor_record_offset":39880,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":41392,"parent_record_offset":41364,"ancestor_record_offset":39880,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":423,"record_offset":40156,"parent_record_offset":40132,"ancestor_record_offset":39880,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":40700,"parent_record_offset":40672,"ancestor_record_offset":39880,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":41256,"parent_record_offset":41224,"ancestor_record_offset":39880,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":40132,"parent_record_offset":40108,"ancestor_record_offset":39880,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":423,"record_offset":41164,"parent_record_offset":41132,"ancestor_record_offset":39880,"inlinee":24562,"source_line":22,"archive":"_pdb/A/modules/00423.jsonl.gz"}
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
// 首条位置: {"module":423,"record_offset":36332,"rva":5941056,"va":5374650176}
// 编译实例 RVAs: 0x5aa740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1900679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e879ffe2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374650370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1900845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e8d3fee2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374650552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t85819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e83b4f0100","size":5,"soft_fail":false,"target_va":5374736096,"va":5374650272,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8160368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e870847c00","size":5,"soft_fail":false,"target_va":5382810688,"va":5374650315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d77bb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8180771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e823d47c00","size":5,"soft_fail":false,"target_va":5382831024,"va":5374650248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28327523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e8633eb001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374650616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28328304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e87041b001","size":5,"soft_fail":false,"target_va":5402978912,"va":5374650603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-82596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650176,"raw_hex":"e85cbdfeff","size":5,"soft_fail":false,"target_va":5374567616,"va":5374650207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":36664,"parent_record_offset":36332,"ancestor_record_offset":36332,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":36756,"parent_record_offset":36332,"ancestor_record_offset":36332,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":36720,"parent_record_offset":36332,"ancestor_record_offset":36332,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":423,"record_offset":37864,"parent_record_offset":36332,"ancestor_record_offset":36332,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":37148,"parent_record_offset":37124,"ancestor_record_offset":36332,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":37172,"parent_record_offset":37148,"ancestor_record_offset":36332,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":38084,"parent_record_offset":38060,"ancestor_record_offset":36332,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":37124,"parent_record_offset":37032,"ancestor_record_offset":36332,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":38060,"parent_record_offset":37952,"ancestor_record_offset":36332,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":37988,"parent_record_offset":37952,"ancestor_record_offset":36332,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":37032,"parent_record_offset":37004,"ancestor_record_offset":36332,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":37004,"parent_record_offset":36976,"ancestor_record_offset":36332,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":37244,"parent_record_offset":37004,"ancestor_record_offset":36332,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":37952,"parent_record_offset":37920,"ancestor_record_offset":36332,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":37312,"parent_record_offset":37284,"ancestor_record_offset":36332,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36948,"parent_record_offset":36908,"ancestor_record_offset":36332,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36976,"parent_record_offset":36948,"ancestor_record_offset":36332,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":36780,"parent_record_offset":36756,"ancestor_record_offset":36332,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":37724,"parent_record_offset":37696,"ancestor_record_offset":36332,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":423,"record_offset":37788,"parent_record_offset":37724,"ancestor_record_offset":36332,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":423,"record_offset":37284,"parent_record_offset":36908,"ancestor_record_offset":36332,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":423,"record_offset":36876,"parent_record_offset":36844,"ancestor_record_offset":36332,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":423,"record_offset":36908,"parent_record_offset":36876,"ancestor_record_offset":36332,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":37068,"parent_record_offset":37032,"ancestor_record_offset":36332,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":37092,"parent_record_offset":37068,"ancestor_record_offset":36332,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":37596,"parent_record_offset":37484,"ancestor_record_offset":36332,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":37368,"parent_record_offset":37340,"ancestor_record_offset":36332,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":37632,"parent_record_offset":37396,"ancestor_record_offset":36332,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":37528,"parent_record_offset":37484,"ancestor_record_offset":36332,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":37484,"parent_record_offset":37444,"ancestor_record_offset":36332,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":37444,"parent_record_offset":37396,"ancestor_record_offset":36332,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":37396,"parent_record_offset":37368,"ancestor_record_offset":36332,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":37340,"parent_record_offset":37312,"ancestor_record_offset":36332,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":37696,"parent_record_offset":37632,"ancestor_record_offset":36332,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":37556,"parent_record_offset":37528,"ancestor_record_offset":36332,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":38024,"parent_record_offset":37988,"ancestor_record_offset":36332,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":37892,"parent_record_offset":37864,"ancestor_record_offset":36332,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":37200,"parent_record_offset":37172,"ancestor_record_offset":36332,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":37756,"parent_record_offset":37724,"ancestor_record_offset":36332,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":423,"record_offset":36688,"parent_record_offset":36664,"ancestor_record_offset":36332,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":423,"record_offset":37664,"parent_record_offset":37632,"ancestor_record_offset":36332,"inlinee":24562,"source_line":22,"archive":"_pdb/A/modules/00423.jsonl.gz"}
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
// 首条位置: {"module":423,"record_offset":38136,"rva":5941504,"va":5374650624}
// 编译实例 RVAs: 0x5aa900
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1901124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e8bcfde2ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374650815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1901277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e823fde2ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374650984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t85374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e87e4d0100","size":5,"soft_fail":false,"target_va":5374736096,"va":5374650717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8159923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e8b3827c00","size":5,"soft_fail":false,"target_va":5382810688,"va":5374650760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d78fb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8185446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e866e67c00","size":5,"soft_fail":false,"target_va":5382836144,"va":5374650693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28327093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e8b53cb001","size":5,"soft_fail":false,"target_va":5402978144,"va":5374651046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28327874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e8c23fb001","size":5,"soft_fail":false,"target_va":5402978912,"va":5374651033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-83038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374650624,"raw_hex":"e8a2bbfeff","size":5,"soft_fail":false,"target_va":5374567616,"va":5374650649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":38500,"parent_record_offset":38136,"ancestor_record_offset":38136,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":423,"record_offset":38464,"parent_record_offset":38136,"ancestor_record_offset":38136,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":423,"record_offset":39608,"parent_record_offset":38136,"ancestor_record_offset":38136,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":38892,"parent_record_offset":38868,"ancestor_record_offset":38136,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":38916,"parent_record_offset":38892,"ancestor_record_offset":38136,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39828,"parent_record_offset":39804,"ancestor_record_offset":38136,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":38868,"parent_record_offset":38776,"ancestor_record_offset":38136,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":423,"record_offset":39804,"parent_record_offset":39696,"ancestor_record_offset":38136,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":39732,"parent_record_offset":39696,"ancestor_record_offset":38136,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":38776,"parent_record_offset":38748,"ancestor_record_offset":38136,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":38748,"parent_record_offset":38720,"ancestor_record_offset":38136,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":38988,"parent_record_offset":38748,"ancestor_record_offset":38136,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":423,"record_offset":39696,"parent_record_offset":39664,"ancestor_record_offset":38136,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":39056,"parent_record_offset":39028,"ancestor_record_offset":38136,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":38692,"parent_record_offset":38652,"ancestor_record_offset":38136,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":38720,"parent_record_offset":38692,"ancestor_record_offset":38136,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":38524,"parent_record_offset":38500,"ancestor_record_offset":38136,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":423,"record_offset":39468,"parent_record_offset":39440,"ancestor_record_offset":38136,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":423,"record_offset":39532,"parent_record_offset":39468,"ancestor_record_offset":38136,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":423,"record_offset":39028,"parent_record_offset":38652,"ancestor_record_offset":38136,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":423,"record_offset":38620,"parent_record_offset":38588,"ancestor_record_offset":38136,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":423,"record_offset":38652,"parent_record_offset":38620,"ancestor_record_offset":38136,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":38812,"parent_record_offset":38776,"ancestor_record_offset":38136,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":423,"record_offset":38836,"parent_record_offset":38812,"ancestor_record_offset":38136,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":423,"record_offset":39340,"parent_record_offset":39228,"ancestor_record_offset":38136,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":39112,"parent_record_offset":39084,"ancestor_record_offset":38136,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":423,"record_offset":39376,"parent_record_offset":39140,"ancestor_record_offset":38136,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":39272,"parent_record_offset":39228,"ancestor_record_offset":38136,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":39228,"parent_record_offset":39188,"ancestor_record_offset":38136,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":423,"record_offset":39188,"parent_record_offset":39140,"ancestor_record_offset":38136,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":39140,"parent_record_offset":39112,"ancestor_record_offset":38136,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":39084,"parent_record_offset":39056,"ancestor_record_offset":38136,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":423,"record_offset":39440,"parent_record_offset":39376,"ancestor_record_offset":38136,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":39300,"parent_record_offset":39272,"ancestor_record_offset":38136,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":423,"record_offset":39768,"parent_record_offset":39732,"ancestor_record_offset":38136,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":39636,"parent_record_offset":39608,"ancestor_record_offset":38136,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":38944,"parent_record_offset":38916,"ancestor_record_offset":38136,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":423,"record_offset":39500,"parent_record_offset":39468,"ancestor_record_offset":38136,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::update_curve_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":423,"record_offset":39408,"parent_record_offset":39376,"ancestor_record_offset":38136,"inlinee":24562,"source_line":22,"archive":"_pdb/A/modules/00423.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::update_curve_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5FF2
// 首条位置: {"module":423,"inlinee":24562}
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
// 首条位置: {"module":454,"record_offset":26180,"rva":6429216,"va":5375138336}
// 编译实例 RVAs: 0x621a20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2388966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e81a8cdbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375138673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e891e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8811741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e8dd748600","size":5,"soft_fail":false,"target_va":5383950816,"va":5375139070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e8d010
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8828299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e88bb58600","size":5,"soft_fail":false,"target_va":5383966736,"va":5375138432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28454364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e8dc2db201","size":5,"soft_fail":false,"target_va":5403593360,"va":5375138991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28454550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e8962eb201","size":5,"soft_fail":false,"target_va":5403593376,"va":5375138821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27914620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e87cf1a901","size":5,"soft_fail":false,"target_va":5403053552,"va":5375138927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27839509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e815cca801","size":5,"soft_fail":false,"target_va":5402978880,"va":5375139366,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t27840086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e856cea801","size":5,"soft_fail":false,"target_va":5402979344,"va":5375139253,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27840700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375138336,"raw_hex":"e8bcd0a801","size":5,"soft_fail":false,"target_va":5402980128,"va":5375139423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":27956,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":27680,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":8653,"source_line":1598,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":26344,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":27984,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":27768,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":26688,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":454,"record_offset":27740,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":12681,"source_line":176,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":26536,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::new_from_points
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":454,"record_offset":28908,"parent_record_offset":26180,"ancestor_record_offset":26180,"inlinee":21342,"source_line":216,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":26884,"parent_record_offset":26860,"ancestor_record_offset":26180,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":26860,"parent_record_offset":26760,"ancestor_record_offset":26180,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":26792,"parent_record_offset":26760,"ancestor_record_offset":26180,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":28140,"parent_record_offset":28112,"ancestor_record_offset":26180,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":28252,"parent_record_offset":28208,"ancestor_record_offset":26180,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":28112,"parent_record_offset":28088,"ancestor_record_offset":26180,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":26760,"parent_record_offset":26736,"ancestor_record_offset":26180,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":28048,"parent_record_offset":28016,"ancestor_record_offset":26180,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":28208,"parent_record_offset":28048,"ancestor_record_offset":26180,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":28284,"parent_record_offset":28048,"ancestor_record_offset":26180,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":28088,"parent_record_offset":28048,"ancestor_record_offset":26180,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":454,"record_offset":28340,"parent_record_offset":28316,"ancestor_record_offset":26180,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":454,"record_offset":28388,"parent_record_offset":28364,"ancestor_record_offset":26180,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$2::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":454,"record_offset":28016,"parent_record_offset":27984,"ancestor_record_offset":26180,"inlinee":4434,"source_line":31,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":454,"record_offset":27704,"parent_record_offset":27680,"ancestor_record_offset":26180,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":26824,"parent_record_offset":26792,"ancestor_record_offset":26180,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":28164,"parent_record_offset":28140,"ancestor_record_offset":26180,"inlinee":4907,"source_line":1680,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":454,"record_offset":26588,"parent_record_offset":26560,"ancestor_record_offset":26180,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":26712,"parent_record_offset":26688,"ancestor_record_offset":26180,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":454,"record_offset":28936,"parent_record_offset":28908,"ancestor_record_offset":26180,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":26372,"parent_record_offset":26344,"ancestor_record_offset":26180,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":454,"record_offset":26400,"parent_record_offset":26372,"ancestor_record_offset":26180,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":26616,"parent_record_offset":26560,"ancestor_record_offset":26180,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":454,"record_offset":26560,"parent_record_offset":26536,"ancestor_record_offset":26180,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":454,"record_offset":26648,"parent_record_offset":26616,"ancestor_record_offset":26180,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/00454.jsonl.gz"}
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
// 首条位置: {"module":454,"record_offset":30432,"rva":6430976,"va":5375140096}
// 编译实例 RVAs: 0x622100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2515170","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e81e9fd9ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5375140189,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2390611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e8ad85dbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5375140302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2391052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e8f483dbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5375140759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bf6e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-404415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e841d4f9ff","size":5,"soft_fail":false,"target_va":5374736096,"va":5375140506,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e4670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t795151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e80f220c00","size":5,"soft_fail":false,"target_va":5375936112,"va":5375140956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d72c40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: geo.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7670141","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e87d097500","size":5,"soft_fail":false,"target_va":5382810688,"va":5375140542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t27837234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e832c3a801","size":5,"soft_fail":false,"target_va":5402978144,"va":5375140905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2386225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e831692400","size":5,"soft_fail":false,"target_va":5377527120,"va":5375140890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t27838262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e836c7a801","size":5,"soft_fail":false,"target_va":5402979152,"va":5375140885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2226052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375140096,"raw_hex":"e884f72100","size":5,"soft_fail":false,"target_va":5377366224,"va":5375140167,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":31136,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":30568,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":454,"record_offset":30652,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":24287,"source_line":24,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::mesh::Mesh::set_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":454,"record_offset":33032,"parent_record_offset":30432,"ancestor_record_offset":30432,"inlinee":24560,"source_line":48,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":31468,"parent_record_offset":31444,"ancestor_record_offset":30432,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":31492,"parent_record_offset":31468,"ancestor_record_offset":30432,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":33232,"parent_record_offset":33208,"ancestor_record_offset":30432,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":31444,"parent_record_offset":31356,"ancestor_record_offset":30432,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":454,"record_offset":33208,"parent_record_offset":33108,"ancestor_record_offset":30432,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":33140,"parent_record_offset":33108,"ancestor_record_offset":30432,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":31356,"parent_record_offset":31328,"ancestor_record_offset":30432,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":31328,"parent_record_offset":31300,"ancestor_record_offset":30432,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":31564,"parent_record_offset":31328,"ancestor_record_offset":30432,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":454,"record_offset":33108,"parent_record_offset":33080,"ancestor_record_offset":30432,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":31632,"parent_record_offset":31608,"ancestor_record_offset":30432,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":31272,"parent_record_offset":31232,"ancestor_record_offset":30432,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":31300,"parent_record_offset":31272,"ancestor_record_offset":30432,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":31168,"parent_record_offset":31136,"ancestor_record_offset":30432,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":454,"record_offset":31836,"parent_record_offset":31804,"ancestor_record_offset":30432,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":454,"record_offset":31912,"parent_record_offset":31836,"ancestor_record_offset":30432,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":454,"record_offset":31608,"parent_record_offset":31232,"ancestor_record_offset":30432,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":454,"record_offset":31200,"parent_record_offset":31168,"ancestor_record_offset":30432,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":454,"record_offset":31232,"parent_record_offset":31200,"ancestor_record_offset":30432,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":454,"record_offset":31388,"parent_record_offset":31356,"ancestor_record_offset":30432,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":454,"record_offset":31412,"parent_record_offset":31388,"ancestor_record_offset":30432,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":30964,"parent_record_offset":30940,"ancestor_record_offset":30432,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":31016,"parent_record_offset":30992,"ancestor_record_offset":30432,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":454,"record_offset":30992,"parent_record_offset":30940,"ancestor_record_offset":30432,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":454,"record_offset":32764,"parent_record_offset":32656,"ancestor_record_offset":30432,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":454,"record_offset":31680,"parent_record_offset":31656,"ancestor_record_offset":30432,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":454,"record_offset":31740,"parent_record_offset":31704,"ancestor_record_offset":30432,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":454,"record_offset":32696,"parent_record_offset":32656,"ancestor_record_offset":30432,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":454,"record_offset":32656,"parent_record_offset":32620,"ancestor_record_offset":30432,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":454,"record_offset":32620,"parent_record_offset":32576,"ancestor_record_offset":30432,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":32576,"parent_record_offset":32552,"ancestor_record_offset":30432,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":31656,"parent_record_offset":31632,"ancestor_record_offset":30432,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":454,"record_offset":31804,"parent_record_offset":31740,"ancestor_record_offset":30432,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":454,"record_offset":31040,"parent_record_offset":31016,"ancestor_record_offset":30432,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":32724,"parent_record_offset":32696,"ancestor_record_offset":30432,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":454,"record_offset":33172,"parent_record_offset":33140,"ancestor_record_offset":30432,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":454,"record_offset":30900,"parent_record_offset":30876,"ancestor_record_offset":30432,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":454,"record_offset":30616,"parent_record_offset":30592,"ancestor_record_offset":30432,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":33056,"parent_record_offset":33032,"ancestor_record_offset":30432,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":31952,"parent_record_offset":31704,"ancestor_record_offset":30432,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":454,"record_offset":30592,"parent_record_offset":30568,"ancestor_record_offset":30432,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":31520,"parent_record_offset":31492,"ancestor_record_offset":30432,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":454,"record_offset":31064,"parent_record_offset":31040,"ancestor_record_offset":30432,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":454,"record_offset":31704,"parent_record_offset":31680,"ancestor_record_offset":30432,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":454,"record_offset":30676,"parent_record_offset":30652,"ancestor_record_offset":30432,"inlinee":22684,"source_line":11,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":454,"record_offset":31768,"parent_record_offset":31740,"ancestor_record_offset":30432,"inlinee":26064,"source_line":101,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] country_core::utils::geo::mesh_from_pts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/geo.rs
//     evidence={"module":454,"record_offset":32832,"parent_record_offset":32800,"ancestor_record_offset":30432,"inlinee":26065,"source_line":104,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":454,"record_offset":30704,"parent_record_offset":30676,"ancestor_record_offset":30432,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":454,"record_offset":30752,"parent_record_offset":30728,"ancestor_record_offset":30432,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":454,"record_offset":30828,"parent_record_offset":30800,"ancestor_record_offset":30432,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":454,"record_offset":30940,"parent_record_offset":30800,"ancestor_record_offset":30432,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":454,"record_offset":30728,"parent_record_offset":30704,"ancestor_record_offset":30432,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":454,"record_offset":30852,"parent_record_offset":30828,"ancestor_record_offset":30432,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":454,"record_offset":30876,"parent_record_offset":30852,"ancestor_record_offset":30432,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":454,"record_offset":30800,"parent_record_offset":30776,"ancestor_record_offset":30432,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":454,"record_offset":30776,"parent_record_offset":30752,"ancestor_record_offset":30432,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::geo::mesh_from_pts::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65D0
// 首条位置: {"module":454,"inlinee":26064}
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
// IPI IDs: 0x65D1
// 首条位置: {"module":454,"inlinee":26065}
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
// 函数: country_core::utils::geo::mesh_from_pts
// 信息 ID: d4f74b2812ec246e02208fc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0xcc3246 -> 0x622100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$0
// 信息 ID: 3b24aa4ac9e1aac06fa185ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=454, inline_record=31768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::mesh_from_pts::closure$1
// 信息 ID: 5e24d663446c7eaf84ad11b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::mesh_from_pts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=454, inline_record=32832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::rewire_enclosed_curve
// 信息 ID: cd47b3fddc161519f6cf45c2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::eraser::wall_eraser::wall_eraser @ 0x5a7359 -> 0x621a20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh::closure$1
// 信息 ID: 6d2d9d319fb70ab8fffdb45c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::utils::geo::update_curve_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >; module=423, inline_record=37664; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >; module=423, inline_record=39408; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >; module=423, inline_record=41164; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >; module=423, inline_record=43456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >
// 信息 ID: cf7b76d256988dea9b5ea681
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["glam::f32::vec3::Vec3", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged @ 0x5c491c -> 0x5aac80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >
// 信息 ID: 9df2115b1fac750b62dd7bbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::copied::Copied -> ["core::slice::iter::Iter<glam::f32::vec3::Vec3>"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation @ 0x64179a -> 0x5aaab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1> >
// 信息 ID: e4f43df3943ba08b9caabdef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line @ 0xf0321a -> 0x5aa740 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >
// 信息 ID: 16898a051794c71bbef2022d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<glam::f32::vec2::Vec2>", "country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1"]; 字段布局 = unknown。
// [类型关系] country_core::utils::geo::update_curve_mesh -> ["core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims @ 0x1804be3 -> 0x5aa900 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
