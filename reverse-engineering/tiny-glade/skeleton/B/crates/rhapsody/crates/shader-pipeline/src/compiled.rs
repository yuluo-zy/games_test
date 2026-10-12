// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/crates/shader-pipeline/src/compiled.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: c8c20064026577d3ac2c2ada033638d7528c90998e047f91b5dfecd6c2bd9dd2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: compiled.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 534, 1712, 3151
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use speedy; // namespace candidate
//   package identities: [{"name":"speedy","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$2::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7729
// 首条位置: {"module":534,"inlinee":30505}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":15116,"parent_record_offset":12552,"ancestor_record_offset":12552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":13608,"parent_record_offset":12552,"ancestor_record_offset":12552,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":12868,"parent_record_offset":12552,"ancestor_record_offset":12552,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":12628,"parent_record_offset":12552,"ancestor_record_offset":12552,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":13104,"parent_record_offset":12552,"ancestor_record_offset":12552,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13412,"parent_record_offset":13384,"ancestor_record_offset":12552,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13440,"parent_record_offset":13412,"ancestor_record_offset":12552,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":12552,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13384,"parent_record_offset":13336,"ancestor_record_offset":12552,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13888,"parent_record_offset":13832,"ancestor_record_offset":12552,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13264,"parent_record_offset":13228,"ancestor_record_offset":12552,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13860,"parent_record_offset":13832,"ancestor_record_offset":12552,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13832,"parent_record_offset":13808,"ancestor_record_offset":12552,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13336,"parent_record_offset":13300,"ancestor_record_offset":12552,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13300,"parent_record_offset":13264,"ancestor_record_offset":12552,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":12552,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":13724,"parent_record_offset":13684,"ancestor_record_offset":12552,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":13192,"parent_record_offset":13104,"ancestor_record_offset":12552,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":13228,"parent_record_offset":13192,"ancestor_record_offset":12552,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":12552,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":14644,"parent_record_offset":14540,"ancestor_record_offset":12552,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":534,"record_offset":13648,"parent_record_offset":13608,"ancestor_record_offset":12552,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":13060,"parent_record_offset":12992,"ancestor_record_offset":12552,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":13760,"parent_record_offset":13724,"ancestor_record_offset":12552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":13964,"parent_record_offset":13684,"ancestor_record_offset":12552,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$8::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":534,"record_offset":12672,"parent_record_offset":12628,"ancestor_record_offset":12552,"inlinee":30507,"source_line":3,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":534,"record_offset":14020,"parent_record_offset":13992,"ancestor_record_offset":12552,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":13684,"parent_record_offset":13648,"ancestor_record_offset":12552,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":12760,"parent_record_offset":12716,"ancestor_record_offset":12552,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":12792,"parent_record_offset":12716,"ancestor_record_offset":12552,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":12716,"parent_record_offset":12672,"ancestor_record_offset":12552,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::impl$4::read_from
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x771E
// 首条位置: {"module":534,"inlinee":30494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":24832,"parent_record_offset":11656,"ancestor_record_offset":11656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":25560,"parent_record_offset":11656,"ancestor_record_offset":11656,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":11700,"parent_record_offset":11656,"ancestor_record_offset":11656,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":17932,"parent_record_offset":11656,"ancestor_record_offset":11656,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":11840,"parent_record_offset":11656,"ancestor_record_offset":11656,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":11656,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":11656,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13912,"parent_record_offset":13888,"ancestor_record_offset":11656,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":12368,"parent_record_offset":12192,"ancestor_record_offset":11656,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":13888,"parent_record_offset":13832,"ancestor_record_offset":11656,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":12132,"parent_record_offset":12100,"ancestor_record_offset":11656,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13860,"parent_record_offset":13832,"ancestor_record_offset":11656,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13832,"parent_record_offset":13808,"ancestor_record_offset":11656,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":12192,"parent_record_offset":12160,"ancestor_record_offset":11656,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":12160,"parent_record_offset":12132,"ancestor_record_offset":11656,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":13808,"parent_record_offset":13784,"ancestor_record_offset":11656,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":12228,"parent_record_offset":12192,"ancestor_record_offset":11656,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::from_utf8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":534,"record_offset":13724,"parent_record_offset":13684,"ancestor_record_offset":11656,"inlinee":14307,"source_line":576,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":12068,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":12100,"parent_record_offset":12068,"ancestor_record_offset":11656,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":534,"record_offset":24860,"parent_record_offset":24832,"ancestor_record_offset":11656,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":11656,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":12276,"parent_record_offset":12252,"ancestor_record_offset":11656,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":11656,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":534,"record_offset":20964,"parent_record_offset":20932,"ancestor_record_offset":11656,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":19144,"parent_record_offset":19116,"ancestor_record_offset":11656,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21024,"parent_record_offset":21000,"ancestor_record_offset":11656,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":534,"record_offset":21196,"parent_record_offset":21048,"ancestor_record_offset":11656,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":20732,"parent_record_offset":20524,"ancestor_record_offset":11656,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21048,"parent_record_offset":21024,"ancestor_record_offset":11656,"inlinee":5298,"source_line":2539,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21000,"parent_record_offset":20868,"ancestor_record_offset":11656,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":11656,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":534,"record_offset":19116,"parent_record_offset":19092,"ancestor_record_offset":11656,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":12300,"parent_record_offset":12276,"ancestor_record_offset":11656,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":12012,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":534,"record_offset":25300,"parent_record_offset":25268,"ancestor_record_offset":11656,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":534,"record_offset":13648,"parent_record_offset":13608,"ancestor_record_offset":11656,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=24
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":534,"record_offset":11800,"parent_record_offset":11748,"ancestor_record_offset":11656,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=48
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":13760,"parent_record_offset":13724,"ancestor_record_offset":11656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":15640,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_unaligned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":21108,"parent_record_offset":21076,"ancestor_record_offset":11656,"inlinee":6431,"source_line":1461,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":15608,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::and_then
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":13608,"parent_record_offset":12552,"ancestor_record_offset":11656,"inlinee":4667,"source_line":1351,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":13964,"parent_record_offset":13684,"ancestor_record_offset":11656,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19172,"parent_record_offset":19116,"ancestor_record_offset":11656,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":19212,"parent_record_offset":18868,"ancestor_record_offset":11656,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":24516,"parent_record_offset":18008,"ancestor_record_offset":11656,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":11656,"inlinee":6916,"source_line":1103,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":20868,"parent_record_offset":20840,"ancestor_record_offset":11656,"inlinee":6915,"source_line":4480,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$82::extend::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":534,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":11656,"inlinee":6918,"source_line":4492,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::reserve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":20932,"parent_record_offset":20900,"ancestor_record_offset":11656,"inlinee":5655,"source_line":896,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":534,"record_offset":21912,"parent_record_offset":21888,"ancestor_record_offset":11656,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::impl$2::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":534,"record_offset":12552,"parent_record_offset":12472,"ancestor_record_offset":11656,"inlinee":30505,"source_line":11,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::pipeline_desc::impl$8::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs
//     evidence={"module":534,"record_offset":12672,"parent_record_offset":12628,"ancestor_record_offset":11656,"inlinee":30507,"source_line":3,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$25::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":19648,"parent_record_offset":19604,"ancestor_record_offset":11656,"inlinee":30528,"source_line":31,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$61::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21604,"parent_record_offset":21580,"ancestor_record_offset":11656,"inlinee":30537,"source_line":69,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$79::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":21344,"parent_record_offset":21320,"ancestor_record_offset":11656,"inlinee":30535,"source_line":75,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$84::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18868,"parent_record_offset":18844,"ancestor_record_offset":11656,"inlinee":30519,"source_line":78,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$89::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18176,"parent_record_offset":18120,"ancestor_record_offset":11656,"inlinee":30517,"source_line":83,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":18008,"parent_record_offset":17932,"ancestor_record_offset":11656,"inlinee":30515,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":19556,"parent_record_offset":19508,"ancestor_record_offset":11656,"inlinee":30526,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":23504,"parent_record_offset":23476,"ancestor_record_offset":11656,"inlinee":30538,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$94::read_from::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":534,"record_offset":23532,"parent_record_offset":23504,"ancestor_record_offset":11656,"inlinee":30540,"source_line":88,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::error::error_invalid_string_utf8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/error.rs
//     evidence={"module":534,"record_offset":14020,"parent_record_offset":13992,"ancestor_record_offset":11656,"inlinee":27487,"source_line":137,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":16616,"parent_record_offset":16572,"ancestor_record_offset":11656,"inlinee":30513,"source_line":195,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::read_length_u32
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":12868,"parent_record_offset":12552,"ancestor_record_offset":11656,"inlinee":30496,"source_line":159,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::private::vec_to_string
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/private.rs
//     evidence={"module":534,"record_offset":13684,"parent_record_offset":13648,"ancestor_record_offset":11656,"inlinee":30508,"source_line":39,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::can_read_at_least
//     owner=registry_dependency_definition; count=28
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":12040,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":30502,"source_line":150,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_borrowed_from_reader
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":20572,"parent_record_offset":20524,"ancestor_record_offset":11656,"inlinee":30533,"source_line":123,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable::impl$1::read_bytes_into_ptr
//     owner=registry_dependency_definition; count=25
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable.rs
//     evidence={"module":534,"record_offset":11748,"parent_record_offset":11724,"ancestor_record_offset":11656,"inlinee":30500,"source_line":56,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$2::read_from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":20308,"parent_record_offset":20272,"ancestor_record_offset":11656,"inlinee":30532,"source_line":54,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$34::read_from
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":11656,"inlinee":30530,"source_line":116,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$7::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":16476,"parent_record_offset":16436,"ancestor_record_offset":11656,"inlinee":30510,"source_line":187,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::readable_impl::impl$9::read_from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/readable_impl.rs
//     evidence={"module":534,"record_offset":16572,"parent_record_offset":16532,"ancestor_record_offset":11656,"inlinee":30512,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_collection
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19092,"parent_record_offset":19068,"ancestor_record_offset":11656,"inlinee":30521,"source_line":630,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_key_value_collection
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19068,"parent_record_offset":18868,"ancestor_record_offset":11656,"inlinee":30520,"source_line":652,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u32
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":11656,"inlinee":30498,"source_line":216,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_u8
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":19376,"parent_record_offset":18008,"ancestor_record_offset":11656,"inlinee":30524,"source_line":108,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_value
//     owner=registry_dependency_definition; count=18
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":12472,"parent_record_offset":11840,"ancestor_record_offset":11656,"inlinee":30503,"source_line":504,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::Reader::read_vec
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":13104,"parent_record_offset":12552,"ancestor_record_offset":11656,"inlinee":30501,"source_line":514,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] speedy::reader::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/speedy-0.8.6/src/reader.rs
//     evidence={"module":534,"record_offset":21076,"parent_record_offset":21048,"ancestor_record_offset":11656,"inlinee":30542,"source_line":22,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20812,"parent_record_offset":20768,"ancestor_record_offset":11656,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$87::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20768,"parent_record_offset":20732,"ancestor_record_offset":11656,"inlinee":11275,"source_line":3230,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$88::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":534,"record_offset":20840,"parent_record_offset":20768,"ancestor_record_offset":11656,"inlinee":11277,"source_line":3246,"archive":"_pdb/B/modules/00534.jsonl.gz"}
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
// 首条位置: {"module":534,"record_offset":3132,"rva":10370128,"va":5379079248}
// 编译实例 RVAs: 0x9e3c50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4179456","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e8003ac0ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5379080187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3998079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e881fec2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379079578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3748424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e848323900","size":5,"soft_fail":false,"target_va":5382827728,"va":5379079299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141795d10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14360720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e89020db00","size":5,"soft_fail":false,"target_va":5393440016,"va":5379079291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32261799","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e8a746ec01","size":5,"soft_fail":false,"target_va":5411341280,"va":5379079476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32767148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e8acfcf301","size":5,"soft_fail":false,"target_va":5411847168,"va":5379080015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3760739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e863623900","size":5,"soft_fail":false,"target_va":5382840208,"va":5379079464,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t2182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e886080000","size":5,"soft_fail":false,"target_va":5379081840,"va":5379079653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3625491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379079248,"raw_hex":"e813523700","size":5,"soft_fail":false,"target_va":5382704816,"va":5379079320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379079248,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379079549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":4572,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":3572,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":4544,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":534,"record_offset":3348,"parent_record_offset":3132,"ancestor_record_offset":3132,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3132,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":4136,"parent_record_offset":4048,"ancestor_record_offset":3132,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":534,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3132,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":3652,"parent_record_offset":3616,"ancestor_record_offset":3132,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3132,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":6000,"parent_record_offset":5968,"ancestor_record_offset":3132,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":4740,"parent_record_offset":4716,"ancestor_record_offset":3132,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":4716,"parent_record_offset":4692,"ancestor_record_offset":3132,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":4080,"parent_record_offset":4048,"ancestor_record_offset":3132,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":534,"record_offset":3372,"parent_record_offset":3348,"ancestor_record_offset":3132,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":534,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":3132,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":3132,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":3736,"parent_record_offset":3616,"ancestor_record_offset":3132,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3132,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3132,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3132,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":534,"record_offset":3616,"parent_record_offset":3572,"ancestor_record_offset":3132,"inlinee":30486,"source_line":30,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":534,"record_offset":4836,"parent_record_offset":4796,"ancestor_record_offset":3132,"inlinee":30487,"source_line":38,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":3132,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":3516,"parent_record_offset":3492,"ancestor_record_offset":3132,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":3492,"parent_record_offset":3468,"ancestor_record_offset":3132,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":3132,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":3792,"parent_record_offset":3764,"ancestor_record_offset":3132,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":3444,"parent_record_offset":3420,"ancestor_record_offset":3132,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":3420,"parent_record_offset":3396,"ancestor_record_offset":3132,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":3396,"parent_record_offset":3372,"ancestor_record_offset":3132,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00534.jsonl.gz"}
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
// 首条位置: {"module":534,"record_offset":6932,"rva":10371424,"va":5379080544}
// 编译实例 RVAs: 0x9e4160
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=2
//     target_aliases=92 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4180752","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e8f034c0ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5379081483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3999375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e871f9c2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5379080874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3747128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e8382d3900","size":5,"soft_fail":false,"target_va":5382827728,"va":5379080595,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141796080
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14360304","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e8f01edb00","size":5,"soft_fail":false,"target_va":5393440896,"va":5379080587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a83e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32260503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e89741ec01","size":5,"soft_fail":false,"target_va":5411341280,"va":5379080772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t32765852","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e89cf7f301","size":5,"soft_fail":false,"target_va":5411847168,"va":5379081311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t3759443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e8535d3900","size":5,"soft_fail":false,"target_va":5382840208,"va":5379080760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"speedy","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e876030000","size":5,"soft_fail":false,"target_va":5379081840,"va":5379080949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3624195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379080544,"raw_hex":"e8034d3700","size":5,"soft_fail":false,"target_va":5382704816,"va":5379080616,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5379080544,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5379080845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":8372,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":7372,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":534,"record_offset":8344,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":534,"record_offset":7148,"parent_record_offset":6932,"ancestor_record_offset":6932,"inlinee":12456,"source_line":267,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":7960,"parent_record_offset":7936,"ancestor_record_offset":6932,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":7936,"parent_record_offset":7848,"ancestor_record_offset":6932,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":534,"record_offset":7848,"parent_record_offset":7820,"ancestor_record_offset":6932,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":7452,"parent_record_offset":7416,"ancestor_record_offset":6932,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":7500,"parent_record_offset":7476,"ancestor_record_offset":6932,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":9800,"parent_record_offset":9768,"ancestor_record_offset":6932,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":6932,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":534,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":6932,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":7880,"parent_record_offset":7848,"ancestor_record_offset":6932,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":534,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":6932,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":534,"record_offset":7904,"parent_record_offset":7880,"ancestor_record_offset":6932,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":7476,"parent_record_offset":7452,"ancestor_record_offset":6932,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":7536,"parent_record_offset":7416,"ancestor_record_offset":6932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":7648,"parent_record_offset":7620,"ancestor_record_offset":6932,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":7696,"parent_record_offset":7672,"ancestor_record_offset":6932,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":7672,"parent_record_offset":7648,"ancestor_record_offset":6932,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":534,"record_offset":7416,"parent_record_offset":7372,"ancestor_record_offset":6932,"inlinee":30486,"source_line":30,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":534,"record_offset":8636,"parent_record_offset":8596,"ancestor_record_offset":6932,"inlinee":30487,"source_line":38,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":6932,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":6932,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":534,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":6932,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":7620,"parent_record_offset":7592,"ancestor_record_offset":6932,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":7592,"parent_record_offset":7564,"ancestor_record_offset":6932,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6932,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":7220,"parent_record_offset":7196,"ancestor_record_offset":6932,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":534,"record_offset":7196,"parent_record_offset":7172,"ancestor_record_offset":6932,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7716
// 首条位置: {"module":534,"inlinee":30486}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":3652,"parent_record_offset":3616,"ancestor_record_offset":3616,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":3736,"parent_record_offset":3616,"ancestor_record_offset":3616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":4160,"parent_record_offset":4136,"ancestor_record_offset":3616,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":4136,"parent_record_offset":4048,"ancestor_record_offset":3616,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":534,"record_offset":4048,"parent_record_offset":4020,"ancestor_record_offset":3616,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":3700,"parent_record_offset":3676,"ancestor_record_offset":3616,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":4080,"parent_record_offset":4048,"ancestor_record_offset":3616,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":534,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":3616,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":3616,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":3764,"parent_record_offset":3736,"ancestor_record_offset":3616,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3616,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3616,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":3872,"parent_record_offset":3848,"ancestor_record_offset":3616,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":3820,"parent_record_offset":3792,"ancestor_record_offset":3616,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":3792,"parent_record_offset":3764,"ancestor_record_offset":3616,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7717
// 首条位置: {"module":534,"inlinee":30487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":4872,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":4956,"parent_record_offset":4836,"ancestor_record_offset":4836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":5436,"parent_record_offset":5412,"ancestor_record_offset":4836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":534,"record_offset":5412,"parent_record_offset":5324,"ancestor_record_offset":4836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":534,"record_offset":5324,"parent_record_offset":5296,"ancestor_record_offset":4836,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":534,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":4836,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":534,"record_offset":5356,"parent_record_offset":5324,"ancestor_record_offset":4836,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":534,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":4836,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":534,"record_offset":4896,"parent_record_offset":4872,"ancestor_record_offset":4836,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":534,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":4836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":5124,"parent_record_offset":5096,"ancestor_record_offset":4836,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":5172,"parent_record_offset":5148,"ancestor_record_offset":4836,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":534,"record_offset":5148,"parent_record_offset":5124,"ancestor_record_offset":4836,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":5096,"parent_record_offset":5068,"ancestor_record_offset":4836,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/00534.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":534,"record_offset":5068,"parent_record_offset":5040,"ancestor_record_offset":4836,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/00534.jsonl.gz"}
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
// 首条位置: {"module":1712,"record_offset":552,"rva":24730896,"va":5393440016}
// 编译实例 RVAs: 0x1795d10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18358674","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440016,"raw_hex":"e86edee7fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393440157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18359265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440016,"raw_hex":"e81fdce7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393440764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cfbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9674375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440016,"raw_hex":"e8879e9300","size":5,"soft_fail":false,"target_va":5403114480,"va":5393440100,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10600537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440016,"raw_hex":"e8a73f5eff","size":5,"soft_fail":false,"target_va":5382840208,"va":5393440740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18100729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440016,"raw_hex":"e8f9311401","size":5,"soft_fail":false,"target_va":5411540963,"va":5393440229,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1712,"record_offset":2136,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":992,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":16882,"source_line":271,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1712,"record_offset":2448,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":2220,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::compiled::calculate_shader_defines_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1712,"record_offset":708,"parent_record_offset":552,"ancestor_record_offset":552,"inlinee":77189,"source_line":76,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":552,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":1552,"parent_record_offset":1528,"ancestor_record_offset":552,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":2396,"parent_record_offset":2372,"ancestor_record_offset":552,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":1504,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":2372,"parent_record_offset":2316,"ancestor_record_offset":552,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1712,"record_offset":2184,"parent_record_offset":2160,"ancestor_record_offset":552,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":552,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":2344,"parent_record_offset":2316,"ancestor_record_offset":552,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":552,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":1296,"parent_record_offset":1260,"ancestor_record_offset":552,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":552,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":552,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":1344,"parent_record_offset":1296,"ancestor_record_offset":552,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":1020,"parent_record_offset":992,"ancestor_record_offset":552,"inlinee":46825,"source_line":712,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":1816,"parent_record_offset":1780,"ancestor_record_offset":552,"inlinee":20340,"source_line":713,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":552,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":552,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":1176,"parent_record_offset":1132,"ancestor_record_offset":552,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":1204,"parent_record_offset":1176,"ancestor_record_offset":552,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":1076,"parent_record_offset":1048,"ancestor_record_offset":552,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":552,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":1916,"parent_record_offset":1868,"ancestor_record_offset":552,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":552,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1712,"record_offset":2040,"parent_record_offset":1916,"ancestor_record_offset":552,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1712,"record_offset":1608,"parent_record_offset":1132,"ancestor_record_offset":552,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1712,"record_offset":1104,"parent_record_offset":1076,"ancestor_record_offset":552,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1712,"record_offset":1132,"parent_record_offset":1104,"ancestor_record_offset":552,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1712,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":552,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1712,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":552,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1712,"record_offset":1456,"parent_record_offset":1428,"ancestor_record_offset":552,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1712,"record_offset":1692,"parent_record_offset":1664,"ancestor_record_offset":552,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1712,"record_offset":1780,"parent_record_offset":1720,"ancestor_record_offset":552,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":1048,"parent_record_offset":1020,"ancestor_record_offset":552,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":1664,"parent_record_offset":1636,"ancestor_record_offset":552,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":1868,"parent_record_offset":1780,"ancestor_record_offset":552,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1712,"record_offset":1428,"parent_record_offset":1400,"ancestor_record_offset":552,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1712,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":552,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":552,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":868,"parent_record_offset":820,"ancestor_record_offset":552,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":1988,"parent_record_offset":1916,"ancestor_record_offset":552,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":552,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":1720,"parent_record_offset":1692,"ancestor_record_offset":552,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":820,"parent_record_offset":708,"ancestor_record_offset":552,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":900,"parent_record_offset":820,"ancestor_record_offset":552,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1712,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":552,"inlinee":8206,"source_line":1836,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1712,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":552,"inlinee":8204,"source_line":1858,"archive":"_pdb/B/modules/01712.jsonl.gz"}
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
// 首条位置: {"module":1712,"record_offset":2764,"rva":24731776,"va":5393440896}
// 编译实例 RVAs: 0x1796080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18359553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440896,"raw_hex":"e8ffdae7fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5393441036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18360133","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440896,"raw_hex":"e8bbd8e7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5393441632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cfbf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9673495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440896,"raw_hex":"e8179b9300","size":5,"soft_fail":false,"target_va":5403114480,"va":5393440980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10601405","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440896,"raw_hex":"e8433c5eff","size":5,"soft_fail":false,"target_va":5382840208,"va":5393441608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18099851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393440896,"raw_hex":"e88b2e1401","size":5,"soft_fail":false,"target_va":5411540963,"va":5393441107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1712,"record_offset":4348,"parent_record_offset":2764,"ancestor_record_offset":2764,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":3204,"parent_record_offset":2764,"ancestor_record_offset":2764,"inlinee":16882,"source_line":271,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1712,"record_offset":4660,"parent_record_offset":2764,"ancestor_record_offset":2764,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":4432,"parent_record_offset":2764,"ancestor_record_offset":2764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] shader_pipeline::compiled::calculate_shader_defines_hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/compiled.rs
//     evidence={"module":1712,"record_offset":2920,"parent_record_offset":2764,"ancestor_record_offset":2764,"inlinee":77189,"source_line":76,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":3740,"parent_record_offset":3716,"ancestor_record_offset":2764,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":3764,"parent_record_offset":3740,"ancestor_record_offset":2764,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":4608,"parent_record_offset":4584,"ancestor_record_offset":2764,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":3716,"parent_record_offset":3508,"ancestor_record_offset":2764,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1712,"record_offset":4584,"parent_record_offset":4528,"ancestor_record_offset":2764,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1712,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":2764,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":3444,"parent_record_offset":3416,"ancestor_record_offset":2764,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":4556,"parent_record_offset":4528,"ancestor_record_offset":2764,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":4528,"parent_record_offset":4504,"ancestor_record_offset":2764,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":3508,"parent_record_offset":3472,"ancestor_record_offset":2764,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":3472,"parent_record_offset":3444,"ancestor_record_offset":2764,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2764,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1712,"record_offset":3556,"parent_record_offset":3508,"ancestor_record_offset":2764,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":3232,"parent_record_offset":3204,"ancestor_record_offset":2764,"inlinee":46825,"source_line":712,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::replace_ascii::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1712,"record_offset":4028,"parent_record_offset":3992,"ancestor_record_offset":2764,"inlinee":20340,"source_line":713,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":2764,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":2764,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":3388,"parent_record_offset":3344,"ancestor_record_offset":2764,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":3416,"parent_record_offset":3388,"ancestor_record_offset":2764,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":3288,"parent_record_offset":3260,"ancestor_record_offset":2764,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":2948,"parent_record_offset":2920,"ancestor_record_offset":2764,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":4128,"parent_record_offset":4080,"ancestor_record_offset":2764,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2764,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":1712,"record_offset":4252,"parent_record_offset":4128,"ancestor_record_offset":2764,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1712,"record_offset":3820,"parent_record_offset":3344,"ancestor_record_offset":2764,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":1712,"record_offset":3316,"parent_record_offset":3288,"ancestor_record_offset":2764,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":1712,"record_offset":3344,"parent_record_offset":3316,"ancestor_record_offset":2764,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1712,"record_offset":3584,"parent_record_offset":3556,"ancestor_record_offset":2764,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1712,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":2764,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1712,"record_offset":3668,"parent_record_offset":3640,"ancestor_record_offset":2764,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1712,"record_offset":3904,"parent_record_offset":3876,"ancestor_record_offset":2764,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1712,"record_offset":3992,"parent_record_offset":3932,"ancestor_record_offset":2764,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":3260,"parent_record_offset":3232,"ancestor_record_offset":2764,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":2764,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1712,"record_offset":4080,"parent_record_offset":3992,"ancestor_record_offset":2764,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1712,"record_offset":3640,"parent_record_offset":3612,"ancestor_record_offset":2764,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1712,"record_offset":4372,"parent_record_offset":4348,"ancestor_record_offset":2764,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":4456,"parent_record_offset":4432,"ancestor_record_offset":2764,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2764,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":3080,"parent_record_offset":3032,"ancestor_record_offset":2764,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1712,"record_offset":4200,"parent_record_offset":4128,"ancestor_record_offset":2764,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2764,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":3932,"parent_record_offset":3904,"ancestor_record_offset":2764,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":3032,"parent_record_offset":2920,"ancestor_record_offset":2764,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":3112,"parent_record_offset":3032,"ancestor_record_offset":2764,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$38::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1712,"record_offset":4708,"parent_record_offset":4684,"ancestor_record_offset":2764,"inlinee":8206,"source_line":1836,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$40::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1712,"record_offset":4684,"parent_record_offset":4660,"ancestor_record_offset":2764,"inlinee":8204,"source_line":1858,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: shader_pipeline::compiled::calculate_shader_defines_hash
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12D85
// 首条位置: {"module":1712,"inlinee":77189}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":736,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":820,"parent_record_offset":708,"ancestor_record_offset":708,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":708,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1712,"record_offset":760,"parent_record_offset":736,"ancestor_record_offset":708,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":948,"parent_record_offset":924,"ancestor_record_offset":708,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1712,"record_offset":868,"parent_record_offset":820,"ancestor_record_offset":708,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":924,"parent_record_offset":900,"ancestor_record_offset":708,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01712.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1712,"record_offset":900,"parent_record_offset":820,"ancestor_record_offset":708,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01712.jsonl.gz"}
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
// 首条位置: {"module":3151,"record_offset":1452,"rva":34405360,"va":5403114480}
// 编译实例 RVAs: 0x20cfbf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140106160
//     owner=unresolved_namespace; count=4
//     target_aliases=35 candidates; complete list: compiled.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33331968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5403114480,"raw_hex":"e8006503fe","size":5,"soft_fail":false,"target_va":5369782624,"va":5403114587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::hash::impls::impl$23::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":3151,"record_offset":1668,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":9028,"source_line":892,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::DefaultHasher::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3151,"record_offset":1640,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":8959,"source_line":109,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_direct] std::hash::random::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3151,"record_offset":2036,"parent_record_offset":1452,"ancestor_record_offset":1452,"inlinee":8980,"source_line":142,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$25::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":3151,"record_offset":1692,"parent_record_offset":1668,"ancestor_record_offset":1452,"inlinee":9032,"source_line":2594,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":3151,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":1452,"inlinee":9036,"source_line":363,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$2::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":3151,"record_offset":1716,"parent_record_offset":1692,"ancestor_record_offset":1452,"inlinee":6318,"source_line":863,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1452,"inlinee":8981,"source_line":241,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":1764,"parent_record_offset":1740,"ancestor_record_offset":1452,"inlinee":9034,"source_line":236,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::finish
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":1452,"inlinee":8983,"source_line":310,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$5::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":1452,"inlinee":9035,"source_line":302,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::c_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":2128,"parent_record_offset":2084,"ancestor_record_offset":1452,"inlinee":8985,"source_line":360,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$8::d_rounds
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":3151,"record_offset":2472,"parent_record_offset":2084,"ancestor_record_offset":1452,"inlinee":8987,"source_line":365,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::rotate_left
//     owner=rust_standard_library; count=23
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3151,"record_offset":2216,"parent_record_offset":2128,"ancestor_record_offset":1452,"inlinee":8986,"source_line":260,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3151,"record_offset":2188,"parent_record_offset":2128,"ancestor_record_offset":1452,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/03151.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":3151,"record_offset":1740,"parent_record_offset":1716,"ancestor_record_offset":1452,"inlinee":9033,"source_line":137,"archive":"_pdb/B/modules/03151.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$0
// 信息 ID: 4baf3c79017d3c41de4b86e7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>; module=534, inline_record=3616; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>; module=534, inline_record=7416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled::closure$1
// 信息 ID: fa161b7220b38f9b4ad5a669
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>; module=534, inline_record=4836; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>; module=534, inline_record=8636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc>
// 信息 ID: f17f1741dbddc09e92b0dd1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled -> ["shader_pipeline::pipeline_desc::ComputePipelineDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::ComputeShaderProgram::new @ 0x937e4f -> 0x9e3c50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc>
// 信息 ID: 819e70b42ca88af8000696a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled -> ["shader_pipeline::pipeline_desc::RasterPipelineDesc"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x938482 -> 0x9e4160 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::shader::compile_shader_variants @ 0x938619 -> 0x9e4160 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::calculate_hash<tuple$<alloc::string::String,alloc::string::String> >
// 信息 ID: eb6f76a127336d7fb46c6785
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["alloc::string::String", "alloc::string::String"]; 字段布局 = unknown。
// [类型关系] shader_pipeline::compiled::calculate_hash -> ["tuple$<alloc::string::String,alloc::string::String>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::impl$0::compiled_file_path @ 0x1795d64 -> 0x20cfbf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] shader_pipeline::compiled::impl$1::compiled_file_path @ 0x17960d4 -> 0x20cfbf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::calculate_shader_defines_hash
// 信息 ID: 7e5ee8e23becb36629fea752
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] shader_pipeline::compiled::impl$0::compiled_file_path; module=1712, inline_record=708; 不是独立调用边。
// [PDB 内联宿主] shader_pipeline::compiled::impl$1::compiled_file_path; module=1712, inline_record=2920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$0::compiled_file_path
// 信息 ID: 68f5caf9411f5cb7e8decd72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::ComputePipelineDesc> @ 0x9e3c7b -> 0x1795d10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$1::compiled_file_path
// 信息 ID: c1452a5f4af33ac2111f97a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] shader_pipeline::compiled::ShaderCompilerArtifacts::load_compiled<shader_pipeline::pipeline_desc::RasterPipelineDesc> @ 0x9e418b -> 0x1796080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$2::read_from
// 信息 ID: 8f650092a4e523ce7cab14e6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=12552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: shader_pipeline::compiled::impl$4::read_from
// 信息 ID: ff854db4e17567ef878051ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] speedy::readable::Readable::read_from_buffer<shader_pipeline::compiled::CompiledShaderPipeline,speedy::context::LittleEndian>; module=534, inline_record=11656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
