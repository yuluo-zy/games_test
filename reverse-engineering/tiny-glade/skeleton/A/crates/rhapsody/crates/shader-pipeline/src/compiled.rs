// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/crates/shader-pipeline/src/compiled.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 24b937849d7d640534dee1bb8e799e2bc3d48e1eb69626ad9fbd2ba73dd969cc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compiled.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 1331, 2833, 3942
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$2::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF29D
// 首条位置: {"module":1331,"inlinee":62109}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":8972,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":7840,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":7092,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6880,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":7304,"parent_record_offset":6804,"ancestor_record_offset":6804,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":7580,"parent_record_offset":7552,"ancestor_record_offset":6804,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":7608,"parent_record_offset":7580,"ancestor_record_offset":6804,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":6804,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":7552,"parent_record_offset":7504,"ancestor_record_offset":6804,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":8084,"parent_record_offset":8028,"ancestor_record_offset":6804,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":6804,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":7504,"parent_record_offset":7468,"ancestor_record_offset":6804,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":7468,"parent_record_offset":7432,"ancestor_record_offset":6804,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":7692,"parent_record_offset":7468,"ancestor_record_offset":6804,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":6804,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1331,"record_offset":7944,"parent_record_offset":7908,"ancestor_record_offset":6804,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":7396,"parent_record_offset":7304,"ancestor_record_offset":6804,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":7432,"parent_record_offset":7396,"ancestor_record_offset":6804,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":7044,"parent_record_offset":7016,"ancestor_record_offset":6804,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":8488,"parent_record_offset":8380,"ancestor_record_offset":6804,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1331,"record_offset":7876,"parent_record_offset":7840,"ancestor_record_offset":6804,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":7260,"parent_record_offset":7192,"ancestor_record_offset":6804,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":7980,"parent_record_offset":7944,"ancestor_record_offset":6804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":7644,"parent_record_offset":7608,"ancestor_record_offset":6804,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$8::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1331,"record_offset":6912,"parent_record_offset":6880,"ancestor_record_offset":6804,"inlinee":62111,"source_line":3,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":6804,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":6984,"parent_record_offset":6944,"ancestor_record_offset":6804,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":7016,"parent_record_offset":6944,"ancestor_record_offset":6804,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6944,"parent_record_offset":6912,"ancestor_record_offset":6804,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$4::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF29B
// 首条位置: {"module":1331,"inlinee":62107}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":18560,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":18532,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":9248,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":6024,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":11724,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6164,"parent_record_offset":5988,"ancestor_record_offset":5988,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":6588,"parent_record_offset":6564,"ancestor_record_offset":5988,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":5988,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":5988,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":6564,"parent_record_offset":6472,"ancestor_record_offset":5988,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":8084,"parent_record_offset":8028,"ancestor_record_offset":5988,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":5988,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":6472,"parent_record_offset":6444,"ancestor_record_offset":5988,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":6444,"parent_record_offset":6412,"ancestor_record_offset":5988,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":6684,"parent_record_offset":6444,"ancestor_record_offset":5988,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":8028,"parent_record_offset":8004,"ancestor_record_offset":5988,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":1331,"record_offset":7944,"parent_record_offset":7908,"ancestor_record_offset":5988,"inlinee":13019,"source_line":576,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":6380,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":6412,"parent_record_offset":6380,"ancestor_record_offset":5988,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$24::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1331,"record_offset":18584,"parent_record_offset":18560,"ancestor_record_offset":5988,"inlinee":4700,"source_line":3200,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":6508,"parent_record_offset":6472,"ancestor_record_offset":5988,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1331,"record_offset":6532,"parent_record_offset":6508,"ancestor_record_offset":5988,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1331,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":5988,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1331,"record_offset":13032,"parent_record_offset":13004,"ancestor_record_offset":5988,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1331,"record_offset":13004,"parent_record_offset":12980,"ancestor_record_offset":5988,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":14668,"parent_record_offset":14532,"ancestor_record_offset":5988,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":6320,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1331,"record_offset":19420,"parent_record_offset":19388,"ancestor_record_offset":5988,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1331,"record_offset":7876,"parent_record_offset":7840,"ancestor_record_offset":5988,"inlinee":4961,"source_line":250,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1331,"record_offset":6124,"parent_record_offset":6072,"ancestor_record_offset":5988,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=46
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":7980,"parent_record_offset":7944,"ancestor_record_offset":5988,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1331,"record_offset":9212,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":5988,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":9180,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":7840,"parent_record_offset":6804,"ancestor_record_offset":5988,"inlinee":4730,"source_line":1319,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":11276,"parent_record_offset":11032,"ancestor_record_offset":5988,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":13060,"parent_record_offset":13004,"ancestor_record_offset":5988,"inlinee":24116,"source_line":1934,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":10996,"parent_record_offset":10068,"ancestor_record_offset":5988,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":11600,"parent_record_offset":10068,"ancestor_record_offset":5988,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=unresolved_namespace; count=5
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1331,"record_offset":15456,"parent_record_offset":15432,"ancestor_record_offset":5988,"inlinee":5572,"source_line":3669,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::impl$2::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1331,"record_offset":6804,"parent_record_offset":6724,"ancestor_record_offset":5988,"inlinee":62109,"source_line":11,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$8::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":1331,"record_offset":6912,"parent_record_offset":6880,"ancestor_record_offset":5988,"inlinee":62111,"source_line":3,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":13584,"parent_record_offset":13528,"ancestor_record_offset":5988,"inlinee":62123,"source_line":31,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":15104,"parent_record_offset":15076,"ancestor_record_offset":5988,"inlinee":61810,"source_line":69,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":14840,"parent_record_offset":14816,"ancestor_record_offset":5988,"inlinee":61808,"source_line":75,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":12756,"parent_record_offset":12732,"ancestor_record_offset":5988,"inlinee":62117,"source_line":78,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$89::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":11932,"parent_record_offset":11876,"ancestor_record_offset":5988,"inlinee":62115,"source_line":83,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":11792,"parent_record_offset":11724,"ancestor_record_offset":5988,"inlinee":62113,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":13472,"parent_record_offset":13416,"ancestor_record_offset":5988,"inlinee":62121,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":17232,"parent_record_offset":17204,"ancestor_record_offset":5988,"inlinee":62127,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1331,"record_offset":17260,"parent_record_offset":17232,"ancestor_record_offset":5988,"inlinee":62129,"source_line":88,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/error.rs
//     evidence={"module":1331,"record_offset":11324,"parent_record_offset":11300,"ancestor_record_offset":5988,"inlinee":27867,"source_line":137,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":10252,"parent_record_offset":10196,"ancestor_record_offset":5988,"inlinee":61830,"source_line":195,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":7092,"parent_record_offset":6804,"ancestor_record_offset":5988,"inlinee":61820,"source_line":159,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/private.rs
//     evidence={"module":1331,"record_offset":7908,"parent_record_offset":7876,"ancestor_record_offset":5988,"inlinee":61822,"source_line":39,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":6352,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":61800,"source_line":150,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":14572,"parent_record_offset":14532,"ancestor_record_offset":5988,"inlinee":62126,"source_line":123,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable.rs
//     evidence={"module":1331,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5988,"inlinee":61801,"source_line":56,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":14280,"parent_record_offset":14248,"ancestor_record_offset":5988,"inlinee":62125,"source_line":54,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":13672,"parent_record_offset":13644,"ancestor_record_offset":5988,"inlinee":61803,"source_line":116,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$7::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":10068,"parent_record_offset":10016,"ancestor_record_offset":5988,"inlinee":61867,"source_line":187,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$9::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":1331,"record_offset":10196,"parent_record_offset":10148,"ancestor_record_offset":5988,"inlinee":61829,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12980,"parent_record_offset":12956,"ancestor_record_offset":5988,"inlinee":62119,"source_line":630,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":12956,"parent_record_offset":12756,"ancestor_record_offset":5988,"inlinee":62118,"source_line":652,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5988,"inlinee":61804,"source_line":216,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":13292,"parent_record_offset":11792,"ancestor_record_offset":5988,"inlinee":61845,"source_line":108,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":6724,"parent_record_offset":6164,"ancestor_record_offset":5988,"inlinee":61791,"source_line":504,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/speedy-0.8.6/src/reader.rs
//     evidence={"module":1331,"record_offset":7304,"parent_record_offset":6804,"ancestor_record_offset":5988,"inlinee":61821,"source_line":514,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1331,"record_offset":1904,"rva":15302512,"va":5384011632}
// 编译实例 RVAs: 0xe97f70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11697676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8f4814dff","size":5,"soft_fail":false,"target_va":5372314208,"va":5384011879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11262480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8f02554ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384012187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8840632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8481a79ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5384011683,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141822b40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10005408","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8a0ab9800","size":5,"soft_fail":false,"target_va":5394017088,"va":5384011675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19079795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e873222301","size":5,"soft_fail":false,"target_va":5403091664,"va":5384011864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19581198","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e80ec92a01","size":5,"soft_fail":false,"target_va":5403593360,"va":5384012157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6481969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8cf179dff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384011852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1434","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e89a050000","size":5,"soft_fail":false,"target_va":5384013360,"va":5384011921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6606634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384011632,"raw_hex":"e8d6309bff","size":5,"soft_fail":false,"target_va":5377405072,"va":5384011701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":2816,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":2344,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":2508,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1331,"record_offset":2120,"parent_record_offset":1904,"ancestor_record_offset":1904,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":1904,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":2936,"parent_record_offset":2872,"ancestor_record_offset":1904,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2416,"parent_record_offset":2384,"ancestor_record_offset":1904,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":1904,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":2904,"parent_record_offset":2872,"ancestor_record_offset":1904,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":2872,"parent_record_offset":2844,"ancestor_record_offset":1904,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1331,"record_offset":2144,"parent_record_offset":2120,"ancestor_record_offset":1904,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1331,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":1904,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":2696,"parent_record_offset":2576,"ancestor_record_offset":1904,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1331,"record_offset":2384,"parent_record_offset":2344,"ancestor_record_offset":1904,"inlinee":62099,"source_line":30,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1331,"record_offset":2576,"parent_record_offset":2536,"ancestor_record_offset":1904,"inlinee":62100,"source_line":38,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":2240,"parent_record_offset":2216,"ancestor_record_offset":1904,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":2288,"parent_record_offset":2264,"ancestor_record_offset":1904,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":2264,"parent_record_offset":2240,"ancestor_record_offset":1904,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":1904,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":1904,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":2168,"parent_record_offset":2144,"ancestor_record_offset":1904,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1331,"record_offset":3780,"rva":15303376,"va":5384012496}
// 编译实例 RVAs: 0xe982d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=4
//     target_aliases=89 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11698540","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8947e4dff","size":5,"soft_fail":false,"target_va":5372314208,"va":5384012743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11263344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8902254ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384013051,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=1
//     target_aliases=28 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8841496","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8e81679ff","size":5,"soft_fail":false,"target_va":5375171056,"va":5384012547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141822ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10004896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8a0a99800","size":5,"soft_fail":false,"target_va":5394017440,"va":5384012539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca2d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19078931","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8131f2301","size":5,"soft_fail":false,"target_va":5403091664,"va":5384012728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19580334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8aec52a01","size":5,"soft_fail":false,"target_va":5403593360,"va":5384013021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6482833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e86f149dff","size":5,"soft_fail":false,"target_va":5377529888,"va":5384012716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e83a020000","size":5,"soft_fail":false,"target_va":5384013360,"va":5384012785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6607498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384012496,"raw_hex":"e8762d9bff","size":5,"soft_fail":false,"target_va":5377405072,"va":5384012565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":4692,"parent_record_offset":3780,"ancestor_record_offset":3780,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":4220,"parent_record_offset":3780,"ancestor_record_offset":3780,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1331,"record_offset":4384,"parent_record_offset":3780,"ancestor_record_offset":3780,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":1331,"record_offset":3996,"parent_record_offset":3780,"ancestor_record_offset":3780,"inlinee":7565,"source_line":259,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":4836,"parent_record_offset":4812,"ancestor_record_offset":3780,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1331,"record_offset":4812,"parent_record_offset":4748,"ancestor_record_offset":3780,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":4292,"parent_record_offset":4260,"ancestor_record_offset":3780,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3780,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":4780,"parent_record_offset":4748,"ancestor_record_offset":3780,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1331,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":3780,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":1331,"record_offset":4020,"parent_record_offset":3996,"ancestor_record_offset":3780,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1331,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3780,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":4572,"parent_record_offset":4452,"ancestor_record_offset":3780,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1331,"record_offset":4260,"parent_record_offset":4220,"ancestor_record_offset":3780,"inlinee":62099,"source_line":30,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1331,"record_offset":4452,"parent_record_offset":4412,"ancestor_record_offset":3780,"inlinee":62100,"source_line":38,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":3780,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3780,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":1331,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":3780,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":3780,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":3780,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":1331,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":3780,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF293
// 首条位置: {"module":1331,"inlinee":62099}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2416,"parent_record_offset":2384,"ancestor_record_offset":2384,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2464,"parent_record_offset":2440,"ancestor_record_offset":2384,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1331,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":2384,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF294
// 首条位置: {"module":1331,"inlinee":62100}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2612,"parent_record_offset":2576,"ancestor_record_offset":2576,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":2696,"parent_record_offset":2576,"ancestor_record_offset":2576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":1331,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2576,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1331,"record_offset":2636,"parent_record_offset":2612,"ancestor_record_offset":2576,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1331,"record_offset":2720,"parent_record_offset":2696,"ancestor_record_offset":2576,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01331.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$0::compiled_file_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2833,"record_offset":2316,"rva":25307968,"va":5394017088}
// 编译实例 RVAs: 0x1822b40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21267615","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017088,"raw_hex":"e8617bbbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394017322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c95c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4665607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017088,"raw_hex":"e807314700","size":5,"soft_fail":false,"target_va":5398682768,"va":5394017156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16487415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017088,"raw_hex":"e8096c04ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5394017298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<char>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017088,"raw_hex":"e86ffdffff","size":5,"soft_fail":false,"target_va":5394016544,"va":5394017196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2833,"record_offset":2668,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2833,"record_offset":2960,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2833,"record_offset":2752,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::compiled::calculate_shader_defines_hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":2833,"record_offset":2468,"parent_record_offset":2316,"ancestor_record_offset":2316,"inlinee":90431,"source_line":76,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2833,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":2316,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2833,"record_offset":2888,"parent_record_offset":2832,"ancestor_record_offset":2316,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2833,"record_offset":2716,"parent_record_offset":2692,"ancestor_record_offset":2316,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2833,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":2316,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2833,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2316,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2833,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":2316,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2833,"record_offset":2692,"parent_record_offset":2668,"ancestor_record_offset":2316,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2833,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2316,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2316,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":2572,"parent_record_offset":2524,"ancestor_record_offset":2316,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":2604,"parent_record_offset":2524,"ancestor_record_offset":2316,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":2316,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2833,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2316,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2833,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2316,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$1::compiled_file_path
// 记录形式: compiled_procedure; instances=1
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2833,"record_offset":3256,"rva":25308320,"va":5394017440}
// 编译实例 RVAs: 0x1822ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21267955","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017440,"raw_hex":"e80d7abbfe","size":5,"soft_fail":false,"target_va":5372749712,"va":5394017662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141c95c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4665255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017440,"raw_hex":"e8a72f4700","size":5,"soft_fail":false,"target_va":5398682768,"va":5394017508,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-16487755","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017440,"raw_hex":"e8b56a04ff","size":5,"soft_fail":false,"target_va":5377529888,"va":5394017638,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::str::impl$5::replace<char>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-1009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5394017440,"raw_hex":"e80ffcffff","size":5,"soft_fail":false,"target_va":5394016544,"va":5394017548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2833,"record_offset":3608,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":2833,"record_offset":3900,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2833,"record_offset":3692,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::compiled::calculate_shader_defines_hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":2833,"record_offset":3408,"parent_record_offset":3256,"ancestor_record_offset":3256,"inlinee":90431,"source_line":76,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2833,"record_offset":3852,"parent_record_offset":3828,"ancestor_record_offset":3256,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":2833,"record_offset":3828,"parent_record_offset":3772,"ancestor_record_offset":3256,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":2833,"record_offset":3656,"parent_record_offset":3632,"ancestor_record_offset":3256,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2833,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":3256,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":2833,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3256,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2833,"record_offset":3436,"parent_record_offset":3408,"ancestor_record_offset":3256,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":2833,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":3256,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":2833,"record_offset":3716,"parent_record_offset":3692,"ancestor_record_offset":3256,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":3568,"parent_record_offset":3544,"ancestor_record_offset":3256,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":3512,"parent_record_offset":3464,"ancestor_record_offset":3256,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":3544,"parent_record_offset":3464,"ancestor_record_offset":3256,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":3464,"parent_record_offset":3408,"ancestor_record_offset":3256,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$37::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2833,"record_offset":3948,"parent_record_offset":3924,"ancestor_record_offset":3256,"inlinee":11358,"source_line":1711,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$39::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":2833,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":3256,"inlinee":11356,"source_line":1733,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::calculate_shader_defines_hash
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1613F
// 首条位置: {"module":2833,"inlinee":90431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":2833,"record_offset":2496,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":2524,"parent_record_offset":2468,"ancestor_record_offset":2468,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2468,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":2833,"record_offset":2572,"parent_record_offset":2524,"ancestor_record_offset":2468,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":2833,"record_offset":2604,"parent_record_offset":2524,"ancestor_record_offset":2468,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/02833.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::calculate_hash<tuple$<alloc::string::String,alloc::string::String> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3942,"record_offset":1492,"rva":29973648,"va":5398682768}
// 编译实例 RVAs: 0x1c95c90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dde70
//     owner=unresolved_namespace; count=4
//     target_aliases=30 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29064848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5398682768,"raw_hex":"e8708144fe","size":5,"soft_fail":false,"target_va":5369618032,"va":5398682875,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3942,"record_offset":1708,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8466,"source_line":887,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3942,"record_offset":1680,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8464,"source_line":109,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3942,"record_offset":2076,"parent_record_offset":1492,"ancestor_record_offset":1492,"inlinee":8437,"source_line":142,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$23::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3942,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1492,"inlinee":8455,"source_line":2382,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3942,"record_offset":1852,"parent_record_offset":1828,"ancestor_record_offset":1492,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":3942,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":1492,"inlinee":8449,"source_line":858,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":1492,"inlinee":8438,"source_line":246,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":1492,"inlinee":8451,"source_line":241,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":1492,"inlinee":8440,"source_line":315,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":1828,"parent_record_offset":1804,"ancestor_record_offset":1492,"inlinee":8452,"source_line":307,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":2172,"parent_record_offset":2124,"ancestor_record_offset":1492,"inlinee":8442,"source_line":365,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":3942,"record_offset":2564,"parent_record_offset":2124,"ancestor_record_offset":1492,"inlinee":8443,"source_line":370,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3942,"record_offset":2272,"parent_record_offset":2172,"ancestor_record_offset":1492,"inlinee":7983,"source_line":234,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3942,"record_offset":2240,"parent_record_offset":2172,"ancestor_record_offset":1492,"inlinee":6614,"source_line":1639,"archive":"_pdb/A/modules/03942.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":3942,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":1492,"inlinee":8450,"source_line":137,"archive":"_pdb/A/modules/03942.jsonl.gz"}
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
// 完整结构及原始类型记录: compiled.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: compiled.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: compiled.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
// 信息 ID: 712ff3e4a672cb51104d5660
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>; module=1331, inline_record=2384; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>; module=1331, inline_record=4260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
// 信息 ID: c0fc4ce8a9ca7046bb9bfaeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>; module=1331, inline_record=2576; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>; module=1331, inline_record=4452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>
// 信息 ID: f8de88fb559941bcf3a168ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled -> ["shader_pipeline::pipeline_desc::ComputePipelineDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::ComputeShaderProgram::new @ 0x6500c2 -> 0xe97f70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>
// 信息 ID: 3ea7c63381733129d883fcd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled -> ["shader_pipeline::pipeline_desc::RasterPipelineDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x650534 -> 0xe982d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x65069a -> 0xe982d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::calculate_hash<tuple$<alloc::string::String,alloc::string::String> >
// 信息 ID: 9632e7ca6f3f4bc17f665e81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::compiled::calculate_hash -> ["tuple$<alloc::string::String,alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::impl$0::compiled_file_path @ 0x1822b84 -> 0x1c95c90 (direct_call); 同地址别名未裁定。
// [静态直接引用] shader_pipeline::compiled::impl$1::compiled_file_path @ 0x1822ce4 -> 0x1c95c90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::calculate_shader_defines_hash
// 信息 ID: e98a377ede3c9a72889ed1d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::impl$0::compiled_file_path; module=2833, inline_record=2468; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::impl$1::compiled_file_path; module=2833, inline_record=3408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$0::compiled_file_path
// 信息 ID: 4b710bfcf31674235dc3cff0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc> @ 0xe97f9b -> 0x1822b40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$1::compiled_file_path
// 信息 ID: 338e15c022590fe0587e9c3a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc> @ 0xe982fb -> 0x1822ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$2::read_from
// 信息 ID: a2c0188aa7df4a64f242631e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=6804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$4::read_from
// 信息 ID: 1bceba0475b38ae9623cf885
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=1331, inline_record=5988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
