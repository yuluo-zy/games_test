// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/menu/credits.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: ef40ad1777ffcac702180cf26e27301ca58006f450ea5ca2f3431e85f1f4bdb0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: credits.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1181, 2755, 2759, 4531
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use once_cell; // namespace candidate
//   package identities: [{"name":"once_cell","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.21.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 18
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1855F
// 首条位置: {"module":2755,"inlinee":99679}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2755,"record_offset":1764,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2755,"record_offset":892,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":2755,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":836,"inlinee":9324,"source_line":310,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":836,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":836,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":1380,"parent_record_offset":1356,"ancestor_record_offset":836,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":836,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":1356,"parent_record_offset":1272,"ancestor_record_offset":836,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2755,"record_offset":1272,"parent_record_offset":1244,"ancestor_record_offset":836,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":836,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":836,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2072,"parent_record_offset":2044,"ancestor_record_offset":836,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":836,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":836,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1896,"parent_record_offset":1852,"ancestor_record_offset":836,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":836,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1852,"parent_record_offset":1824,"ancestor_record_offset":836,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2755,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":836,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":2016,"parent_record_offset":1980,"ancestor_record_offset":836,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2755,"record_offset":1300,"parent_record_offset":1272,"ancestor_record_offset":836,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":836,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2755,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":836,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2755,"record_offset":2232,"parent_record_offset":1980,"ancestor_record_offset":836,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":932,"parent_record_offset":892,"ancestor_record_offset":836,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2755,"record_offset":1072,"parent_record_offset":1044,"ancestor_record_offset":836,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2755,"record_offset":1120,"parent_record_offset":1096,"ancestor_record_offset":836,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2755,"record_offset":1096,"parent_record_offset":1072,"ancestor_record_offset":836,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2755,"record_offset":1044,"parent_record_offset":1016,"ancestor_record_offset":836,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":2755,"record_offset":1016,"parent_record_offset":988,"ancestor_record_offset":836,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2755,"record_offset":1796,"parent_record_offset":1764,"ancestor_record_offset":836,"inlinee":99681,"source_line":23,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 23
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18561
// 首条位置: {"module":2755,"inlinee":99681}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2755,"record_offset":1824,"parent_record_offset":1796,"ancestor_record_offset":1796,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2156,"parent_record_offset":2132,"ancestor_record_offset":1796,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":1796,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":2132,"parent_record_offset":2104,"ancestor_record_offset":1796,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2044,"parent_record_offset":2016,"ancestor_record_offset":1796,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2104,"parent_record_offset":2072,"ancestor_record_offset":1796,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":2072,"parent_record_offset":2044,"ancestor_record_offset":1796,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1980,"parent_record_offset":1952,"ancestor_record_offset":1796,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1952,"parent_record_offset":1924,"ancestor_record_offset":1796,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1896,"parent_record_offset":1852,"ancestor_record_offset":1796,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1924,"parent_record_offset":1896,"ancestor_record_offset":1796,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":1852,"parent_record_offset":1824,"ancestor_record_offset":1796,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":2016,"parent_record_offset":1980,"ancestor_record_offset":1796,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":2256,"parent_record_offset":2232,"ancestor_record_offset":1796,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2755,"record_offset":2232,"parent_record_offset":1980,"ancestor_record_offset":1796,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":4224,"rva":31432816,"va":5400141936}
// 编译实例 RVAs: 0x1dfa070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=7
//     target_aliases=7 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25466696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8b8687bfe","size":5,"soft_fail":false,"target_va":5374676336,"va":5400143027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25416315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8852d7cfe","size":5,"soft_fail":false,"target_va":5374728464,"va":5400144774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25413034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8563a7cfe","size":5,"soft_fail":false,"target_va":5374729632,"va":5400142661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25414730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8b6337cfe","size":5,"soft_fail":false,"target_va":5374730016,"va":5400144741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25063982","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8d28d81fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400145481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f99fe0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15074844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8e4f919ff","size":5,"soft_fail":false,"target_va":5385068512,"va":5400143351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9ba30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15070201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8070c1aff","size":5,"soft_fail":false,"target_va":5385075248,"va":5400145444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9cab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15063464","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e858261aff","size":5,"soft_fail":false,"target_va":5385079472,"va":5400142931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9cc20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15062990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e832281aff","size":5,"soft_fail":false,"target_va":5385079840,"va":5400142825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15062557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8e3291aff","size":5,"soft_fail":false,"target_va":5385083184,"va":5400145736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa04f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15048425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e817611aff","size":5,"soft_fail":false,"target_va":5385094384,"va":5400142804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15003212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8b4111bff","size":5,"soft_fail":false,"target_va":5385142608,"va":5400145815,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15002272","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e860151bff","size":5,"soft_fail":false,"target_va":5385143488,"va":5400145755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-24938948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e83c7683fe","size":5,"soft_fail":false,"target_va":5375203792,"va":5400142735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11395211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e88be0ad00","size":5,"soft_fail":false,"target_va":5411541584,"va":5400146368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=7
//     evidence={"assembly":"\tcallq\t11397642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e80aeaad00","size":5,"soft_fail":false,"target_va":5411541776,"va":5400144129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e878fbffff","size":5,"soft_fail":false,"target_va":5400140864,"va":5400142019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11401259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e82bf8ad00","size":5,"soft_fail":false,"target_va":5411543360,"va":5400142096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18757003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e875cae1fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5400142534,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18751814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8badee1fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5400142449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18757179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8c5c9e1fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5400143654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18757299","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e84dc9e1fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5400141998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11448639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e83fb1ae00","size":5,"soft_fail":false,"target_va":5411594832,"va":5400146188,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-14857721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8074a1dff","size":5,"soft_fail":false,"target_va":5385288736,"va":5400146452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11390743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e817cfad00","size":5,"soft_fail":false,"target_va":5411537104,"va":5400146356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_family
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19309394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8ae5cd9fe","size":5,"soft_fail":false,"target_va":5380836480,"va":5400145869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24601772","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e8549b88fe","size":5,"soft_fail":false,"target_va":5375540928,"va":5400142695,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24594936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400141936,"raw_hex":"e808b688fe","size":5,"soft_fail":false,"target_va":5375549872,"va":5400144803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2755,"record_offset":6828,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":9676,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":5844,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":4412,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":4868,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen_without_block_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":6888,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":61862,"source_line":439,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":12440,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":12678,"source_line":473,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":10984,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":18952,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":20043,"source_line":295,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":13844,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":7408,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":8544,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17696,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17608,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":19822,"source_line":266,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17520,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17316,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$29::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17784,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":61826,"source_line":198,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::background_color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2755,"record_offset":7176,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":99683,"source_line":304,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::max_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2755,"record_offset":7264,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":99685,"source_line":309,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2755,"record_offset":7352,"parent_record_offset":4224,"ancestor_record_offset":4224,"inlinee":99686,"source_line":314,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":16404,"parent_record_offset":16380,"ancestor_record_offset":4224,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":16380,"parent_record_offset":16324,"ancestor_record_offset":4224,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":7048,"parent_record_offset":7016,"ancestor_record_offset":4224,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":16352,"parent_record_offset":16324,"ancestor_record_offset":4224,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":16324,"parent_record_offset":16300,"ancestor_record_offset":4224,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":7096,"parent_record_offset":7072,"ancestor_record_offset":4224,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":7072,"parent_record_offset":7048,"ancestor_record_offset":4224,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":16300,"parent_record_offset":16276,"ancestor_record_offset":4224,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":7016,"parent_record_offset":6980,"ancestor_record_offset":4224,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":6980,"parent_record_offset":6912,"ancestor_record_offset":4224,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2755,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":4224,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6180,"parent_record_offset":6152,"ancestor_record_offset":4224,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6232,"parent_record_offset":6208,"ancestor_record_offset":4224,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6208,"parent_record_offset":6180,"ancestor_record_offset":4224,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6128,"parent_record_offset":6100,"ancestor_record_offset":4224,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6152,"parent_record_offset":6128,"ancestor_record_offset":4224,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6324,"parent_record_offset":6152,"ancestor_record_offset":4224,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2755,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":4224,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2755,"record_offset":8268,"parent_record_offset":8244,"ancestor_record_offset":4224,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2755,"record_offset":7776,"parent_record_offset":7752,"ancestor_record_offset":4224,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2755,"record_offset":7860,"parent_record_offset":7836,"ancestor_record_offset":4224,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":4224,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":8336,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2755,"record_offset":6256,"parent_record_offset":6232,"ancestor_record_offset":4224,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2755,"record_offset":4568,"parent_record_offset":4536,"ancestor_record_offset":4224,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2755,"record_offset":8008,"parent_record_offset":7980,"ancestor_record_offset":4224,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2755,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":4224,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2755,"record_offset":5996,"parent_record_offset":5952,"ancestor_record_offset":4224,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2755,"record_offset":8512,"parent_record_offset":7408,"ancestor_record_offset":4224,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2755,"record_offset":4816,"parent_record_offset":4788,"ancestor_record_offset":4224,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2755,"record_offset":6784,"parent_record_offset":5952,"ancestor_record_offset":4224,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":6388,"parent_record_offset":6100,"ancestor_record_offset":4224,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2755,"record_offset":8128,"parent_record_offset":8104,"ancestor_record_offset":4224,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":7448,"parent_record_offset":7408,"ancestor_record_offset":4224,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4224,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4224,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2755,"record_offset":5464,"parent_record_offset":5436,"ancestor_record_offset":4224,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":6280,"parent_record_offset":6256,"ancestor_record_offset":4224,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":5208,"parent_record_offset":5184,"ancestor_record_offset":4224,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4224,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":5536,"parent_record_offset":5508,"ancestor_record_offset":4224,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2755,"record_offset":5184,"parent_record_offset":5068,"ancestor_record_offset":4224,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2755,"record_offset":4536,"parent_record_offset":4508,"ancestor_record_offset":4224,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2755,"record_offset":4896,"parent_record_offset":4868,"ancestor_record_offset":4224,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2755,"record_offset":5036,"parent_record_offset":5004,"ancestor_record_offset":4224,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2755,"record_offset":5004,"parent_record_offset":4968,"ancestor_record_offset":4224,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2755,"record_offset":4944,"parent_record_offset":4920,"ancestor_record_offset":4224,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2755,"record_offset":4968,"parent_record_offset":4944,"ancestor_record_offset":4224,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2755,"record_offset":4920,"parent_record_offset":4896,"ancestor_record_offset":4224,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2755,"record_offset":5508,"parent_record_offset":4968,"ancestor_record_offset":4224,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4508,"parent_record_offset":4476,"ancestor_record_offset":4224,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":4224,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4668,"parent_record_offset":4636,"ancestor_record_offset":4224,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4764,"parent_record_offset":4668,"ancestor_record_offset":4224,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4636,"parent_record_offset":4612,"ancestor_record_offset":4224,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4448,"parent_record_offset":4412,"ancestor_record_offset":4224,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2755,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":4224,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2755,"record_offset":5068,"parent_record_offset":5036,"ancestor_record_offset":4224,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2755,"record_offset":5352,"parent_record_offset":5068,"ancestor_record_offset":4224,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2755,"record_offset":5324,"parent_record_offset":5068,"ancestor_record_offset":4224,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2755,"record_offset":5128,"parent_record_offset":5068,"ancestor_record_offset":4224,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2755,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":4224,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":7920,"parent_record_offset":7892,"ancestor_record_offset":4224,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":7952,"parent_record_offset":7920,"ancestor_record_offset":4224,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2755,"record_offset":7892,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":7752,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":8364,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2755,"record_offset":7836,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2755,"record_offset":7724,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2755,"record_offset":8244,"parent_record_offset":8220,"ancestor_record_offset":4224,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2755,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":4224,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2755,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":4224,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2755,"record_offset":8220,"parent_record_offset":8052,"ancestor_record_offset":4224,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2755,"record_offset":7564,"parent_record_offset":7532,"ancestor_record_offset":4224,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":8160,"parent_record_offset":8076,"ancestor_record_offset":4224,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":8104,"parent_record_offset":8076,"ancestor_record_offset":4224,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":8456,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":8076,"parent_record_offset":8052,"ancestor_record_offset":4224,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":7616,"parent_record_offset":7592,"ancestor_record_offset":4224,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":7592,"parent_record_offset":7532,"ancestor_record_offset":4224,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":4224,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":7696,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2755,"record_offset":8052,"parent_record_offset":7640,"ancestor_record_offset":4224,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":5880,"parent_record_offset":5844,"ancestor_record_offset":4224,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":9740,"parent_record_offset":9708,"ancestor_record_offset":4224,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":10256,"parent_record_offset":9996,"ancestor_record_offset":4224,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2755,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":4224,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":6100,"parent_record_offset":5952,"ancestor_record_offset":4224,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2755,"record_offset":6360,"parent_record_offset":6100,"ancestor_record_offset":4224,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2755,"record_offset":7476,"parent_record_offset":7408,"ancestor_record_offset":4224,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2755,"record_offset":6052,"parent_record_offset":6024,"ancestor_record_offset":4224,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2755,"record_offset":5952,"parent_record_offset":5916,"ancestor_record_offset":4224,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2755,"record_offset":5916,"parent_record_offset":5880,"ancestor_record_offset":4224,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":4224,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":17428,"parent_record_offset":17368,"ancestor_record_offset":4224,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":17368,"parent_record_offset":17316,"ancestor_record_offset":4224,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":4224,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":17816,"parent_record_offset":17784,"ancestor_record_offset":4224,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2755,"record_offset":18976,"parent_record_offset":18952,"ancestor_record_offset":4224,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":6952,"parent_record_offset":6912,"ancestor_record_offset":4224,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":13892,"parent_record_offset":13868,"ancestor_record_offset":4224,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":13920,"parent_record_offset":13868,"ancestor_record_offset":4224,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":13868,"parent_record_offset":13844,"ancestor_record_offset":4224,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17548,"parent_record_offset":17520,"ancestor_record_offset":4224,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":17484,"parent_record_offset":17368,"ancestor_record_offset":4224,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":4224,"inlinee":99687,"source_line":205,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":4224,"inlinee":99684,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF17D
// 首条位置: {"module":1181,"inlinee":61821}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1181,"record_offset":127788,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$5::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1181,"record_offset":116120,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":16882,"source_line":271,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1181,"record_offset":126556,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1181,"record_offset":126528,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1181,"record_offset":118652,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":129004,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":119820,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":20192,"source_line":1442,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":120556,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":7092,"source_line":159,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::strip_prefix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":120344,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":9143,"source_line":2397,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::trim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":120128,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":14223,"source_line":2125,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":119876,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] once_cell::sync::impl$12::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1181,"record_offset":115876,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":6040,"source_line":1375,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":127732,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19917,"source_line":336,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":122944,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":20043,"source_line":295,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":120584,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":120696,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":121824,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123576,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::family
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123488,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19822,"source_line":266,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123400,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":128740,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":19823,"source_line":271,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label_with_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123224,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":61822,"source_line":50,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$28::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":125380,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":61828,"source_line":192,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$29::deref_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123664,"parent_record_offset":115664,"ancestor_record_offset":115664,"inlinee":61826,"source_line":198,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":116404,"parent_record_offset":116380,"ancestor_record_offset":115664,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":116428,"parent_record_offset":116404,"ancestor_record_offset":115664,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":118596,"parent_record_offset":118572,"ancestor_record_offset":115664,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":116380,"parent_record_offset":116348,"ancestor_record_offset":115664,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":118572,"parent_record_offset":118516,"ancestor_record_offset":115664,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":117140,"parent_record_offset":117096,"ancestor_record_offset":115664,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":116960,"parent_record_offset":116932,"ancestor_record_offset":115664,"inlinee":4962,"source_line":326,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":116288,"parent_record_offset":116260,"ancestor_record_offset":115664,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":117044,"parent_record_offset":117020,"ancestor_record_offset":115664,"inlinee":4737,"source_line":512,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":118544,"parent_record_offset":118516,"ancestor_record_offset":115664,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":118516,"parent_record_offset":118492,"ancestor_record_offset":115664,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":117020,"parent_record_offset":116988,"ancestor_record_offset":115664,"inlinee":4964,"source_line":622,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":117188,"parent_record_offset":117164,"ancestor_record_offset":115664,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":117164,"parent_record_offset":117140,"ancestor_record_offset":115664,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":116988,"parent_record_offset":116960,"ancestor_record_offset":115664,"inlinee":4963,"source_line":541,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":116348,"parent_record_offset":116316,"ancestor_record_offset":115664,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":116316,"parent_record_offset":116288,"ancestor_record_offset":115664,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":118492,"parent_record_offset":118468,"ancestor_record_offset":115664,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":128148,"parent_record_offset":128104,"ancestor_record_offset":115664,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1181,"record_offset":127968,"parent_record_offset":127932,"ancestor_record_offset":115664,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1181,"record_offset":127932,"parent_record_offset":127896,"ancestor_record_offset":115664,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1181,"record_offset":127860,"parent_record_offset":127824,"ancestor_record_offset":115664,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1181,"record_offset":127896,"parent_record_offset":127860,"ancestor_record_offset":115664,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1181,"record_offset":127824,"parent_record_offset":127788,"ancestor_record_offset":115664,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1181,"record_offset":116636,"parent_record_offset":116120,"ancestor_record_offset":115664,"inlinee":6418,"source_line":1126,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1181,"record_offset":116204,"parent_record_offset":116120,"ancestor_record_offset":115664,"inlinee":9147,"source_line":489,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":116888,"parent_record_offset":116708,"ancestor_record_offset":115664,"inlinee":5428,"source_line":2565,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":116672,"parent_record_offset":116636,"ancestor_record_offset":115664,"inlinee":5425,"source_line":3033,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":117228,"parent_record_offset":116888,"ancestor_record_offset":115664,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":116932,"parent_record_offset":116888,"ancestor_record_offset":115664,"inlinee":4961,"source_line":1271,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":116232,"parent_record_offset":116204,"ancestor_record_offset":115664,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1181,"record_offset":116260,"parent_record_offset":116232,"ancestor_record_offset":115664,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1181,"record_offset":116708,"parent_record_offset":116672,"ancestor_record_offset":115664,"inlinee":5427,"source_line":59,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1181,"record_offset":128176,"parent_record_offset":128148,"ancestor_record_offset":115664,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1181,"record_offset":128204,"parent_record_offset":128176,"ancestor_record_offset":115664,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":119084,"parent_record_offset":118824,"ancestor_record_offset":115664,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":121056,"parent_record_offset":121032,"ancestor_record_offset":115664,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":121140,"parent_record_offset":121116,"ancestor_record_offset":115664,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":121080,"parent_record_offset":121056,"ancestor_record_offset":115664,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":121616,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":128252,"parent_record_offset":128228,"ancestor_record_offset":115664,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1181,"record_offset":118716,"parent_record_offset":118684,"ancestor_record_offset":115664,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1181,"record_offset":128228,"parent_record_offset":128204,"ancestor_record_offset":115664,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1181,"record_offset":118912,"parent_record_offset":118884,"ancestor_record_offset":115664,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1181,"record_offset":117548,"parent_record_offset":117520,"ancestor_record_offset":115664,"inlinee":5005,"source_line":2021,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":1181,"record_offset":119996,"parent_record_offset":119964,"ancestor_record_offset":115664,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":121288,"parent_record_offset":121260,"ancestor_record_offset":115664,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":121260,"parent_record_offset":121232,"ancestor_record_offset":115664,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1181,"record_offset":119232,"parent_record_offset":119208,"ancestor_record_offset":115664,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":116596,"parent_record_offset":116568,"ancestor_record_offset":115664,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":121792,"parent_record_offset":120696,"ancestor_record_offset":115664,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1181,"record_offset":120080,"parent_record_offset":120052,"ancestor_record_offset":115664,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1181,"record_offset":128396,"parent_record_offset":127968,"ancestor_record_offset":115664,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1181,"record_offset":116844,"parent_record_offset":116820,"ancestor_record_offset":115664,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":118420,"parent_record_offset":116120,"ancestor_record_offset":115664,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":117096,"parent_record_offset":116888,"ancestor_record_offset":115664,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":121408,"parent_record_offset":121384,"ancestor_record_offset":115664,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":116820,"parent_record_offset":116796,"ancestor_record_offset":115664,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":120736,"parent_record_offset":120696,"ancestor_record_offset":115664,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":116796,"parent_record_offset":116772,"ancestor_record_offset":115664,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1181,"record_offset":119108,"parent_record_offset":119084,"ancestor_record_offset":115664,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1181,"record_offset":119132,"parent_record_offset":119108,"ancestor_record_offset":115664,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1181,"record_offset":119428,"parent_record_offset":119400,"ancestor_record_offset":115664,"inlinee":5362,"source_line":2612,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1181,"record_offset":118860,"parent_record_offset":118824,"ancestor_record_offset":115664,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1181,"record_offset":120428,"parent_record_offset":120400,"ancestor_record_offset":115664,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1181,"record_offset":118968,"parent_record_offset":118944,"ancestor_record_offset":115664,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1181,"record_offset":118944,"parent_record_offset":118884,"ancestor_record_offset":115664,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1181,"record_offset":118884,"parent_record_offset":118860,"ancestor_record_offset":115664,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1181,"record_offset":116748,"parent_record_offset":116708,"ancestor_record_offset":115664,"inlinee":6415,"source_line":137,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::make_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1181,"record_offset":116772,"parent_record_offset":116748,"ancestor_record_offset":115664,"inlinee":6416,"source_line":88,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":1181,"record_offset":119008,"parent_record_offset":118824,"ancestor_record_offset":115664,"inlinee":6428,"source_line":26,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\memchr.rs
//     evidence={"module":1181,"record_offset":119040,"parent_record_offset":119008,"ancestor_record_offset":115664,"inlinee":6429,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":119964,"parent_record_offset":119936,"ancestor_record_offset":115664,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::match_indices
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":116488,"parent_record_offset":116120,"ancestor_record_offset":115664,"inlinee":16883,"source_line":2061,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::strip_suffix
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1181,"record_offset":119316,"parent_record_offset":119280,"ancestor_record_offset":115664,"inlinee":25083,"source_line":2425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$5::call
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1181,"record_offset":119280,"parent_record_offset":119256,"ancestor_record_offset":115664,"inlinee":32858,"source_line":97,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$6::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\internal_macros.rs
//     evidence={"module":1181,"record_offset":119256,"parent_record_offset":119232,"ancestor_record_offset":115664,"inlinee":32856,"source_line":104,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::MatchIndicesInternal::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":116568,"parent_record_offset":116544,"ancestor_record_offset":115664,"inlinee":16887,"source_line":1062,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::get_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":119172,"parent_record_offset":118780,"ancestor_record_offset":115664,"inlinee":17880,"source_line":645,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::SplitInternal::next_inclusive
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":118780,"parent_record_offset":118748,"ancestor_record_offset":115664,"inlinee":32854,"source_line":678,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$30::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":118684,"parent_record_offset":118652,"ancestor_record_offset":115664,"inlinee":32851,"source_line":1172,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$45::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":118748,"parent_record_offset":118716,"ancestor_record_offset":115664,"inlinee":32853,"source_line":1425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$91::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":1181,"record_offset":116544,"parent_record_offset":116120,"ancestor_record_offset":115664,"inlinee":16885,"source_line":514,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$1::next_match
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":118824,"parent_record_offset":118780,"ancestor_record_offset":115664,"inlinee":17879,"source_line":432,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::into_searcher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":116512,"parent_record_offset":116488,"ancestor_record_offset":115664,"inlinee":4997,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":120400,"parent_record_offset":120372,"ancestor_record_offset":115664,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":119400,"parent_record_offset":119372,"ancestor_record_offset":115664,"inlinee":8777,"source_line":1024,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":120372,"parent_record_offset":120344,"ancestor_record_offset":115664,"inlinee":9146,"source_line":1013,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":119372,"parent_record_offset":119344,"ancestor_record_offset":115664,"inlinee":25084,"source_line":1033,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::strip_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1181,"record_offset":119344,"parent_record_offset":119316,"ancestor_record_offset":115664,"inlinee":32859,"source_line":611,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":120216,"parent_record_offset":120184,"ancestor_record_offset":115664,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":120184,"parent_record_offset":120156,"ancestor_record_offset":115664,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":119936,"parent_record_offset":119904,"ancestor_record_offset":115664,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":120028,"parent_record_offset":119936,"ancestor_record_offset":115664,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":119904,"parent_record_offset":119876,"ancestor_record_offset":115664,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":1181,"record_offset":120052,"parent_record_offset":120028,"ancestor_record_offset":115664,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":116040,"parent_record_offset":116016,"ancestor_record_offset":115664,"inlinee":6050,"source_line":1566,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":116064,"parent_record_offset":116040,"ancestor_record_offset":115664,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":121572,"parent_record_offset":121548,"ancestor_record_offset":115664,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":121200,"parent_record_offset":121172,"ancestor_record_offset":115664,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":121232,"parent_record_offset":121200,"ancestor_record_offset":115664,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":121172,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":121032,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":121644,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":121116,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":121004,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":121524,"parent_record_offset":121500,"ancestor_record_offset":115664,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":125512,"parent_record_offset":125476,"ancestor_record_offset":115664,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":125536,"parent_record_offset":125512,"ancestor_record_offset":115664,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":120812,"parent_record_offset":120788,"ancestor_record_offset":115664,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":120788,"parent_record_offset":120764,"ancestor_record_offset":115664,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":121500,"parent_record_offset":121332,"ancestor_record_offset":115664,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":120844,"parent_record_offset":120812,"ancestor_record_offset":115664,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":121440,"parent_record_offset":121356,"ancestor_record_offset":115664,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":121384,"parent_record_offset":121356,"ancestor_record_offset":115664,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":121736,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":121356,"parent_record_offset":121332,"ancestor_record_offset":115664,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":120896,"parent_record_offset":120872,"ancestor_record_offset":115664,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":125596,"parent_record_offset":125536,"ancestor_record_offset":115664,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":120872,"parent_record_offset":120812,"ancestor_record_offset":115664,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":120920,"parent_record_offset":120896,"ancestor_record_offset":115664,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":120976,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":121332,"parent_record_offset":120920,"ancestor_record_offset":115664,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::imp::OnceCell::is_initialized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs
//     evidence={"module":1181,"record_offset":116016,"parent_record_offset":115992,"ancestor_record_offset":115664,"inlinee":6048,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::Lazy::force
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1181,"record_offset":115904,"parent_record_offset":115876,"ancestor_record_offset":115664,"inlinee":6042,"source_line":1308,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1181,"record_offset":115992,"parent_record_offset":115960,"ancestor_record_offset":115664,"inlinee":6046,"source_line":963,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1181,"record_offset":115932,"parent_record_offset":115904,"ancestor_record_offset":115664,"inlinee":6044,"source_line":1115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] once_cell::sync::OnceCell::get_or_try_init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"once_cell","version":"1.21.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/lib.rs
//     evidence={"module":1181,"record_offset":115960,"parent_record_offset":115932,"ancestor_record_offset":115664,"inlinee":6045,"source_line":1152,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":125476,"parent_record_offset":125444,"ancestor_record_offset":115664,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":120764,"parent_record_offset":120696,"ancestor_record_offset":115664,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":125444,"parent_record_offset":125412,"ancestor_record_offset":115664,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":123308,"parent_record_offset":123248,"ancestor_record_offset":115664,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":123248,"parent_record_offset":123224,"ancestor_record_offset":115664,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":123452,"parent_record_offset":123428,"ancestor_record_offset":115664,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":125412,"parent_record_offset":125380,"ancestor_record_offset":115664,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":123704,"parent_record_offset":123664,"ancestor_record_offset":115664,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":122968,"parent_record_offset":122944,"ancestor_record_offset":115664,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":127756,"parent_record_offset":127732,"ancestor_record_offset":115664,"inlinee":14896,"source_line":603,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":120632,"parent_record_offset":120608,"ancestor_record_offset":115664,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":120660,"parent_record_offset":120608,"ancestor_record_offset":115664,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":120608,"parent_record_offset":120584,"ancestor_record_offset":115664,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123428,"parent_record_offset":123400,"ancestor_record_offset":115664,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label_with_id::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":123364,"parent_record_offset":123248,"ancestor_record_offset":115664,"inlinee":61824,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF189
// 首条位置: {"module":1181,"inlinee":61833}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1181,"record_offset":138392,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1181,"record_offset":141312,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":138096,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":138332,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":137956,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":137728,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1181,"record_offset":138128,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1181,"record_offset":138012,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":137640,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":139732,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":19831,"source_line":333,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":138476,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":13179,"source_line":230,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":136464,"parent_record_offset":136336,"ancestor_record_offset":136336,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":137904,"parent_record_offset":137880,"ancestor_record_offset":136336,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":137880,"parent_record_offset":137824,"ancestor_record_offset":136336,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1181,"record_offset":138440,"parent_record_offset":138416,"ancestor_record_offset":136336,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":137852,"parent_record_offset":137824,"ancestor_record_offset":136336,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":137824,"parent_record_offset":137800,"ancestor_record_offset":136336,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":137800,"parent_record_offset":137776,"ancestor_record_offset":136336,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1181,"record_offset":141340,"parent_record_offset":141312,"ancestor_record_offset":136336,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":137384,"parent_record_offset":137360,"ancestor_record_offset":136336,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":136892,"parent_record_offset":136868,"ancestor_record_offset":136336,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":136976,"parent_record_offset":136952,"ancestor_record_offset":136336,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":136916,"parent_record_offset":136892,"ancestor_record_offset":136336,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":137452,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":137124,"parent_record_offset":137096,"ancestor_record_offset":136336,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":137096,"parent_record_offset":137068,"ancestor_record_offset":136336,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":137980,"parent_record_offset":137956,"ancestor_record_offset":136336,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":138416,"parent_record_offset":138392,"ancestor_record_offset":136336,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":139612,"parent_record_offset":138512,"ancestor_record_offset":136336,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1181,"record_offset":138184,"parent_record_offset":138152,"ancestor_record_offset":136336,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1181,"record_offset":138212,"parent_record_offset":138152,"ancestor_record_offset":136336,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":137752,"parent_record_offset":137728,"ancestor_record_offset":136336,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":137244,"parent_record_offset":137220,"ancestor_record_offset":136336,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":138548,"parent_record_offset":138512,"ancestor_record_offset":136336,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":136520,"parent_record_offset":136492,"ancestor_record_offset":136336,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":138036,"parent_record_offset":138012,"ancestor_record_offset":136336,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":138060,"parent_record_offset":138036,"ancestor_record_offset":136336,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":137408,"parent_record_offset":137384,"ancestor_record_offset":136336,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":137036,"parent_record_offset":137008,"ancestor_record_offset":136336,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":137068,"parent_record_offset":137036,"ancestor_record_offset":136336,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":137008,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":136868,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":137480,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":136952,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":136840,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":137360,"parent_record_offset":137336,"ancestor_record_offset":136336,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":136620,"parent_record_offset":136580,"ancestor_record_offset":136336,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":136648,"parent_record_offset":136620,"ancestor_record_offset":136336,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":138632,"parent_record_offset":138604,"ancestor_record_offset":136336,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":138604,"parent_record_offset":138576,"ancestor_record_offset":136336,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":137336,"parent_record_offset":137168,"ancestor_record_offset":136336,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":136680,"parent_record_offset":136648,"ancestor_record_offset":136336,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":137276,"parent_record_offset":137192,"ancestor_record_offset":136336,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":137220,"parent_record_offset":137192,"ancestor_record_offset":136336,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":137572,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":137192,"parent_record_offset":137168,"ancestor_record_offset":136336,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":136732,"parent_record_offset":136708,"ancestor_record_offset":136336,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":136708,"parent_record_offset":136648,"ancestor_record_offset":136336,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":138692,"parent_record_offset":138632,"ancestor_record_offset":136336,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":136756,"parent_record_offset":136732,"ancestor_record_offset":136336,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":136812,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":137168,"parent_record_offset":136756,"ancestor_record_offset":136336,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1181,"record_offset":138240,"parent_record_offset":138152,"ancestor_record_offset":136336,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1181,"record_offset":138152,"parent_record_offset":138128,"ancestor_record_offset":136336,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1181,"record_offset":138264,"parent_record_offset":138240,"ancestor_record_offset":136336,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":136580,"parent_record_offset":136548,"ancestor_record_offset":136336,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":138576,"parent_record_offset":138512,"ancestor_record_offset":136336,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":136548,"parent_record_offset":136492,"ancestor_record_offset":136336,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":139668,"parent_record_offset":139644,"ancestor_record_offset":136336,"inlinee":20045,"source_line":305,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":139784,"parent_record_offset":139760,"ancestor_record_offset":136336,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":136492,"parent_record_offset":136464,"ancestor_record_offset":136336,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":138512,"parent_record_offset":138476,"ancestor_record_offset":136336,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":139692,"parent_record_offset":139668,"ancestor_record_offset":136336,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":137668,"parent_record_offset":137640,"ancestor_record_offset":136336,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":137692,"parent_record_offset":137668,"ancestor_record_offset":136336,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":139760,"parent_record_offset":139732,"ancestor_record_offset":136336,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::ui::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":1181,"record_offset":139644,"parent_record_offset":138476,"ancestor_record_offset":136336,"inlinee":61835,"source_line":222,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ui::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF18B
// 首条位置: {"module":1181,"inlinee":61835}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":139668,"parent_record_offset":139644,"ancestor_record_offset":139644,"inlinee":20045,"source_line":305,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":139692,"parent_record_offset":139668,"ancestor_record_offset":139644,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server
// 记录形式: compiled_procedure; instances=1
// 源码行: 238
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":19808,"rva":31437840,"va":5400146960}
// 编译实例 RVAs: 0x1dfb410
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14002e900
//     owner=unresolved_namespace; count=1
//     target_aliases=130 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31247814","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e83a3223fe","size":5,"soft_fail":false,"target_va":5368899840,"va":5400147649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25065889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e85f8681fe","size":5,"soft_fail":false,"target_va":5375081488,"va":5400147372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25065767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e8d98681fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400147266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa1950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15047995","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e8c5621aff","size":5,"soft_fail":false,"target_va":5385099600,"va":5400147590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14234d690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5579323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e83b225500","size":5,"soft_fail":false,"target_va":5405726352,"va":5400147024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428e5920
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11445289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e829a4ae00","size":5,"soft_fail":false,"target_va":5411592480,"va":5400147186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17306974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e8a2eaf7fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5400147177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11393483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e8cbd9ad00","size":5,"soft_fail":false,"target_va":5411540963,"va":5400147475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::spawn
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5579669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400146960,"raw_hex":"e895235500","size":5,"soft_fail":false,"target_va":5405727024,"va":5400147350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5400146960,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5400147237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2755,"record_offset":20492,"parent_record_offset":19808,"ancestor_record_offset":19808,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2755,"record_offset":19964,"parent_record_offset":19808,"ancestor_record_offset":19808,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2755,"record_offset":20464,"parent_record_offset":19808,"ancestor_record_offset":19808,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20808,"parent_record_offset":20784,"ancestor_record_offset":19808,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20832,"parent_record_offset":20808,"ancestor_record_offset":19808,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":19808,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20784,"parent_record_offset":20756,"ancestor_record_offset":19808,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20248,"parent_record_offset":20160,"ancestor_record_offset":19808,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2755,"record_offset":20160,"parent_record_offset":20132,"ancestor_record_offset":19808,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2755,"record_offset":20048,"parent_record_offset":20008,"ancestor_record_offset":19808,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2755,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":19808,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":20696,"parent_record_offset":20668,"ancestor_record_offset":19808,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":20756,"parent_record_offset":20724,"ancestor_record_offset":19808,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":20724,"parent_record_offset":20696,"ancestor_record_offset":19808,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":20632,"parent_record_offset":20604,"ancestor_record_offset":19808,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":20604,"parent_record_offset":20576,"ancestor_record_offset":19808,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":20548,"parent_record_offset":20520,"ancestor_record_offset":19808,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":20576,"parent_record_offset":20548,"ancestor_record_offset":19808,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2755,"record_offset":20520,"parent_record_offset":20492,"ancestor_record_offset":19808,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":20668,"parent_record_offset":20632,"ancestor_record_offset":19808,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2755,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":19808,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2755,"record_offset":20908,"parent_record_offset":20884,"ancestor_record_offset":19808,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2755,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":19808,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2755,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":19808,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2755,"record_offset":20884,"parent_record_offset":20632,"ancestor_record_offset":19808,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":20132,"parent_record_offset":20008,"ancestor_record_offset":19808,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":2755,"record_offset":20008,"parent_record_offset":19964,"ancestor_record_offset":19808,"inlinee":99689,"source_line":242,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 242
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18569
// 首条位置: {"module":2755,"inlinee":99689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2755,"record_offset":20048,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":20132,"parent_record_offset":20008,"ancestor_record_offset":20008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20272,"parent_record_offset":20248,"ancestor_record_offset":20008,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":20248,"parent_record_offset":20160,"ancestor_record_offset":20008,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":2755,"record_offset":20160,"parent_record_offset":20132,"ancestor_record_offset":20008,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2755,"record_offset":20096,"parent_record_offset":20072,"ancestor_record_offset":20008,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2755,"record_offset":20192,"parent_record_offset":20160,"ancestor_record_offset":20008,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2755,"record_offset":20216,"parent_record_offset":20192,"ancestor_record_offset":20008,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2755,"record_offset":20072,"parent_record_offset":20048,"ancestor_record_offset":20008,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20853
// 首条位置: {"module":4531,"inlinee":133203}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":4531,"record_offset":968,"parent_record_offset":940,"ancestor_record_offset":940,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4531,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":940,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4531,"record_offset":1520,"parent_record_offset":1436,"ancestor_record_offset":940,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4531,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":940,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4531,"record_offset":1040,"parent_record_offset":1000,"ancestor_record_offset":940,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4531,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":940,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4531,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":940,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4531,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":940,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4531,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":940,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4531,"record_offset":1124,"parent_record_offset":1000,"ancestor_record_offset":940,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":940,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":940,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":940,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4531,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":940,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4531,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":940,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/credits.rs
//     evidence={"module":4531,"record_offset":1000,"parent_record_offset":968,"ancestor_record_offset":940,"inlinee":133205,"source_line":250,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 250
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x20855
// 首条位置: {"module":4531,"inlinee":133205}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4531,"record_offset":1040,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4531,"record_offset":1124,"parent_record_offset":1000,"ancestor_record_offset":1000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4531,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":1000,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":4531,"record_offset":1520,"parent_record_offset":1436,"ancestor_record_offset":1000,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":4531,"record_offset":1436,"parent_record_offset":1408,"ancestor_record_offset":1000,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":4531,"record_offset":1088,"parent_record_offset":1064,"ancestor_record_offset":1000,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":4531,"record_offset":1464,"parent_record_offset":1436,"ancestor_record_offset":1000,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":4531,"record_offset":1488,"parent_record_offset":1464,"ancestor_record_offset":1000,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":4531,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":1000,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":4531,"record_offset":1152,"parent_record_offset":1124,"ancestor_record_offset":1000,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1236,"parent_record_offset":1208,"ancestor_record_offset":1000,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":1000,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4531,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":1000,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4531,"record_offset":1208,"parent_record_offset":1180,"ancestor_record_offset":1000,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":4531,"record_offset":1180,"parent_record_offset":1152,"ancestor_record_offset":1000,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/04531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$3::scroll<u64>
// 记录形式: compiled_procedure; instances=1
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1181,"record_offset":204252,"rva":16385264,"va":5385094384}
// 编译实例 RVAs: 0xfa04f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=1
//     target_aliases=7 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10418195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e8ed0761ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385094526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10366002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e8ced361ff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385094461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10364426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e8f6d961ff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385094437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9cc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e8c1c6ffff","size":5,"soft_fail":false,"target_va":5385079840,"va":5385094490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26446968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e8788c9301","size":5,"soft_fail":false,"target_va":5411541776,"va":5385094803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9544613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094384,"raw_hex":"e85b5c6eff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385094480,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":204384,"parent_record_offset":204252,"ancestor_record_offset":204252,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":204496,"parent_record_offset":204252,"ancestor_record_offset":204252,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":205352,"parent_record_offset":205328,"ancestor_record_offset":204252,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":204860,"parent_record_offset":204836,"ancestor_record_offset":204252,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":204944,"parent_record_offset":204920,"ancestor_record_offset":204252,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":204884,"parent_record_offset":204860,"ancestor_record_offset":204252,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":205420,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":205092,"parent_record_offset":205064,"ancestor_record_offset":204252,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":205064,"parent_record_offset":205036,"ancestor_record_offset":204252,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":205596,"parent_record_offset":204496,"ancestor_record_offset":204252,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":205212,"parent_record_offset":205188,"ancestor_record_offset":204252,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":204532,"parent_record_offset":204496,"ancestor_record_offset":204252,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":205376,"parent_record_offset":205352,"ancestor_record_offset":204252,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":205004,"parent_record_offset":204976,"ancestor_record_offset":204252,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":205036,"parent_record_offset":205004,"ancestor_record_offset":204252,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":204976,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":204836,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":205448,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":204920,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":204808,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":205328,"parent_record_offset":205304,"ancestor_record_offset":204252,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":204616,"parent_record_offset":204588,"ancestor_record_offset":204252,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":204588,"parent_record_offset":204560,"ancestor_record_offset":204252,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":205304,"parent_record_offset":205136,"ancestor_record_offset":204252,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":204648,"parent_record_offset":204616,"ancestor_record_offset":204252,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":205244,"parent_record_offset":205160,"ancestor_record_offset":204252,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":205188,"parent_record_offset":205160,"ancestor_record_offset":204252,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":205540,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":205160,"parent_record_offset":205136,"ancestor_record_offset":204252,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":204700,"parent_record_offset":204676,"ancestor_record_offset":204252,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":204676,"parent_record_offset":204616,"ancestor_record_offset":204252,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":204724,"parent_record_offset":204700,"ancestor_record_offset":204252,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":204780,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":205136,"parent_record_offset":204724,"ancestor_record_offset":204252,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":204560,"parent_record_offset":204496,"ancestor_record_offset":204252,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":204436,"parent_record_offset":204384,"ancestor_record_offset":204252,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":204412,"parent_record_offset":204384,"ancestor_record_offset":204252,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":204460,"parent_record_offset":204436,"ancestor_record_offset":204252,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$7::default
// 记录形式: inlinee_declaration; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1856B
// 首条位置: {"module":2755,"inlinee":99691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$0::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 298
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF195
// 首条位置: {"module":1181,"inlinee":61845}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":155060,"parent_record_offset":155036,"ancestor_record_offset":155036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":155108,"parent_record_offset":155084,"ancestor_record_offset":155036,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":155084,"parent_record_offset":155060,"ancestor_record_offset":155036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":155144,"parent_record_offset":155108,"ancestor_record_offset":155036,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":155168,"parent_record_offset":155144,"ancestor_record_offset":155036,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":155200,"parent_record_offset":155108,"ancestor_record_offset":155036,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::background_color
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 304
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18563
// 首条位置: {"module":2755,"inlinee":99683}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":7204,"parent_record_offset":7176,"ancestor_record_offset":7176,"inlinee":99684,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":7176,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::max_scroll_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 309
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18565
// 首条位置: {"module":2755,"inlinee":99685}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":7292,"parent_record_offset":7264,"ancestor_record_offset":7264,"inlinee":99684,"source_line":210,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":7264,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::ScrollView::offset
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 314
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18566
// 首条位置: {"module":2755,"inlinee":99686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::credits::ScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2755,"record_offset":7376,"parent_record_offset":7352,"ancestor_record_offset":7352,"inlinee":99687,"source_line":205,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 322
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":20968,"rva":31438544,"va":5400147664}
// 编译实例 RVAs: 0x1dfb6d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::prepare_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 327
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":21084,"rva":31438592,"va":5400147712}
// 编译实例 RVAs: 0x1dfb700
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18264319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147712,"raw_hex":"e8014fe9fe","size":5,"soft_fail":false,"target_va":5381883504,"va":5400147818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2755,"record_offset":21312,"parent_record_offset":21084,"ancestor_record_offset":21084,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":21228,"parent_record_offset":21084,"ancestor_record_offset":21084,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2755,"record_offset":21256,"parent_record_offset":21084,"ancestor_record_offset":21084,"inlinee":14896,"source_line":603,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$20::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":21340,"parent_record_offset":21084,"ancestor_record_offset":21084,"inlinee":19710,"source_line":1583,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::credits::impl$2::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 383
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2755,"record_offset":21372,"rva":31438848,"va":5400147968}
// 编译实例 RVAs: 0x1dfb800
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f11f0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25208866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8de577ffe","size":5,"soft_fail":false,"target_va":5374939632,"va":5400148493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25066683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8458381fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400148182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a82b20
//     owner=unresolved_namespace; count=1
//     target_aliases=20 candidates; complete list: credits.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20417822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8e272c8fe","size":5,"soft_fail":false,"target_va":5379730208,"va":5400148025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextRenderer::rect
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19317847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8a93bd9fe","size":5,"soft_fail":false,"target_va":5380830480,"va":5400148322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19305771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8d56ad9fe","size":5,"soft_fail":false,"target_va":5380842304,"va":5400148070,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::RawShapeStream::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19305749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e8eb6ad9fe","size":5,"soft_fail":false,"target_va":5380842272,"va":5400148016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::shape_stream::ShapeStream::fill_color
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19306398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400147968,"raw_hex":"e86268d9fe","size":5,"soft_fail":false,"target_va":5380841648,"va":5400148041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":21512,"parent_record_offset":21372,"ancestor_record_offset":21372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":22136,"parent_record_offset":21372,"ancestor_record_offset":21372,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":22312,"parent_record_offset":21372,"ancestor_record_offset":21372,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":22224,"parent_record_offset":21372,"ancestor_record_offset":21372,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::RenderContext::draw_rect
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2755,"record_offset":22400,"parent_record_offset":21372,"ancestor_record_offset":21372,"inlinee":19795,"source_line":1654,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":21852,"parent_record_offset":21828,"ancestor_record_offset":21372,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2755,"record_offset":21828,"parent_record_offset":21772,"ancestor_record_offset":21372,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":21800,"parent_record_offset":21772,"ancestor_record_offset":21372,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":21772,"parent_record_offset":21748,"ancestor_record_offset":21372,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2755,"record_offset":21748,"parent_record_offset":21724,"ancestor_record_offset":21372,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2755,"record_offset":21560,"parent_record_offset":21536,"ancestor_record_offset":21372,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2755,"record_offset":22432,"parent_record_offset":22400,"ancestor_record_offset":21372,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2755,"record_offset":22028,"parent_record_offset":22000,"ancestor_record_offset":21372,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2755,"record_offset":22340,"parent_record_offset":22312,"ancestor_record_offset":21372,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2755,"record_offset":22164,"parent_record_offset":22136,"ancestor_record_offset":21372,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2755,"record_offset":22252,"parent_record_offset":22224,"ancestor_record_offset":21372,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2755,"record_offset":21536,"parent_record_offset":21512,"ancestor_record_offset":21372,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":22488,"parent_record_offset":22400,"ancestor_record_offset":21372,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02755.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$95::from
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2755,"record_offset":22456,"parent_record_offset":22432,"ancestor_record_offset":21372,"inlinee":4583,"source_line":1811,"archive":"_pdb/B/modules/02755.jsonl.gz"}
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
// 完整结构及原始类型记录: credits.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::CRATE_CREDITS
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["toolbar::menu::credits::CRATE_CREDITS::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::credits::ScrollContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::process_input_action<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::process_input_action", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::pre_update<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::NodeContent::pre_update", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any_mut<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any_mut", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::type_name<toolbar::menu::credits::ScrollContent>", "constructor": "toolbar::mei::core::impl$21::type_name", "arguments": ["toolbar::menu::credits::ScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ScrollView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::credits::ThirdPartyLicenseServer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>", "constructor": "core::option::Option", "arguments": ["toolbar::menu::credits::ThirdPartyLicenseServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >", "constructor": "enum2$", "arguments": ["core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::credits::ThirdPartyLicenseServer>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::credits::ThirdPartyLicenseServer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)", "void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: credits.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-camera/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "toolbar::menu::credits::ThirdPartyLicenseServer", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: credits.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS> > >", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >,bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput> > >", "bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::widget_params::UiNavState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::steam::SteamInput>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::ActiveInputModeTracker>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "country_core::systems::steam::SteamOverlayOpenResources", "bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::system::system_param::Local<f32>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<f32>

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0
// 信息 ID: 70790d0318194ff85cde7791
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::CRATE_CREDITS::closure_env$0 <- core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2755, inline_record=836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::CRATE_CREDITS::closure$0::closure$0
// 信息 ID: 08f0d67a53bf0b9a98f24cdb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::CRATE_CREDITS", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<toolbar::menu::credits::CRATE_CREDITS::closure_env$0,tuple$<> >; module=2755, inline_record=1796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::background_color
// 信息 ID: bc144e4c21c45b6bfa85ee26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2755, inline_record=7176; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::max_scroll_pos
// 信息 ID: 303cc8e87783a77f6711cd33
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2755, inline_record=7264; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ScrollView::offset
// 信息 ID: 7967429fec7a2cbaaaf4de2a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::ui; module=2755, inline_record=7352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$0::create_node_content
// 信息 ID: 401172ab740ac8fe7bf69e04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::credits::ScrollContent>; module=1181, inline_record=155036; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::on_input
// 信息 ID: 656733028868603b32cd6ca6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::prepare_layout
// 信息 ID: cbf75bba52dfed4367e771a9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$2::render
// 信息 ID: 8e0a24663dc6460407ab7654
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$3::scroll<u64>
// 信息 ID: e45888eec51825162fecf12e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] toolbar::menu::credits::impl$3::scroll -> ["u64"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::credits::ui @ 0x1dfa3d4 -> 0xfa04f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::impl$7::default
// 信息 ID: 9cae1d68a9783275c79e793f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server
// 信息 ID: b67035fe143405737ee54b43
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1> @ 0xf9bbb6 -> 0x1dfb410 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$0
// 信息 ID: c968e1fbeca4cdaedca59345
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::credits::spawn_third_party_license_http_server; module=2755, inline_record=20008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1
// 信息 ID: 00e2865ec6059464f621b902
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::thread::spawn<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ops::function::FnOnce::call_once<std::thread::impl$0::spawn_unchecked_::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- core::ptr::drop_in_place<std::thread::impl$0::spawn_unchecked_::closure_env$1<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::sys::backtrace::__rust_begin_short_backtrace<std::thread::impl$0::spawn_unchecked_::closure$1::closure$0::closure_env$0<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >,tuple$<> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1 <- std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=4531, inline_record=940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::spawn_third_party_license_http_server::closure$1::closure$0
// 信息 ID: 0e7d21f2d0b289d7da09eac2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::spawn_third_party_license_http_server", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::sys::backtrace::__rust_begin_short_backtrace<toolbar::menu::credits::spawn_third_party_license_http_server::closure_env$1,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=4531, inline_record=1000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui
// 信息 ID: 1a68d07b74738b6d9dc8a0d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fc8d029f3a5cd8d5737278ec: void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > @ 0xff3237 -> 0x1dfa070 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,country_core::systems::steam::SteamOverlayOpenResources,bevy_ecs::system::system_param::Local<enum2$<core::option::Option<toolbar::menu::credits::ThirdPartyLicenseServer> > >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::system::system_param::Local<f32>)> > > @ 0x1e192ff -> 0x1dfa070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$0
// 信息 ID: b1d54d08b35feba122d9e1c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$0>; module=1181, inline_record=115664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$1
// 信息 ID: e273f4b2f1c0f8a83a0f0c6f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::credits::ui::closure_env$1 <- toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1181, inline_record=136336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::credits::ui::closure$1::closure$0
// 信息 ID: 96e0022aa1d6e4cf177e9677
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::credits::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::credits::ui::closure_env$1>; module=1181, inline_record=139644; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
