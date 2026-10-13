// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2085361b53b63bda61fe69b76c611df39f8c14e86c2ce10456089e5d1894656a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pipeline_desc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 56, 136, 145, 390, 400, 451, 462, 489, 492, 1331, 1454
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"PDB_registry_path_only","version":"0.8.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$8::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF29F
// 首条位置: {"module":1331,"inlinee":62111}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6944,"parent_record_offset":6912,"ancestor_record_offset":6912,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":7044,"parent_record_offset":7016,"ancestor_record_offset":6912,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":6984,"parent_record_offset":6944,"ancestor_record_offset":6912,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":7016,"parent_record_offset":6944,"ancestor_record_offset":6912,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x105DB
// 首条位置: {"module":1454,"inlinee":67035}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1454,"record_offset":992,"parent_record_offset":944,"ancestor_record_offset":944,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":1064,"parent_record_offset":944,"ancestor_record_offset":944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::shader_defines::impl$1::clone
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":1454,"record_offset":1032,"parent_record_offset":944,"ancestor_record_offset":944,"inlinee":67037,"source_line":3,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":944,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1454,"record_offset":1200,"parent_record_offset":1144,"ancestor_record_offset":944,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1172,"parent_record_offset":1144,"ancestor_record_offset":944,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1454,"record_offset":1144,"parent_record_offset":1116,"ancestor_record_offset":944,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1454,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":944,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01454.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B71
// 首条位置: {"module":489,"inlinee":27505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":489,"record_offset":36712,"parent_record_offset":36676,"ancestor_record_offset":36676,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":36844,"parent_record_offset":36676,"ancestor_record_offset":36676,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::shader_defines::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":489,"record_offset":37044,"parent_record_offset":36676,"ancestor_record_offset":36676,"inlinee":27507,"source_line":3,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$70::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":489,"record_offset":36904,"parent_record_offset":36872,"ancestor_record_offset":36676,"inlinee":4604,"source_line":362,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":489,"record_offset":36740,"parent_record_offset":36712,"ancestor_record_offset":36676,"inlinee":4578,"source_line":16,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":37272,"parent_record_offset":37248,"ancestor_record_offset":36676,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":489,"record_offset":37212,"parent_record_offset":37188,"ancestor_record_offset":36676,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":489,"record_offset":37188,"parent_record_offset":37164,"ancestor_record_offset":36676,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":489,"record_offset":37164,"parent_record_offset":37128,"ancestor_record_offset":36676,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":489,"record_offset":36872,"parent_record_offset":36844,"ancestor_record_offset":36676,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":36768,"parent_record_offset":36740,"ancestor_record_offset":36676,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":37128,"parent_record_offset":37100,"ancestor_record_offset":36676,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":489,"record_offset":36796,"parent_record_offset":36768,"ancestor_record_offset":36676,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":489,"record_offset":37248,"parent_record_offset":37128,"ancestor_record_offset":36676,"inlinee":24537,"source_line":35,"archive":"_pdb/A/modules/00489.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6BF3
// 首条位置: {"module":492,"inlinee":27635}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":492,"record_offset":16640,"parent_record_offset":16592,"ancestor_record_offset":16592,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":17232,"parent_record_offset":16592,"ancestor_record_offset":16592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17012,"parent_record_offset":16988,"ancestor_record_offset":16592,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17036,"parent_record_offset":17012,"ancestor_record_offset":16592,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17384,"parent_record_offset":17360,"ancestor_record_offset":16592,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":16988,"parent_record_offset":16944,"ancestor_record_offset":16592,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":492,"record_offset":17360,"parent_record_offset":17304,"ancestor_record_offset":16592,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17332,"parent_record_offset":17304,"ancestor_record_offset":16592,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":16944,"parent_record_offset":16916,"ancestor_record_offset":16592,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":16916,"parent_record_offset":16888,"ancestor_record_offset":16592,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17104,"parent_record_offset":16916,"ancestor_record_offset":16592,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":492,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":16592,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16852,"parent_record_offset":16824,"ancestor_record_offset":16592,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16824,"parent_record_offset":16796,"ancestor_record_offset":16592,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16768,"parent_record_offset":16724,"ancestor_record_offset":16592,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16796,"parent_record_offset":16768,"ancestor_record_offset":16592,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":492,"record_offset":16724,"parent_record_offset":16696,"ancestor_record_offset":16592,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":492,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":16592,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":492,"record_offset":16668,"parent_record_offset":16640,"ancestor_record_offset":16592,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":492,"record_offset":16888,"parent_record_offset":16852,"ancestor_record_offset":16592,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":492,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":16592,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":492,"record_offset":17144,"parent_record_offset":16852,"ancestor_record_offset":16592,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":17256,"parent_record_offset":17232,"ancestor_record_offset":16592,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":492,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":16592,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00492.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::ComputePipelineDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1F01
// 首条位置: {"module":56,"inlinee":7937}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":56,"record_offset":9240,"parent_record_offset":9196,"ancestor_record_offset":9196,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":62736,"parent_record_offset":62684,"ancestor_record_offset":62684,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":56,"record_offset":9728,"parent_record_offset":9196,"ancestor_record_offset":9196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":9196,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9544,"parent_record_offset":9520,"ancestor_record_offset":9196,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9880,"parent_record_offset":9856,"ancestor_record_offset":9196,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9496,"parent_record_offset":9468,"ancestor_record_offset":9196,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":56,"record_offset":9856,"parent_record_offset":9800,"ancestor_record_offset":9196,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9828,"parent_record_offset":9800,"ancestor_record_offset":9196,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9468,"parent_record_offset":9440,"ancestor_record_offset":9196,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":9196,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9612,"parent_record_offset":9440,"ancestor_record_offset":9196,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":56,"record_offset":9800,"parent_record_offset":9776,"ancestor_record_offset":9196,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":9380,"parent_record_offset":9352,"ancestor_record_offset":9196,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":9352,"parent_record_offset":9324,"ancestor_record_offset":9196,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":9296,"parent_record_offset":9268,"ancestor_record_offset":9196,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":9196,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":56,"record_offset":9268,"parent_record_offset":9240,"ancestor_record_offset":9196,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":62792,"parent_record_offset":62764,"ancestor_record_offset":62684,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":62764,"parent_record_offset":62736,"ancestor_record_offset":62684,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":56,"record_offset":9412,"parent_record_offset":9380,"ancestor_record_offset":9196,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":56,"record_offset":9672,"parent_record_offset":9648,"ancestor_record_offset":9196,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":56,"record_offset":9648,"parent_record_offset":9380,"ancestor_record_offset":9196,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":56,"record_offset":9752,"parent_record_offset":9728,"ancestor_record_offset":9196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":56,"record_offset":9568,"parent_record_offset":9544,"ancestor_record_offset":9196,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00056.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C69
// 首条位置: {"module":400,"inlinee":11369}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":400,"record_offset":64000,"parent_record_offset":63956,"ancestor_record_offset":63956,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":64624,"parent_record_offset":63956,"ancestor_record_offset":63956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":64396,"parent_record_offset":64372,"ancestor_record_offset":63956,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":64420,"parent_record_offset":64396,"ancestor_record_offset":63956,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":64744,"parent_record_offset":64720,"ancestor_record_offset":63956,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":64372,"parent_record_offset":64344,"ancestor_record_offset":63956,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":400,"record_offset":64720,"parent_record_offset":64696,"ancestor_record_offset":63956,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":64344,"parent_record_offset":64316,"ancestor_record_offset":63956,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":64316,"parent_record_offset":64288,"ancestor_record_offset":63956,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":64492,"parent_record_offset":64316,"ancestor_record_offset":63956,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":400,"record_offset":64696,"parent_record_offset":64672,"ancestor_record_offset":63956,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":64252,"parent_record_offset":64224,"ancestor_record_offset":63956,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":64224,"parent_record_offset":64196,"ancestor_record_offset":63956,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":64168,"parent_record_offset":64140,"ancestor_record_offset":63956,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":64196,"parent_record_offset":64168,"ancestor_record_offset":63956,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":400,"record_offset":64140,"parent_record_offset":64112,"ancestor_record_offset":63956,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":400,"record_offset":64112,"parent_record_offset":64084,"ancestor_record_offset":63956,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":400,"record_offset":64084,"parent_record_offset":64056,"ancestor_record_offset":63956,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":400,"record_offset":64288,"parent_record_offset":64252,"ancestor_record_offset":63956,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":400,"record_offset":64056,"parent_record_offset":64028,"ancestor_record_offset":63956,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":400,"record_offset":64552,"parent_record_offset":64528,"ancestor_record_offset":63956,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":400,"record_offset":64028,"parent_record_offset":64000,"ancestor_record_offset":63956,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":400,"record_offset":64528,"parent_record_offset":64252,"ancestor_record_offset":63956,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":64648,"parent_record_offset":64624,"ancestor_record_offset":63956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":400,"record_offset":64448,"parent_record_offset":64420,"ancestor_record_offset":63956,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00400.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":145,"record_offset":232,"rva":2586080,"va":5371295200}
// 编译实例 RVAs: 0x2775e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140387ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1115474","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e852051100","size":5,"soft_fail":false,"target_va":5372411040,"va":5371295561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1454423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e857311600","size":5,"soft_fail":false,"target_va":5372749696,"va":5371295268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1454319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e8ef301600","size":5,"soft_fail":false,"target_va":5372749712,"va":5371295388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32297991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e807d4ec01","size":5,"soft_fail":false,"target_va":5403593360,"va":5371295364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31682639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e84f70e301","size":5,"soft_fail":false,"target_va":5402978144,"va":5371295500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6231634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371295200,"raw_hex":"e852165f00","size":5,"soft_fail":false,"target_va":5377527120,"va":5371295481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":145,"record_offset":396,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":1660,"parent_record_offset":232,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":844,"parent_record_offset":820,"ancestor_record_offset":232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":868,"parent_record_offset":844,"ancestor_record_offset":232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":820,"parent_record_offset":784,"ancestor_record_offset":232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":1812,"parent_record_offset":1748,"ancestor_record_offset":232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":1780,"parent_record_offset":1748,"ancestor_record_offset":232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":936,"parent_record_offset":756,"ancestor_record_offset":232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":1748,"parent_record_offset":1716,"ancestor_record_offset":232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":692,"parent_record_offset":660,"ancestor_record_offset":232,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":660,"parent_record_offset":628,"ancestor_record_offset":232,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":596,"parent_record_offset":564,"ancestor_record_offset":232,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":628,"parent_record_offset":596,"ancestor_record_offset":232,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":564,"parent_record_offset":532,"ancestor_record_offset":232,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":145,"record_offset":532,"parent_record_offset":500,"ancestor_record_offset":232,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":145,"record_offset":500,"parent_record_offset":468,"ancestor_record_offset":232,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":728,"parent_record_offset":692,"ancestor_record_offset":232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":145,"record_offset":468,"parent_record_offset":440,"ancestor_record_offset":232,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":145,"record_offset":1000,"parent_record_offset":976,"ancestor_record_offset":232,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":145,"record_offset":440,"parent_record_offset":396,"ancestor_record_offset":232,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":145,"record_offset":976,"parent_record_offset":692,"ancestor_record_offset":232,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":1688,"parent_record_offset":1660,"ancestor_record_offset":232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":892,"parent_record_offset":868,"ancestor_record_offset":232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":145,"record_offset":1068,"parent_record_offset":396,"ancestor_record_offset":232,"inlinee":11342,"source_line":64,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2C4E
// 首条位置: {"module":145,"inlinee":11342}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":145,"record_offset":1096,"parent_record_offset":1068,"ancestor_record_offset":1068,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":1068,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":1068,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":145,"record_offset":1412,"parent_record_offset":1384,"ancestor_record_offset":1068,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":1384,"parent_record_offset":1356,"ancestor_record_offset":1068,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":1356,"parent_record_offset":1328,"ancestor_record_offset":1068,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":145,"record_offset":1528,"parent_record_offset":1356,"ancestor_record_offset":1068,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":1068,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":1264,"parent_record_offset":1236,"ancestor_record_offset":1068,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":1068,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":1068,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":145,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":1068,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":145,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":1068,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":145,"record_offset":1124,"parent_record_offset":1096,"ancestor_record_offset":1068,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":145,"record_offset":1328,"parent_record_offset":1292,"ancestor_record_offset":1068,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":145,"record_offset":1588,"parent_record_offset":1564,"ancestor_record_offset":1068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":145,"record_offset":1564,"parent_record_offset":1292,"ancestor_record_offset":1068,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00145.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":145,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":1068,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00145.jsonl.gz"}
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
// 完整结构及原始类型记录: pipeline_desc.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: pipeline_desc.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: pipeline_desc.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 8; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::shader_refactoring_state::ShaderRefactoringState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>)
// 候选参数类型: ["bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::ClearCanvasCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>,bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>", "bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>", "bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::Without<country_core::components::KeepOnLevelUnload> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::new
// 信息 ID: d1a09088908f899495978e4d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<alloc::string::String> >; module=56, inline_record=9196; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >; module=136, inline_record=15412; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >; module=390, inline_record=11872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline; module=400, inline_record=62684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline; module=400, inline_record=66432; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline; module=400, inline_record=70188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline; module=400, inline_record=73748; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline; module=400, inline_record=77308; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
// 信息 ID: 4cc079a7efd15f0ae14511a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline; module=400, inline_record=63956; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline; module=400, inline_record=67704; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline; module=400, inline_record=71460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline; module=400, inline_record=75020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$19::as_compute_pipeline; module=400, inline_record=78580; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$2::as_compute_pipeline; module=451, inline_record=46284; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$6::as_compute_pipeline; module=451, inline_record=49860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::impl$10::as_compute_pipeline; module=451, inline_record=53440; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >
// 信息 ID: 15342536ccd438bb3d691a3d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<alloc::string::String> > @ 0x7b500 -> 0x2775e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x26763f -> 0x2775e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x51327f -> 0x2775e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
// 信息 ID: 0d536a87cb0c26420a304d26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_shader_into_library<alloc::string::String>; module=492, inline_record=16592; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_shader_into_library<ref$<str$> >; module=492, inline_record=17756; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$14::clone
// 信息 ID: a82bfac8e2e2a83093567b7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_variants::get_raster_shader_variants; module=1454, inline_record=944; 不是独立调用边。
// [PDB 内联宿主] shader_variants::get_raster_shader_variants; module=1454, inline_record=2604; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$16::eq
// 信息 ID: 5a7f847e243dd8e5b636a5a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster; module=489, inline_record=36676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
// 信息 ID: 15265aae4188c555e789f256
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::pipeline_desc::impl$1::with_entry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >; module=145, inline_record=1068; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >; module=1331, inline_record=1084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$8::read_from
// 信息 ID: 0fcb4af32974c08c47835bd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=6912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
