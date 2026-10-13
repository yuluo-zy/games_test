// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/utils/src/rdp.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: b49a6c481759fef6347d0670af15381682435017dd3ad51e7830ee794b427f3a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: rdp.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 554, 567, 1415, 1417, 1467, 1585, 1610, 3036
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 6
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7BAE
// 首条位置: {"module":554,"inlinee":31662}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":51300,"parent_record_offset":51248,"ancestor_record_offset":51248,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":52512,"parent_record_offset":51248,"ancestor_record_offset":51248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":51772,"parent_record_offset":51748,"ancestor_record_offset":51248,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":51796,"parent_record_offset":51772,"ancestor_record_offset":51248,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":52696,"parent_record_offset":52672,"ancestor_record_offset":51248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":51748,"parent_record_offset":51572,"ancestor_record_offset":51248,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":52672,"parent_record_offset":52584,"ancestor_record_offset":51248,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":51512,"parent_record_offset":51484,"ancestor_record_offset":51248,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":52612,"parent_record_offset":52584,"ancestor_record_offset":51248,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":52584,"parent_record_offset":52560,"ancestor_record_offset":51248,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":51572,"parent_record_offset":51540,"ancestor_record_offset":51248,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":51540,"parent_record_offset":51512,"ancestor_record_offset":51248,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":52560,"parent_record_offset":52536,"ancestor_record_offset":51248,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":51608,"parent_record_offset":51572,"ancestor_record_offset":51248,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":51444,"parent_record_offset":51408,"ancestor_record_offset":51248,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":554,"record_offset":51408,"parent_record_offset":51336,"ancestor_record_offset":51248,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":51380,"parent_record_offset":51336,"ancestor_record_offset":51248,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":51484,"parent_record_offset":51444,"ancestor_record_offset":51248,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":51336,"parent_record_offset":51300,"ancestor_record_offset":51248,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":51632,"parent_record_offset":51608,"ancestor_record_offset":51248,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":51656,"parent_record_offset":51632,"ancestor_record_offset":51248,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":554,"record_offset":51872,"parent_record_offset":51848,"ancestor_record_offset":51248,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":51680,"parent_record_offset":51656,"ancestor_record_offset":51248,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":51704,"parent_record_offset":51680,"ancestor_record_offset":51248,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":52640,"parent_record_offset":52612,"ancestor_record_offset":51248,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":51848,"parent_record_offset":51444,"ancestor_record_offset":51248,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":52536,"parent_record_offset":52512,"ancestor_record_offset":51248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::stitch_with
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EBF
// 首条位置: {"module":567,"inlinee":32447}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":29328,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::remove
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29944,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":6099,"source_line":2040,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29168,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$19::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":31604,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":4917,"source_line":3492,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$21::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30456,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":10301,"source_line":3726,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":32752,"parent_record_offset":29048,"ancestor_record_offset":29048,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":29796,"parent_record_offset":29772,"ancestor_record_offset":29048,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":29820,"parent_record_offset":29796,"ancestor_record_offset":29048,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":29048,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":29772,"parent_record_offset":29588,"ancestor_record_offset":29048,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":32912,"parent_record_offset":32824,"ancestor_record_offset":29048,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30040,"parent_record_offset":30016,"ancestor_record_offset":29048,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30584,"parent_record_offset":30556,"ancestor_record_offset":29048,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":29516,"parent_record_offset":29480,"ancestor_record_offset":29048,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":32852,"parent_record_offset":32824,"ancestor_record_offset":29048,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":32824,"parent_record_offset":32800,"ancestor_record_offset":29048,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30644,"parent_record_offset":30612,"ancestor_record_offset":29048,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30088,"parent_record_offset":30064,"ancestor_record_offset":29048,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30064,"parent_record_offset":30040,"ancestor_record_offset":29048,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":30612,"parent_record_offset":30584,"ancestor_record_offset":29048,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":29588,"parent_record_offset":29552,"ancestor_record_offset":29048,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":29552,"parent_record_offset":29516,"ancestor_record_offset":29048,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":32800,"parent_record_offset":32776,"ancestor_record_offset":29048,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":29632,"parent_record_offset":29588,"ancestor_record_offset":29048,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":29436,"parent_record_offset":29400,"ancestor_record_offset":29048,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":29400,"parent_record_offset":29364,"ancestor_record_offset":29048,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":29364,"parent_record_offset":29328,"ancestor_record_offset":29048,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30512,"parent_record_offset":30484,"ancestor_record_offset":29048,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30016,"parent_record_offset":29944,"ancestor_record_offset":29048,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":31668,"parent_record_offset":31636,"ancestor_record_offset":29048,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29988,"parent_record_offset":29944,"ancestor_record_offset":29048,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30556,"parent_record_offset":30512,"ancestor_record_offset":29048,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":30168,"parent_record_offset":29944,"ancestor_record_offset":29048,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":29480,"parent_record_offset":29436,"ancestor_record_offset":29048,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":32260,"parent_record_offset":32216,"ancestor_record_offset":29048,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32360,"parent_record_offset":32260,"ancestor_record_offset":29048,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32712,"parent_record_offset":31668,"ancestor_record_offset":29048,"inlinee":6531,"source_line":13,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":32540,"parent_record_offset":32512,"ancestor_record_offset":29048,"inlinee":6529,"source_line":30,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":31636,"parent_record_offset":31604,"ancestor_record_offset":29048,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":30484,"parent_record_offset":30456,"ancestor_record_offset":29048,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":29656,"parent_record_offset":29632,"ancestor_record_offset":29048,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":29680,"parent_record_offset":29656,"ancestor_record_offset":29048,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":29896,"parent_record_offset":29872,"ancestor_record_offset":29048,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":31388,"parent_record_offset":31364,"ancestor_record_offset":29048,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":31900,"parent_record_offset":31868,"ancestor_record_offset":29048,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":32040,"parent_record_offset":31932,"ancestor_record_offset":29048,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":31868,"parent_record_offset":31668,"ancestor_record_offset":29048,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":32216,"parent_record_offset":32040,"ancestor_record_offset":29048,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":29704,"parent_record_offset":29680,"ancestor_record_offset":29048,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":29728,"parent_record_offset":29704,"ancestor_record_offset":29048,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":32400,"parent_record_offset":31932,"ancestor_record_offset":29048,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":32880,"parent_record_offset":32852,"ancestor_record_offset":29048,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":30668,"parent_record_offset":30644,"ancestor_record_offset":29048,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":32160,"parent_record_offset":32116,"ancestor_record_offset":29048,"inlinee":32453,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$16::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":567,"record_offset":32116,"parent_record_offset":32072,"ancestor_record_offset":29048,"inlinee":32451,"source_line":35,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::into_slice_range
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":567,"record_offset":29252,"parent_record_offset":29224,"ancestor_record_offset":29048,"inlinee":4608,"source_line":454,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":29872,"parent_record_offset":29436,"ancestor_record_offset":29048,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":32428,"parent_record_offset":31932,"ancestor_record_offset":29048,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":30128,"parent_record_offset":29944,"ancestor_record_offset":29048,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":32012,"parent_record_offset":31932,"ancestor_record_offset":29048,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":32312,"parent_record_offset":32260,"ancestor_record_offset":29048,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":29196,"parent_record_offset":29168,"ancestor_record_offset":29048,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":29224,"parent_record_offset":29196,"ancestor_record_offset":29048,"inlinee":4606,"source_line":680,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":29280,"parent_record_offset":29224,"ancestor_record_offset":29048,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":31932,"parent_record_offset":31900,"ancestor_record_offset":29048,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::stitch_with::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":32072,"parent_record_offset":32040,"ancestor_record_offset":29048,"inlinee":32449,"source_line":44,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::stitch_with::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 44
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EC1
// 首条位置: {"module":567,"inlinee":32449}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ops::arith::impl$16::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":567,"record_offset":32116,"parent_record_offset":32072,"ancestor_record_offset":32072,"inlinee":32451,"source_line":35,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":32160,"parent_record_offset":32116,"ancestor_record_offset":32072,"inlinee":32453,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::into_indices
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11627
// 首条位置: {"module":1467,"inlinee":71207}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":7908,"parent_record_offset":7880,"ancestor_record_offset":7880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":7880,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1467,"record_offset":8072,"parent_record_offset":7980,"ancestor_record_offset":7880,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":7880,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7880,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1467,"record_offset":7956,"parent_record_offset":7932,"ancestor_record_offset":7880,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1467,"record_offset":8040,"parent_record_offset":8008,"ancestor_record_offset":7880,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1467,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":7880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 记录形式: compiled_procedure; instances=1
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":567,"record_offset":2040,"rva":11345888,"va":5380055008}
// 编译实例 RVAs: 0xad1fe0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4973639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8b91bb4ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5380055122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4974961","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e88f16b4ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5380056460,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t31790923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e84b17e501","size":5,"soft_fail":false,"target_va":5411847168,"va":5380056240,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t720075","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8cbfc0a00","size":5,"soft_fail":false,"target_va":5380775408,"va":5380055328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t31484324","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8a469e001","size":5,"soft_fail":false,"target_va":5411540963,"va":5380056634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31485120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8c06ce001","size":5,"soft_fail":false,"target_va":5411541776,"va":5380056651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31485495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8376ee001","size":5,"soft_fail":false,"target_va":5411542064,"va":5380056564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t31485680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e8f06ee001","size":5,"soft_fail":false,"target_va":5411542296,"va":5380056611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31486528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380055008,"raw_hex":"e84072e001","size":5,"soft_fail":false,"target_va":5411543120,"va":5380056587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] utils::rdp::Rdp::new_with_opts
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":2168,"parent_record_offset":2040,"ancestor_record_offset":2040,"inlinee":32440,"source_line":81,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2040,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2040,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":2040,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2800,"parent_record_offset":2768,"ancestor_record_offset":2040,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":2040,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":2040,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2040,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":2040,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":2040,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":2040,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":2040,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2040,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2040,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":2040,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":6752,"parent_record_offset":6716,"ancestor_record_offset":2040,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3340,"parent_record_offset":3304,"ancestor_record_offset":2040,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3304,"parent_record_offset":3268,"ancestor_record_offset":2040,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3232,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3268,"parent_record_offset":3232,"ancestor_record_offset":2040,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":2040,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4376,"parent_record_offset":4332,"ancestor_record_offset":2040,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6532,"parent_record_offset":6492,"ancestor_record_offset":2040,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2040,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3668,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4332,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3724,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2652,"parent_record_offset":2448,"ancestor_record_offset":2040,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":2040,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6460,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2040,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2040,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":2040,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":7808,"parent_record_offset":7752,"ancestor_record_offset":2040,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":2904,"parent_record_offset":2448,"ancestor_record_offset":2040,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":567,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":2040,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":567,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2040,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":2040,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":2040,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":5816,"parent_record_offset":5196,"ancestor_record_offset":2040,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":2040,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2040,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2040,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":2040,"inlinee":25349,"source_line":60,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":2040,"inlinee":11866,"source_line":54,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":2040,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":2484,"parent_record_offset":2448,"ancestor_record_offset":2040,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2040,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":567,"record_offset":4876,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":9205,"source_line":122,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":2364,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2040,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2040,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":2040,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":4756,"parent_record_offset":4724,"ancestor_record_offset":2040,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":2040,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":2040,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":2040,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2040,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2040,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":567,"record_offset":4120,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":567,"record_offset":4964,"parent_record_offset":4912,"ancestor_record_offset":2040,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":3596,"parent_record_offset":3340,"ancestor_record_offset":2040,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2040,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":7868,"parent_record_offset":7196,"ancestor_record_offset":2040,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":2040,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2040,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2040,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2040,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":4792,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":4092,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":4696,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":2040,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":567,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":2040,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2040,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":2040,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":2040,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":2040,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":2040,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":2040,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":2040,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5568,"parent_record_offset":5460,"ancestor_record_offset":2040,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4156,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":567,"record_offset":4292,"parent_record_offset":4260,"ancestor_record_offset":2040,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5236,"parent_record_offset":5196,"ancestor_record_offset":2040,"inlinee":32443,"source_line":164,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":5292,"ancestor_record_offset":2040,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":4212,"parent_record_offset":2168,"ancestor_record_offset":2040,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":2040,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":2040,"inlinee":24908,"source_line":179,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":5196,"parent_record_offset":5160,"ancestor_record_offset":2040,"inlinee":32442,"source_line":127,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":7696,"parent_record_offset":7668,"ancestor_record_offset":2040,"inlinee":32444,"source_line":175,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":2040,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EB8
// 首条位置: {"module":567,"inlinee":32440}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3232,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3668,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4332,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3724,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6460,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":567,"record_offset":4876,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":9205,"source_line":122,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":2364,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":567,"record_offset":4120,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":8052,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":4792,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":567,"record_offset":4092,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":4696,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4156,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":4212,"parent_record_offset":2168,"ancestor_record_offset":2168,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2824,"parent_record_offset":2800,"ancestor_record_offset":2168,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2168,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":7988,"parent_record_offset":7964,"ancestor_record_offset":2168,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":2800,"parent_record_offset":2768,"ancestor_record_offset":2168,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":567,"record_offset":7964,"parent_record_offset":7940,"ancestor_record_offset":2168,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":2168,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2708,"parent_record_offset":2680,"ancestor_record_offset":2168,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":8152,"parent_record_offset":8124,"ancestor_record_offset":2168,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":2168,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3828,"parent_record_offset":3804,"ancestor_record_offset":2168,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":3804,"parent_record_offset":3780,"ancestor_record_offset":2168,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2768,"parent_record_offset":2736,"ancestor_record_offset":2168,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":2736,"parent_record_offset":2708,"ancestor_record_offset":2168,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":2168,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":567,"record_offset":6752,"parent_record_offset":6716,"ancestor_record_offset":2168,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3340,"parent_record_offset":3304,"ancestor_record_offset":2168,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3304,"parent_record_offset":3268,"ancestor_record_offset":2168,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":567,"record_offset":3268,"parent_record_offset":3232,"ancestor_record_offset":2168,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3756,"parent_record_offset":3724,"ancestor_record_offset":2168,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":4376,"parent_record_offset":4332,"ancestor_record_offset":2168,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6532,"parent_record_offset":6492,"ancestor_record_offset":2168,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":2168,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2652,"parent_record_offset":2448,"ancestor_record_offset":2168,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2680,"parent_record_offset":2652,"ancestor_record_offset":2168,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":2168,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":3124,"parent_record_offset":3100,"ancestor_record_offset":2168,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":567,"record_offset":6492,"parent_record_offset":6460,"ancestor_record_offset":2168,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":567,"record_offset":7808,"parent_record_offset":7752,"ancestor_record_offset":2168,"inlinee":4710,"source_line":18,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":567,"record_offset":2904,"parent_record_offset":2448,"ancestor_record_offset":2168,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":567,"record_offset":2420,"parent_record_offset":2392,"ancestor_record_offset":2168,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":567,"record_offset":2448,"parent_record_offset":2420,"ancestor_record_offset":2168,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":2168,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":567,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":2168,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":5816,"parent_record_offset":5196,"ancestor_record_offset":2168,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":567,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":2168,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":3000,"parent_record_offset":2976,"ancestor_record_offset":2168,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":3052,"parent_record_offset":3024,"ancestor_record_offset":2168,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":4912,"parent_record_offset":4876,"ancestor_record_offset":2168,"inlinee":25349,"source_line":60,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":567,"record_offset":2508,"parent_record_offset":2484,"ancestor_record_offset":2168,"inlinee":11866,"source_line":54,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":2976,"parent_record_offset":2952,"ancestor_record_offset":2168,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":2484,"parent_record_offset":2448,"ancestor_record_offset":2168,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":567,"record_offset":3076,"parent_record_offset":3052,"ancestor_record_offset":2168,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":2952,"parent_record_offset":2928,"ancestor_record_offset":2168,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":567,"record_offset":3100,"parent_record_offset":3076,"ancestor_record_offset":2168,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":2168,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":4756,"parent_record_offset":4724,"ancestor_record_offset":2168,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":2168,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":567,"record_offset":8184,"parent_record_offset":8152,"ancestor_record_offset":2168,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":2168,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":2168,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":4180,"parent_record_offset":4156,"ancestor_record_offset":2168,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":567,"record_offset":4964,"parent_record_offset":4912,"ancestor_record_offset":2168,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":3596,"parent_record_offset":3340,"ancestor_record_offset":2168,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":567,"record_offset":2604,"parent_record_offset":2580,"ancestor_record_offset":2168,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":7868,"parent_record_offset":7196,"ancestor_record_offset":2168,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":2168,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":567,"record_offset":2580,"parent_record_offset":2556,"ancestor_record_offset":2168,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":567,"record_offset":2556,"parent_record_offset":2532,"ancestor_record_offset":2168,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":567,"record_offset":3148,"parent_record_offset":3124,"ancestor_record_offset":2168,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":567,"record_offset":4724,"parent_record_offset":4696,"ancestor_record_offset":2168,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":567,"record_offset":4816,"parent_record_offset":4792,"ancestor_record_offset":2168,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":3024,"parent_record_offset":3000,"ancestor_record_offset":2168,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":4936,"parent_record_offset":4912,"ancestor_record_offset":2168,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":567,"record_offset":2532,"parent_record_offset":2508,"ancestor_record_offset":2168,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4260,"parent_record_offset":4236,"ancestor_record_offset":2168,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":4236,"parent_record_offset":4212,"ancestor_record_offset":2168,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":2168,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":2168,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5568,"parent_record_offset":5460,"ancestor_record_offset":2168,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":2168,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":567,"record_offset":4292,"parent_record_offset":4260,"ancestor_record_offset":2168,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5236,"parent_record_offset":5196,"ancestor_record_offset":2168,"inlinee":32443,"source_line":164,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":5292,"ancestor_record_offset":2168,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5692,"parent_record_offset":5236,"ancestor_record_offset":2168,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":2168,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":2168,"inlinee":24908,"source_line":179,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":5196,"parent_record_offset":5160,"ancestor_record_offset":2168,"inlinee":32442,"source_line":127,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":567,"record_offset":7696,"parent_record_offset":7668,"ancestor_record_offset":2168,"inlinee":32444,"source_line":175,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":2168,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3036,"record_offset":272,"rva":33426320,"va":5402135440}
// 编译实例 RVAs: 0x1fe0b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27054106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8e62f63fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5402135589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27056048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8502863fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5402137547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141206920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14527087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8915522ff","size":5,"soft_fail":false,"target_va":5387610400,"va":5402137482,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: rdp.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9709788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8dc289400","size":5,"soft_fail":false,"target_va":5411847168,"va":5402137375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t-21360406","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8ea10bafe","size":5,"soft_fail":false,"target_va":5380775408,"va":5402135809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t9403226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e85a7b8f00","size":5,"soft_fail":false,"target_va":5411540963,"va":5402137732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9404025","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8797e8f00","size":5,"soft_fail":false,"target_va":5411541776,"va":5402137746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9404370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8d27f8f00","size":5,"soft_fail":false,"target_va":5411542064,"va":5402137689,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t9404475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e83b808f00","size":5,"soft_fail":false,"target_va":5411542296,"va":5402137816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t9405403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5402135440,"raw_hex":"e8db838f00","size":5,"soft_fail":false,"target_va":5411543120,"va":5402137712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3036,"record_offset":1164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":1744,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":2452,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":1800,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":2628,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":3036,"record_offset":3612,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9205,"source_line":122,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3036,"record_offset":464,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3036,"record_offset":2196,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3036,"record_offset":6704,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3036,"record_offset":3528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3036,"record_offset":2168,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4716,"source_line":279,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3036,"record_offset":3432,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":2232,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":2320,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3036,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":272,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3036,"record_offset":780,"parent_record_offset":756,"ancestor_record_offset":272,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3036,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":272,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3036,"record_offset":732,"parent_record_offset":700,"ancestor_record_offset":272,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":3036,"record_offset":6868,"parent_record_offset":6776,"ancestor_record_offset":272,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":1856,"parent_record_offset":1832,"ancestor_record_offset":272,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":640,"parent_record_offset":612,"ancestor_record_offset":272,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":6804,"parent_record_offset":6776,"ancestor_record_offset":272,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":272,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":272,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":1880,"parent_record_offset":1856,"ancestor_record_offset":272,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":700,"parent_record_offset":668,"ancestor_record_offset":272,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":668,"parent_record_offset":640,"ancestor_record_offset":272,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":272,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3036,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":272,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3036,"record_offset":1272,"parent_record_offset":1236,"ancestor_record_offset":272,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3036,"record_offset":1236,"parent_record_offset":1200,"ancestor_record_offset":272,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":3036,"record_offset":1200,"parent_record_offset":1164,"ancestor_record_offset":272,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":1832,"parent_record_offset":1800,"ancestor_record_offset":272,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":2496,"parent_record_offset":2452,"ancestor_record_offset":272,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":2708,"parent_record_offset":2664,"ancestor_record_offset":272,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":860,"parent_record_offset":836,"ancestor_record_offset":272,"inlinee":4678,"source_line":3552,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":584,"parent_record_offset":548,"ancestor_record_offset":272,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":612,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":492,"parent_record_offset":464,"ancestor_record_offset":272,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$20::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":272,"inlinee":4708,"source_line":3565,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3036,"record_offset":2664,"parent_record_offset":2628,"ancestor_record_offset":272,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":3036,"record_offset":836,"parent_record_offset":548,"ancestor_record_offset":272,"inlinee":4677,"source_line":28,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":3036,"record_offset":520,"parent_record_offset":492,"ancestor_record_offset":272,"inlinee":4673,"source_line":33,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":3036,"record_offset":548,"parent_record_offset":520,"ancestor_record_offset":272,"inlinee":4675,"source_line":52,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3036,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":272,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":3036,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":272,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3036,"record_offset":4856,"parent_record_offset":3980,"ancestor_record_offset":272,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":3036,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":272,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3036,"record_offset":932,"parent_record_offset":908,"ancestor_record_offset":272,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3036,"record_offset":984,"parent_record_offset":956,"ancestor_record_offset":272,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3036,"record_offset":3652,"parent_record_offset":3612,"ancestor_record_offset":272,"inlinee":25349,"source_line":60,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3036,"record_offset":908,"parent_record_offset":884,"ancestor_record_offset":272,"inlinee":4681,"source_line":124,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":3036,"record_offset":1008,"parent_record_offset":984,"ancestor_record_offset":272,"inlinee":4692,"source_line":88,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3036,"record_offset":6668,"parent_record_offset":6644,"ancestor_record_offset":272,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3036,"record_offset":884,"parent_record_offset":860,"ancestor_record_offset":272,"inlinee":4679,"source_line":790,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3036,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":272,"inlinee":4706,"source_line":797,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3036,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":272,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3036,"record_offset":3492,"parent_record_offset":3460,"ancestor_record_offset":272,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3036,"record_offset":1528,"parent_record_offset":1504,"ancestor_record_offset":272,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3036,"record_offset":6836,"parent_record_offset":6804,"ancestor_record_offset":272,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3036,"record_offset":4540,"parent_record_offset":4508,"ancestor_record_offset":272,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3036,"record_offset":4392,"parent_record_offset":4364,"ancestor_record_offset":272,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3036,"record_offset":2260,"parent_record_offset":2232,"ancestor_record_offset":272,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3036,"record_offset":3704,"parent_record_offset":3652,"ancestor_record_offset":272,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3036,"record_offset":1672,"parent_record_offset":1272,"ancestor_record_offset":272,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3036,"record_offset":3892,"parent_record_offset":3868,"ancestor_record_offset":272,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3036,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":272,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3036,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":272,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3036,"record_offset":3868,"parent_record_offset":3844,"ancestor_record_offset":272,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3036,"record_offset":3844,"parent_record_offset":3772,"ancestor_record_offset":272,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3036,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":272,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3036,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":272,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3036,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":272,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3036,"record_offset":956,"parent_record_offset":932,"ancestor_record_offset":272,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::nth
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3036,"record_offset":3676,"parent_record_offset":3652,"ancestor_record_offset":272,"inlinee":15380,"source_line":182,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":2376,"parent_record_offset":2348,"ancestor_record_offset":272,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":272,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":272,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4336,"parent_record_offset":4308,"ancestor_record_offset":272,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4364,"parent_record_offset":4336,"ancestor_record_offset":272,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4508,"parent_record_offset":4336,"ancestor_record_offset":272,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":272,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3036,"record_offset":2412,"parent_record_offset":2376,"ancestor_record_offset":272,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4052,"parent_record_offset":3980,"ancestor_record_offset":272,"inlinee":32443,"source_line":164,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4248,"parent_record_offset":4128,"ancestor_record_offset":272,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4720,"parent_record_offset":4052,"ancestor_record_offset":272,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":5652,"parent_record_offset":3980,"ancestor_record_offset":272,"inlinee":106160,"source_line":94,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4308,"parent_record_offset":4092,"ancestor_record_offset":272,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4092,"parent_record_offset":4052,"ancestor_record_offset":272,"inlinee":24908,"source_line":179,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::rdp::impl$0::new_with_opts::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/rdp.rs
//     evidence={"module":3036,"record_offset":3980,"parent_record_offset":3928,"ancestor_record_offset":272,"inlinee":32442,"source_line":127,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4128,"parent_record_offset":4092,"ancestor_record_offset":272,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EBA
// 首条位置: {"module":567,"inlinee":32442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":567,"record_offset":5816,"parent_record_offset":5196,"ancestor_record_offset":5196,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":5736,"parent_record_offset":3980,"ancestor_record_offset":3980,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_direct] utils::distance_line_point
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5236,"parent_record_offset":5196,"ancestor_record_offset":5196,"inlinee":32443,"source_line":164,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_direct] utils::impl$1::length_squared
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":5652,"parent_record_offset":3980,"ancestor_record_offset":3980,"inlinee":106160,"source_line":94,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5592,"parent_record_offset":5568,"ancestor_record_offset":5196,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":5196,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":567,"record_offset":5344,"parent_record_offset":5320,"ancestor_record_offset":5196,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":5196,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5716,"parent_record_offset":5692,"ancestor_record_offset":5196,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5460,"parent_record_offset":5436,"ancestor_record_offset":5196,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5484,"parent_record_offset":5460,"ancestor_record_offset":5196,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5568,"parent_record_offset":5460,"ancestor_record_offset":5196,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":567,"record_offset":5320,"parent_record_offset":5292,"ancestor_record_offset":5196,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":3980,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4748,"parent_record_offset":4720,"ancestor_record_offset":3980,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_squared
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":3980,"inlinee":24882,"source_line":461,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4336,"parent_record_offset":4308,"ancestor_record_offset":3980,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4364,"parent_record_offset":4336,"ancestor_record_offset":3980,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4508,"parent_record_offset":4336,"ancestor_record_offset":3980,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3036,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":3980,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":567,"record_offset":5772,"parent_record_offset":5740,"ancestor_record_offset":5196,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::dot
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5376,"parent_record_offset":5292,"ancestor_record_offset":5196,"inlinee":29396,"source_line":83,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::length
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5692,"parent_record_offset":5236,"ancestor_record_offset":5196,"inlinee":9567,"source_line":58,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5436,"parent_record_offset":5264,"ancestor_record_offset":5196,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::dot
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4248,"parent_record_offset":4128,"ancestor_record_offset":3980,"inlinee":24443,"source_line":114,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::length
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4720,"parent_record_offset":4052,"ancestor_record_offset":3980,"inlinee":9543,"source_line":89,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::lerp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3036,"record_offset":4308,"parent_record_offset":4092,"ancestor_record_offset":3980,"inlinee":24442,"source_line":109,"archive":"_pdb/B/modules/03036.jsonl.gz"}
//   [pdb_inline_transitive] utils::project_point_onto_line
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":5196,"inlinee":24908,"source_line":179,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   [pdb_inline_transitive] utils::t_of_point_projection_onto_segment
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":567,"record_offset":5292,"parent_record_offset":5264,"ancestor_record_offset":5196,"inlinee":24441,"source_line":152,"archive":"_pdb/B/modules/00567.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: utils::rdp::impl$0::new_with_opts::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 175
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7EBC
// 首条位置: {"module":567,"inlinee":32444}
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
// 完整结构及原始类型记录: rdp.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] utils::rdp::Rdp
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<utils::rdp::Rdp<glam::f32::vec2::Vec2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["utils::rdp::Rdp<glam::f32::vec2::Vec2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new<glam::f32::vec2::Vec2>", "constructor": "utils::rdp::Rdp::new", "arguments": ["glam::f32::vec2::Vec2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>", "constructor": "utils::rdp::Rdp::new_with_opts", "arguments": ["glam::f32::vec3::Vec3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: rdp.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-structures/B-profile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "utils::rdp::Rdp", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: rdp.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::option::Option<country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::PushConstants> > (*)(ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >)
// 候选参数类型: ["ref_mut$<core::iter::adapters::flatten::FlatMap<core::option::Iter<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTileSsbo> >,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::render::pass_builder::draw_stream::InstancedDraw,alloc::alloc::Global>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure$0::closure_env$0>,country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute::closure$0::closure_env$0> >"]
// [文件级接口候选，具体函数归属 unknown] nalgebra::geometry::point::OPoint<f32,nalgebra::base::dimension::Const<3> > (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] relative_path::RelativePathBuf (*)(ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::into_indices
// 信息 ID: bac917b6f10473ff50670f78
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2>; module=1467, inline_record=7880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new<glam::f32::vec2::Vec2>
// 信息 ID: 4f207307daccda49ba56b715
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new -> ["glam::f32::vec2::Vec2"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::new @ 0xad34d6 -> 0xad1fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xad4742 -> 0xad1fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xad4395 -> 0xad1fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xad4517 -> 0xad1fe0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve @ 0xad4310 -> 0xad1fe0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts
// 信息 ID: 313036da3b97a8b06b377b39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=2168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>
// 信息 ID: 87838b36cb35fb9109eb5df4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] utils::rdp::Rdp::new_with_opts -> ["glam::f32::vec3::Vec3"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_lintels::place_support_beams::SupportBeamBuilder::emit_support_beams<system_wall_constructor::construct_lintels::construct_lintels::closure_env$2> @ 0x1243d00 -> 0x1fe0b90 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::Rdp::stitch_with
// 信息 ID: bf90b6f879f17f4b5c233943
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve; module=567, inline_record=29048; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$1
// 信息 ID: d0f87a9243cfc36f599781f9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=5196; 不是独立调用边。
// [PDB 内联宿主] utils::rdp::Rdp::new_with_opts<glam::f32::vec3::Vec3>; module=3036, inline_record=3980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::new_with_opts::closure$2
// 信息 ID: ec6357a297f74e0e46a28832
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "utils::rdp::impl$0::new_with_opts", "captures": "unknown"}
// [闭包环境类型引用] utils::rdp::impl$0::new_with_opts::closure_env$2 <- alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] utils::rdp::Rdp::new<glam::f32::vec2::Vec2>; module=567, inline_record=7696; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,utils::rdp::impl$0::new_with_opts::closure_env$2<glam::f32::vec3::Vec3> > >; module=1415, inline_record=90400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$0::stitch_with::closure$0
// 信息 ID: 06441ede1d6ad1a2e6ed6e41
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "utils::rdp::impl$0::stitch_with", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::in_progress_curve::InProgressCurve::compute_final_curve; module=567, inline_record=32072; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: utils::rdp::impl$1::clone
// 信息 ID: e9e01f39addd0f132c9db9b0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::wall::draw_wall::draw_wall; module=554, inline_record=51248; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
