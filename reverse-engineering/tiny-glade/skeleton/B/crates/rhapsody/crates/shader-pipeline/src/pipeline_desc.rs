// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 70d36740a680b8ee09a5a5672378dce969a7531578d9630ef3a6f0734bc00fe4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: pipeline_desc.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 15, 222, 252, 487, 495, 509, 534, 544, 549, 587, 1634, 1692
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$8::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 3
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x772B
// 首条位置: {"module":534,"inlinee":30507}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":12716,"parent_record_offset":12672,"ancestor_record_offset":12672,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":12672,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":12760,"parent_record_offset":12716,"ancestor_record_offset":12672,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":12792,"parent_record_offset":12716,"ancestor_record_offset":12672,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$14::clone
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12C53
// 首条位置: {"module":1692,"inlinee":76883}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1692,"record_offset":1036,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1692,"record_offset":1104,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::shader_defines::impl$1::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":1692,"record_offset":1076,"parent_record_offset":988,"ancestor_record_offset":988,"inlinee":76885,"source_line":3,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1280,"parent_record_offset":1256,"ancestor_record_offset":988,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1692,"record_offset":1256,"parent_record_offset":1200,"ancestor_record_offset":988,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1228,"parent_record_offset":1200,"ancestor_record_offset":988,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1200,"parent_record_offset":1176,"ancestor_record_offset":988,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1692,"record_offset":1176,"parent_record_offset":1152,"ancestor_record_offset":988,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1692,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":988,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01692.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$16::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 16
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7143
// 首条位置: {"module":509,"inlinee":28995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":509,"record_offset":52532,"parent_record_offset":52500,"ancestor_record_offset":52500,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":509,"record_offset":52664,"parent_record_offset":52500,"ancestor_record_offset":52500,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::shader_defines::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs
//     evidence={"module":509,"record_offset":52832,"parent_record_offset":52500,"ancestor_record_offset":52500,"inlinee":28997,"source_line":3,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$78::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":509,"record_offset":52696,"parent_record_offset":52664,"ancestor_record_offset":52500,"inlinee":5606,"source_line":359,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\partial_eq.rs
//     evidence={"module":509,"record_offset":52560,"parent_record_offset":52532,"ancestor_record_offset":52500,"inlinee":5608,"source_line":15,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":509,"record_offset":53060,"parent_record_offset":53036,"ancestor_record_offset":52500,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":509,"record_offset":53000,"parent_record_offset":52976,"ancestor_record_offset":52500,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":509,"record_offset":52976,"parent_record_offset":52952,"ancestor_record_offset":52500,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":509,"record_offset":52952,"parent_record_offset":52916,"ancestor_record_offset":52500,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":52588,"parent_record_offset":52560,"ancestor_record_offset":52500,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":52916,"parent_record_offset":52888,"ancestor_record_offset":52500,"inlinee":5713,"source_line":57,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":509,"record_offset":52616,"parent_record_offset":52588,"ancestor_record_offset":52500,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$10::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\tuple.rs
//     evidence={"module":509,"record_offset":53036,"parent_record_offset":52916,"ancestor_record_offset":52500,"inlinee":4992,"source_line":34,"archive":"_pdb/B/modules/00509.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7773
// 首条位置: {"module":587,"inlinee":30579}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":587,"record_offset":123432,"parent_record_offset":123376,"ancestor_record_offset":123376,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":124184,"parent_record_offset":123376,"ancestor_record_offset":123376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124020,"parent_record_offset":123996,"ancestor_record_offset":123376,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124044,"parent_record_offset":124020,"ancestor_record_offset":123376,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124360,"parent_record_offset":124336,"ancestor_record_offset":123376,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":123996,"parent_record_offset":123812,"ancestor_record_offset":123376,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":587,"record_offset":124336,"parent_record_offset":124280,"ancestor_record_offset":123376,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123744,"parent_record_offset":123712,"ancestor_record_offset":123376,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124308,"parent_record_offset":124280,"ancestor_record_offset":123376,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124280,"parent_record_offset":124256,"ancestor_record_offset":123376,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123812,"parent_record_offset":123776,"ancestor_record_offset":123376,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123776,"parent_record_offset":123744,"ancestor_record_offset":123376,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":124256,"parent_record_offset":124232,"ancestor_record_offset":123376,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":587,"record_offset":123856,"parent_record_offset":123812,"ancestor_record_offset":123376,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123672,"parent_record_offset":123640,"ancestor_record_offset":123376,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123640,"parent_record_offset":123608,"ancestor_record_offset":123376,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123576,"parent_record_offset":123528,"ancestor_record_offset":123376,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123608,"parent_record_offset":123576,"ancestor_record_offset":123376,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":587,"record_offset":123528,"parent_record_offset":123496,"ancestor_record_offset":123376,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":587,"record_offset":123496,"parent_record_offset":123464,"ancestor_record_offset":123376,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":587,"record_offset":123464,"parent_record_offset":123432,"ancestor_record_offset":123376,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":587,"record_offset":123712,"parent_record_offset":123672,"ancestor_record_offset":123376,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":123880,"parent_record_offset":123856,"ancestor_record_offset":123376,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":587,"record_offset":123904,"parent_record_offset":123880,"ancestor_record_offset":123376,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":124120,"parent_record_offset":124096,"ancestor_record_offset":123376,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":587,"record_offset":123952,"parent_record_offset":123928,"ancestor_record_offset":123376,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":587,"record_offset":123928,"parent_record_offset":123904,"ancestor_record_offset":123376,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":587,"record_offset":124096,"parent_record_offset":123672,"ancestor_record_offset":123376,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":587,"record_offset":124208,"parent_record_offset":124184,"ancestor_record_offset":123376,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00587.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":534,"record_offset":260,"rva":10369408,"va":5379078528}
// 编译实例 RVAs: 0x9e3980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3769223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379078528,"raw_hex":"e887833900","size":5,"soft_fail":false,"target_va":5382847888,"va":5379078660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3997110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379078528,"raw_hex":"e84a02c3ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5379078593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3997312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379078528,"raw_hex":"e88001c3ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379078811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32768532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379078528,"raw_hex":"e81402f401","size":5,"soft_fail":false,"target_va":5411847168,"va":5379078631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t32462207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379078528,"raw_hex":"e87f55ef01","size":5,"soft_fail":false,"target_va":5411540963,"va":5379078751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":534,"record_offset":432,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":1120,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":260,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":260,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":1296,"parent_record_offset":1272,"ancestor_record_offset":260,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":932,"parent_record_offset":756,"ancestor_record_offset":260,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":1272,"parent_record_offset":1216,"ancestor_record_offset":260,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":700,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":1244,"parent_record_offset":1216,"ancestor_record_offset":260,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":1216,"parent_record_offset":1192,"ancestor_record_offset":260,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":756,"parent_record_offset":728,"ancestor_record_offset":260,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":260,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":260,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":792,"parent_record_offset":756,"ancestor_record_offset":260,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":260,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":260,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":556,"parent_record_offset":516,"ancestor_record_offset":260,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":260,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":534,"record_offset":516,"parent_record_offset":488,"ancestor_record_offset":260,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":534,"record_offset":488,"parent_record_offset":460,"ancestor_record_offset":260,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":460,"parent_record_offset":432,"ancestor_record_offset":260,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":672,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":260,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":840,"parent_record_offset":816,"ancestor_record_offset":260,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":260,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":260,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":864,"parent_record_offset":840,"ancestor_record_offset":260,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":1032,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":1144,"parent_record_offset":1120,"ancestor_record_offset":260,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::ComputePipelineDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=30
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1923
// 首条位置: {"module":15,"inlinee":6435}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":15,"record_offset":20860,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":75696,"parent_record_offset":75644,"ancestor_record_offset":75644,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":21332,"parent_record_offset":20816,"ancestor_record_offset":20816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":20816,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21200,"parent_record_offset":21176,"ancestor_record_offset":20816,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21508,"parent_record_offset":21484,"ancestor_record_offset":20816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":20816,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":15,"record_offset":21484,"parent_record_offset":21428,"ancestor_record_offset":20816,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21064,"parent_record_offset":21036,"ancestor_record_offset":20816,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21456,"parent_record_offset":21428,"ancestor_record_offset":20816,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21428,"parent_record_offset":21404,"ancestor_record_offset":20816,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21124,"parent_record_offset":21092,"ancestor_record_offset":20816,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21092,"parent_record_offset":21064,"ancestor_record_offset":20816,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":15,"record_offset":21404,"parent_record_offset":21380,"ancestor_record_offset":20816,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":21000,"parent_record_offset":20972,"ancestor_record_offset":20816,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20972,"parent_record_offset":20944,"ancestor_record_offset":20816,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20916,"parent_record_offset":20888,"ancestor_record_offset":20816,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20944,"parent_record_offset":20916,"ancestor_record_offset":20816,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":15,"record_offset":20888,"parent_record_offset":20860,"ancestor_record_offset":20816,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":487,"record_offset":75752,"parent_record_offset":75724,"ancestor_record_offset":75644,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":75724,"parent_record_offset":75696,"ancestor_record_offset":75644,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":15,"record_offset":21036,"parent_record_offset":21000,"ancestor_record_offset":20816,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":15,"record_offset":21276,"parent_record_offset":21252,"ancestor_record_offset":20816,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=39
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":15,"record_offset":21252,"parent_record_offset":21000,"ancestor_record_offset":20816,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":15,"record_offset":21356,"parent_record_offset":21332,"ancestor_record_offset":20816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00015.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
// 记录形式: inline_instance, inlinee_declaration; instances=24
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B66
// 首条位置: {"module":487,"inlinee":15206}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":487,"record_offset":76948,"parent_record_offset":76904,"ancestor_record_offset":76904,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":77548,"parent_record_offset":76904,"ancestor_record_offset":76904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":77376,"parent_record_offset":77352,"ancestor_record_offset":76904,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":77400,"parent_record_offset":77376,"ancestor_record_offset":76904,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":77692,"parent_record_offset":77668,"ancestor_record_offset":76904,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":77352,"parent_record_offset":77324,"ancestor_record_offset":76904,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":487,"record_offset":77668,"parent_record_offset":77644,"ancestor_record_offset":76904,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":77264,"parent_record_offset":77236,"ancestor_record_offset":76904,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":77644,"parent_record_offset":77620,"ancestor_record_offset":76904,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":77324,"parent_record_offset":77292,"ancestor_record_offset":76904,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":77292,"parent_record_offset":77264,"ancestor_record_offset":76904,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":487,"record_offset":77620,"parent_record_offset":77596,"ancestor_record_offset":76904,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":77200,"parent_record_offset":77172,"ancestor_record_offset":76904,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":77172,"parent_record_offset":77144,"ancestor_record_offset":76904,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":77116,"parent_record_offset":77088,"ancestor_record_offset":76904,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":77144,"parent_record_offset":77116,"ancestor_record_offset":76904,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":487,"record_offset":77088,"parent_record_offset":77060,"ancestor_record_offset":76904,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":487,"record_offset":77060,"parent_record_offset":77032,"ancestor_record_offset":76904,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":487,"record_offset":77032,"parent_record_offset":77004,"ancestor_record_offset":76904,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":487,"record_offset":77236,"parent_record_offset":77200,"ancestor_record_offset":76904,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":487,"record_offset":77004,"parent_record_offset":76976,"ancestor_record_offset":76904,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":487,"record_offset":77476,"parent_record_offset":77452,"ancestor_record_offset":76904,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":487,"record_offset":76976,"parent_record_offset":76948,"ancestor_record_offset":76904,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":487,"record_offset":77452,"parent_record_offset":77200,"ancestor_record_offset":76904,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":487,"record_offset":77572,"parent_record_offset":77548,"ancestor_record_offset":76904,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00487.jsonl.gz"}
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
// 首条位置: {"module":222,"record_offset":1540,"rva":5477232,"va":5374186352}
// 编译实例 RVAs: 0x539370, 0x9e3ab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fe2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t806368","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186352,"raw_hex":"e8e04d0c00","size":5,"soft_fail":false,"target_va":5374993120,"va":5374186747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t895055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186352,"raw_hex":"e84fa80d00","size":5,"soft_fail":false,"target_va":5375081488,"va":5374186428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t894948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186352,"raw_hex":"e8e4a70d00","size":5,"soft_fail":false,"target_va":5375081504,"va":5374186551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: pipeline_desc.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37660638","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186352,"raw_hex":"e8dea73e02","size":5,"soft_fail":false,"target_va":5411847168,"va":5374186525,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37354286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374186352,"raw_hex":"e82efb3902","size":5,"soft_fail":false,"target_va":5411540963,"va":5374186672,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":222,"record_offset":1704,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":3064,"parent_record_offset":1540,"ancestor_record_offset":1540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2324,"parent_record_offset":2300,"ancestor_record_offset":1540,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2348,"parent_record_offset":2324,"ancestor_record_offset":1540,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":1540,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2300,"parent_record_offset":2124,"ancestor_record_offset":1540,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":3240,"parent_record_offset":3176,"ancestor_record_offset":1540,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2064,"parent_record_offset":2036,"ancestor_record_offset":1540,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":3208,"parent_record_offset":3176,"ancestor_record_offset":1540,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":1540,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2124,"parent_record_offset":2092,"ancestor_record_offset":1540,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":1540,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":1540,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2160,"parent_record_offset":2124,"ancestor_record_offset":1540,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2000,"parent_record_offset":1968,"ancestor_record_offset":1540,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":1968,"parent_record_offset":1936,"ancestor_record_offset":1540,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":1904,"parent_record_offset":1872,"ancestor_record_offset":1540,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":1936,"parent_record_offset":1904,"ancestor_record_offset":1540,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":1872,"parent_record_offset":1840,"ancestor_record_offset":1540,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":222,"record_offset":1840,"parent_record_offset":1808,"ancestor_record_offset":1540,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":222,"record_offset":1808,"parent_record_offset":1776,"ancestor_record_offset":1540,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":2036,"parent_record_offset":2000,"ancestor_record_offset":1540,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":222,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":1540,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":222,"record_offset":2208,"parent_record_offset":2184,"ancestor_record_offset":1540,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":222,"record_offset":1776,"parent_record_offset":1748,"ancestor_record_offset":1540,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":222,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":1540,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":222,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1540,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":222,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":1540,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":222,"record_offset":1748,"parent_record_offset":1704,"ancestor_record_offset":1540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":222,"record_offset":2400,"parent_record_offset":2000,"ancestor_record_offset":1540,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":222,"record_offset":3092,"parent_record_offset":3064,"ancestor_record_offset":1540,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":222,"record_offset":2492,"parent_record_offset":1704,"ancestor_record_offset":1540,"inlinee":15195,"source_line":64,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3B5B
// 首条位置: {"module":222,"inlinee":15195}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":222,"record_offset":2520,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":2492,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":2492,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":222,"record_offset":2868,"parent_record_offset":2840,"ancestor_record_offset":2492,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2780,"parent_record_offset":2752,"ancestor_record_offset":2492,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2840,"parent_record_offset":2808,"ancestor_record_offset":2492,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":222,"record_offset":2808,"parent_record_offset":2780,"ancestor_record_offset":2492,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2716,"parent_record_offset":2688,"ancestor_record_offset":2492,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":2492,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2632,"parent_record_offset":2604,"ancestor_record_offset":2492,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":2492,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":222,"record_offset":2604,"parent_record_offset":2576,"ancestor_record_offset":2492,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":222,"record_offset":2576,"parent_record_offset":2548,"ancestor_record_offset":2492,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":222,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":2492,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":222,"record_offset":2752,"parent_record_offset":2716,"ancestor_record_offset":2492,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":222,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":2492,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00222.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":222,"record_offset":2968,"parent_record_offset":2716,"ancestor_record_offset":2492,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00222.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 13; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,bevy_ecs::removal_detection::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::removal_detection::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >,bevy_ecs::removal_detection::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::removal_detection::RemovedComponents<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>", "bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >", "bevy_ecs::removal_detection::RemovedComponents<country_core::components::AutoClutterConvertableToDecoClutter>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume>,ref$<country_core::geometry::instanced_wall::InstancedWall> >,bevy_ecs::query::filter::Changed<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] query_without_filter: bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Changed<country_core::resources::roofs::roof_shape::Roof> > >,bevy_ecs::query::filter::Without<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,tuple$<bevy_ecs::query::filter::With<country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume>,bevy_ecs::query::filter::Added<country_core::resources::roofs::roof_animation::RoofMarkedForDeletion> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::ArchSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::arches::LintelSegments>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref$<country_core::components::AutoClutterConvertableToDecoClutter> >,bevy_ecs::query::filter::Changed<utils::components::transform::Transform> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>,bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>", "bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::events::ClearCanvasCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::save::save_game::AutoSaveState>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::new
// 信息 ID: 2d7d15b414d4e4e0a2ab65b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 21 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String>; module=15, inline_record=20816; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >; module=252, inline_record=70804; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >; module=487, inline_record=34680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline; module=487, inline_record=75644; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline; module=495, inline_record=89424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline; module=534, inline_record=53896; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline; module=544, inline_record=94388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline; module=549, inline_record=15816; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry
// 信息 ID: 92885ba18b9c5eae0dd6c887
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 18 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::vermeer::impl$7::as_compute_pipeline; module=487, inline_record=76904; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::impl$8::as_compute_pipeline; module=495, inline_record=90684; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::bin_lights::impl$2::as_compute_pipeline; module=534, inline_record=55156; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::taa::impl$1::as_compute_pipeline; module=544, inline_record=95648; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$3::as_compute_pipeline; module=549, inline_record=17076; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$7::as_compute_pipeline; module=549, inline_record=20980; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$11::as_compute_pipeline; module=549, inline_record=24892; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::impl$15::as_compute_pipeline; module=549, inline_record=28608; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >
// 信息 ID: 8d84530ca412764c59687410
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String> @ 0x56e2b -> 0x539370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x5cf1ea -> 0x539370 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> > @ 0x8a442a -> 0x9e3ab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment
// 信息 ID: fb4e545189cc23bff97a9041
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_shader_into_library<ref$<str$> >; module=587, inline_record=123376; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_shader_into_library<alloc::string::String>; module=587, inline_record=124748; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_shader_into_library_replace_previous<ref$<str$> >; module=587, inline_record=126128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment<ref$<str$> >
// 信息 ID: 676b8f10f6e03aa3c52f12a3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f6e723bf3a293de264db1c5c: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::pipeline_desc::RasterPipelineDesc::new_vertex_fragment -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > > @ 0x14b4bb7 -> 0x9e3980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > > @ 0x14d7057 -> 0x9e3980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > > @ 0x15026b7 -> 0x9e3980 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>)> > > @ 0x15057d7 -> 0x9e3980 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$14::clone
// 信息 ID: 0168acfdc370ebcc97c58d61
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_variants::get_raster_shader_variants; module=1692, inline_record=988; 不是独立调用边。
// [PDB 内联宿主] shader_variants::get_raster_shader_variants; module=1692, inline_record=2704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$16::eq
// 信息 ID: 3ca3844bfdab4ba9fab8be47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::shader_library::AssetShaderLibrary::add_raster; module=509, inline_record=52500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$1::with_entry::closure$0
// 信息 ID: 02742875af27018be897d871
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::pipeline_desc::impl$1::with_entry", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >; module=222, inline_record=2492; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::pipeline_desc::ComputePipelineDesc::with_entry<ref$<str$> >; module=534, inline_record=2304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::pipeline_desc::impl$8::read_from
// 信息 ID: b363272ee161b49e2d651d6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=12672; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
