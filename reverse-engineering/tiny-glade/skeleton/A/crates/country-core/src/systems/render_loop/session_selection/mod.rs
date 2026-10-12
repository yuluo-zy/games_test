// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/country-core/src/systems/render_loop/session_selection/mod.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 2e8714dd91e770330348f53c7f7549582c3a4c74a76e0391693e755608087c97
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 433, 1308, 1321, 1330
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"PDB_registry_path_only","version":"1.0.86"}]
// use bevy_app; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"PDB_registry_path_only","version":"0.14.5"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"PDB_registry_path_only","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":433,"record_offset":2736,"rva":6075040,"va":5374784160}
// 编译实例 RVAs: 0x5cb2a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_app::app::App::insert_resource
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8319320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784160,"raw_hex":"e858f17e00","size":5,"soft_fail":false,"target_va":5383103536,"va":5374784211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_app::sub_app::SubApp::add_systems
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8259504","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784160,"raw_hex":"e8b0077e00","size":5,"soft_fail":false,"target_va":5383043680,"va":5374784171,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_app::app::App::add_systems
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_app/src/app.rs
//     evidence={"module":433,"record_offset":2892,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":9705,"source_line":284,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::session_selection::impl$2::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":2920,"parent_record_offset":2736,"ancestor_record_offset":2736,"inlinee":24984,"source_line":61,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$2::default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6198
// 首条位置: {"module":433,"inlinee":24984}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::ThumbnailTextures::extract_thumbnails_to_display
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x619A
// 首条位置: {"module":433,"inlinee":24986}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":433,"record_offset":25540,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":8697,"source_line":902,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":433,"record_offset":23616,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$24::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":24408,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":24636,"source_line":2430,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":23864,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":21268,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":25652,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":25680,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":433,"record_offset":16056,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":26336,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":21532,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":21560,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":21380,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":23820,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":21092,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":21204,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":21408,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::session_selection::ThumbnailTextures::assign_pool_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":16112,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":24988,"source_line":147,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::session_selection::ThumbnailTextures::find_backed_thumbnail_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":15876,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":24987,"source_line":123,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::session_selection::refresh_screenshot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":21596,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":24994,"source_line":233,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":433,"record_offset":25708,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":433,"record_offset":25568,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":433,"record_offset":25484,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":433,"record_offset":23700,"parent_record_offset":15724,"ancestor_record_offset":15724,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":15724,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":15724,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":15724,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24168,"parent_record_offset":24140,"ancestor_record_offset":15724,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17460,"parent_record_offset":17404,"ancestor_record_offset":15724,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":433,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":15724,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":25148,"parent_record_offset":25124,"ancestor_record_offset":15724,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17432,"parent_record_offset":17404,"ancestor_record_offset":15724,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":25124,"parent_record_offset":25096,"ancestor_record_offset":15724,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24644,"parent_record_offset":24608,"ancestor_record_offset":15724,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":15724,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24140,"parent_record_offset":24112,"ancestor_record_offset":15724,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":15724,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24288,"parent_record_offset":24112,"ancestor_record_offset":15724,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":15724,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":24048,"parent_record_offset":24020,"ancestor_record_offset":15724,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":24020,"parent_record_offset":23992,"ancestor_record_offset":15724,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23964,"parent_record_offset":23920,"ancestor_record_offset":15724,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":15724,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23920,"parent_record_offset":23892,"ancestor_record_offset":15724,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":433,"record_offset":23892,"parent_record_offset":23864,"ancestor_record_offset":15724,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":24444,"parent_record_offset":24408,"ancestor_record_offset":15724,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":15724,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24608,"parent_record_offset":24516,"ancestor_record_offset":15724,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24468,"parent_record_offset":24444,"ancestor_record_offset":15724,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24676,"parent_record_offset":24516,"ancestor_record_offset":15724,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24552,"parent_record_offset":24516,"ancestor_record_offset":15724,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24084,"parent_record_offset":24048,"ancestor_record_offset":15724,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":16364,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":15724,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":433,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":15724,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":433,"record_offset":22028,"parent_record_offset":21984,"ancestor_record_offset":15724,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21984,"parent_record_offset":21952,"ancestor_record_offset":15724,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21952,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":433,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":15724,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":433,"record_offset":18480,"parent_record_offset":18380,"ancestor_record_offset":15724,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22928,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22196,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":18568,"parent_record_offset":18336,"ancestor_record_offset":15724,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":433,"record_offset":22300,"parent_record_offset":22276,"ancestor_record_offset":15724,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15936,"parent_record_offset":15876,"ancestor_record_offset":15724,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15996,"parent_record_offset":15876,"ancestor_record_offset":15724,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":16024,"parent_record_offset":15876,"ancestor_record_offset":15724,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":22956,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":433,"record_offset":24348,"parent_record_offset":24324,"ancestor_record_offset":15724,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":433,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":15724,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":433,"record_offset":16928,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":15724,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":15724,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16804,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":15724,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":15724,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":433,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":15724,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":433,"record_offset":26384,"parent_record_offset":26360,"ancestor_record_offset":15724,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":18652,"parent_record_offset":18336,"ancestor_record_offset":15724,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":16608,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18912,"parent_record_offset":17968,"ancestor_record_offset":15724,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20760,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20732,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18000,"parent_record_offset":17968,"ancestor_record_offset":15724,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":15724,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":16976,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":15724,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":15724,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":24324,"parent_record_offset":24048,"ancestor_record_offset":15724,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":15724,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=63
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":17244,"parent_record_offset":17032,"ancestor_record_offset":15724,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":433,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":15724,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":15724,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":15724,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16328,"parent_record_offset":16280,"ancestor_record_offset":15724,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":24244,"parent_record_offset":24216,"ancestor_record_offset":15724,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":21908,"parent_record_offset":21880,"ancestor_record_offset":15724,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22500,"parent_record_offset":22252,"ancestor_record_offset":15724,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22536,"parent_record_offset":22252,"ancestor_record_offset":15724,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":15724,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":15724,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":16524,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":15724,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16392,"parent_record_offset":16364,"ancestor_record_offset":15724,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16456,"parent_record_offset":16420,"ancestor_record_offset":15724,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16420,"parent_record_offset":16392,"ancestor_record_offset":15724,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":15724,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":15724,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":21472,"parent_record_offset":21408,"ancestor_record_offset":15724,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":15724,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":15724,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":20668,"parent_record_offset":20640,"ancestor_record_offset":15724,"inlinee":6147,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16280,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::position
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":17848,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":9708,"source_line":331,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":25976,"parent_record_offset":25800,"ancestor_record_offset":15724,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":15724,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26064,"parent_record_offset":26036,"ancestor_record_offset":15724,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":15724,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":15724,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26000,"parent_record_offset":25976,"ancestor_record_offset":15724,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":15724,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SavedSession::screenshot_path
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":433,"record_offset":17032,"parent_record_offset":16112,"ancestor_record_offset":15724,"inlinee":23028,"source_line":322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":17968,"parent_record_offset":17848,"ancestor_record_offset":15724,"inlinee":24990,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":18032,"parent_record_offset":18000,"ancestor_record_offset":15724,"inlinee":24992,"source_line":177,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":19304,"parent_record_offset":19248,"ancestor_record_offset":15724,"inlinee":24995,"source_line":203,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":433,"record_offset":22252,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":7596,"source_line":1083,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/io/reader.rs
//     evidence={"module":433,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":15724,"inlinee":7590,"source_line":293,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":433,"record_offset":22984,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":433,"record_offset":22844,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":433,"record_offset":15968,"parent_record_offset":15876,"ancestor_record_offset":15724,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17120,"parent_record_offset":17096,"ancestor_record_offset":15724,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":15724,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":15724,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":433,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":15724,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":433,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":15724,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":433,"record_offset":25856,"parent_record_offset":25832,"ancestor_record_offset":15724,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17212,"parent_record_offset":17032,"ancestor_record_offset":15724,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":21852,"parent_record_offset":21596,"ancestor_record_offset":15724,"inlinee":7564,"source_line":2750,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17096,"parent_record_offset":17072,"ancestor_record_offset":15724,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18380,"parent_record_offset":18336,"ancestor_record_offset":15724,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18336,"parent_record_offset":18056,"ancestor_record_offset":15724,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17072,"parent_record_offset":17032,"ancestor_record_offset":15724,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":15724,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":15724,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18448,"parent_record_offset":18416,"ancestor_record_offset":15724,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18508,"parent_record_offset":18380,"ancestor_record_offset":15724,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18540,"parent_record_offset":18336,"ancestor_record_offset":15724,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":433,"record_offset":25764,"parent_record_offset":25736,"ancestor_record_offset":15724,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":433,"record_offset":25800,"parent_record_offset":25764,"ancestor_record_offset":15724,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":433,"record_offset":26036,"parent_record_offset":25764,"ancestor_record_offset":15724,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":433,"record_offset":23732,"parent_record_offset":23700,"ancestor_record_offset":15724,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":433,"record_offset":25832,"parent_record_offset":25800,"ancestor_record_offset":15724,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":433,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":15724,"inlinee":13866,"source_line":246,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF04A
// 首条位置: {"module":1308,"inlinee":61514}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":149716,"parent_record_offset":149680,"ancestor_record_offset":149680,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":149748,"parent_record_offset":149716,"ancestor_record_offset":149680,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":149780,"parent_record_offset":149748,"ancestor_record_offset":149680,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::ThumbnailTextures::find_backed_thumbnail_range
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x619B
// 首条位置: {"module":433,"inlinee":24987}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15936,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15996,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":16024,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":433,"record_offset":15968,"parent_record_offset":15876,"ancestor_record_offset":15876,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::ThumbnailTextures::assign_pool_indices
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x619C
// 首条位置: {"module":433,"inlinee":24988}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":16364,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":433,"record_offset":16928,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16804,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":16608,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20760,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20732,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":16976,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":18948,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":16524,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16280,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::position
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":17848,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":9708,"source_line":331,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SavedSession::screenshot_path
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":433,"record_offset":17032,"parent_record_offset":16112,"ancestor_record_offset":16112,"inlinee":23028,"source_line":322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":16112,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17460,"parent_record_offset":17404,"ancestor_record_offset":16112,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17432,"parent_record_offset":17404,"ancestor_record_offset":16112,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":16112,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":433,"record_offset":18480,"parent_record_offset":18380,"ancestor_record_offset":16112,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":18568,"parent_record_offset":18336,"ancestor_record_offset":16112,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":433,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":16112,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":16112,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":16112,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":16112,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":16112,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":18652,"parent_record_offset":18336,"ancestor_record_offset":16112,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18912,"parent_record_offset":17968,"ancestor_record_offset":16112,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18000,"parent_record_offset":17968,"ancestor_record_offset":16112,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":16112,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":16480,"parent_record_offset":16456,"ancestor_record_offset":16112,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":16112,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=42
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":17244,"parent_record_offset":17032,"ancestor_record_offset":16112,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":433,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":16112,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":16112,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16328,"parent_record_offset":16280,"ancestor_record_offset":16112,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":16112,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":16112,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16392,"parent_record_offset":16364,"ancestor_record_offset":16112,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16456,"parent_record_offset":16420,"ancestor_record_offset":16112,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16420,"parent_record_offset":16392,"ancestor_record_offset":16112,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":16112,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":16112,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":16112,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":20668,"parent_record_offset":20640,"ancestor_record_offset":16112,"inlinee":6147,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":17888,"parent_record_offset":17848,"ancestor_record_offset":16112,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SavedSession::screenshot_path
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":433,"record_offset":19356,"parent_record_offset":19304,"ancestor_record_offset":16112,"inlinee":23028,"source_line":322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":17968,"parent_record_offset":17848,"ancestor_record_offset":16112,"inlinee":24990,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":18032,"parent_record_offset":18000,"ancestor_record_offset":16112,"inlinee":24992,"source_line":177,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":19304,"parent_record_offset":19248,"ancestor_record_offset":16112,"inlinee":24995,"source_line":203,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17120,"parent_record_offset":17096,"ancestor_record_offset":16112,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":16112,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":16112,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17212,"parent_record_offset":17032,"ancestor_record_offset":16112,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17096,"parent_record_offset":17072,"ancestor_record_offset":16112,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18380,"parent_record_offset":18336,"ancestor_record_offset":16112,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18336,"parent_record_offset":18056,"ancestor_record_offset":16112,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17072,"parent_record_offset":17032,"ancestor_record_offset":16112,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":16112,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18448,"parent_record_offset":18416,"ancestor_record_offset":16112,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18508,"parent_record_offset":18380,"ancestor_record_offset":16112,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18540,"parent_record_offset":18336,"ancestor_record_offset":16112,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 173
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x619E
// 首条位置: {"module":433,"inlinee":24990}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18912,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18000,"parent_record_offset":17968,"ancestor_record_offset":17968,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":433,"record_offset":18480,"parent_record_offset":18380,"ancestor_record_offset":17968,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":18568,"parent_record_offset":18336,"ancestor_record_offset":17968,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":433,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":17968,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":18652,"parent_record_offset":18336,"ancestor_record_offset":17968,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":17968,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":17968,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":17968,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":18032,"parent_record_offset":18000,"ancestor_record_offset":17968,"inlinee":24992,"source_line":177,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":17968,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18180,"parent_record_offset":18156,"ancestor_record_offset":17968,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18156,"parent_record_offset":18132,"ancestor_record_offset":17968,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18108,"parent_record_offset":18080,"ancestor_record_offset":17968,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18380,"parent_record_offset":18336,"ancestor_record_offset":17968,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18336,"parent_record_offset":18056,"ancestor_record_offset":17968,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":17968,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":17968,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18448,"parent_record_offset":18416,"ancestor_record_offset":17968,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18508,"parent_record_offset":18380,"ancestor_record_offset":17968,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18540,"parent_record_offset":18336,"ancestor_record_offset":17968,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61A0
// 首条位置: {"module":433,"inlinee":24992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":18032,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":433,"record_offset":18480,"parent_record_offset":18380,"ancestor_record_offset":18032,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":18568,"parent_record_offset":18336,"ancestor_record_offset":18032,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":433,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":18032,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":18652,"parent_record_offset":18336,"ancestor_record_offset":18032,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":18032,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":18032,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":18032,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":18032,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18180,"parent_record_offset":18156,"ancestor_record_offset":18032,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":18156,"parent_record_offset":18132,"ancestor_record_offset":18032,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18108,"parent_record_offset":18080,"ancestor_record_offset":18032,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18380,"parent_record_offset":18336,"ancestor_record_offset":18032,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18336,"parent_record_offset":18056,"ancestor_record_offset":18032,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":18032,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18448,"parent_record_offset":18416,"ancestor_record_offset":18032,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18508,"parent_record_offset":18380,"ancestor_record_offset":18032,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18540,"parent_record_offset":18336,"ancestor_record_offset":18032,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 203
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61A3
// 首条位置: {"module":433,"inlinee":24995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":20172,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::SavedSession::screenshot_path
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":433,"record_offset":19356,"parent_record_offset":19304,"ancestor_record_offset":19304,"inlinee":23028,"source_line":322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":19808,"parent_record_offset":19784,"ancestor_record_offset":19304,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":19784,"parent_record_offset":19728,"ancestor_record_offset":19304,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":19756,"parent_record_offset":19728,"ancestor_record_offset":19304,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":19728,"parent_record_offset":19700,"ancestor_record_offset":19304,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":19568,"parent_record_offset":19356,"ancestor_record_offset":19304,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":19444,"parent_record_offset":19420,"ancestor_record_offset":19304,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":19492,"parent_record_offset":19468,"ancestor_record_offset":19304,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":19468,"parent_record_offset":19444,"ancestor_record_offset":19304,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":19536,"parent_record_offset":19356,"ancestor_record_offset":19304,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":19420,"parent_record_offset":19396,"ancestor_record_offset":19304,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":19396,"parent_record_offset":19356,"ancestor_record_offset":19304,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::refresh_screenshot
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 233
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x61A2
// 首条位置: {"module":433,"inlinee":24994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21952,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22928,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22196,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":22956,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":23012,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22692,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":433,"record_offset":22252,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":7596,"source_line":1083,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":433,"record_offset":22984,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":433,"record_offset":22844,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":21852,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":7564,"source_line":2750,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":21712,"parent_record_offset":21596,"ancestor_record_offset":21596,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":23164,"parent_record_offset":23140,"ancestor_record_offset":21596,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":23140,"parent_record_offset":23084,"ancestor_record_offset":21596,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":23112,"parent_record_offset":23084,"ancestor_record_offset":21596,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":21596,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":433,"record_offset":22028,"parent_record_offset":21984,"ancestor_record_offset":21596,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21984,"parent_record_offset":21952,"ancestor_record_offset":21596,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":433,"record_offset":22300,"parent_record_offset":22276,"ancestor_record_offset":21596,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":23036,"parent_record_offset":23012,"ancestor_record_offset":21596,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":21908,"parent_record_offset":21880,"ancestor_record_offset":21596,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22500,"parent_record_offset":22252,"ancestor_record_offset":21596,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22536,"parent_record_offset":22252,"ancestor_record_offset":21596,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":21596,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":22892,"parent_record_offset":22868,"ancestor_record_offset":21596,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/io/reader.rs
//     evidence={"module":433,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":21596,"inlinee":7590,"source_line":293,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":21760,"parent_record_offset":21736,"ancestor_record_offset":21596,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":21808,"parent_record_offset":21784,"ancestor_record_offset":21596,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":21784,"parent_record_offset":21760,"ancestor_record_offset":21596,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":433,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":21596,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":21736,"parent_record_offset":21712,"ancestor_record_offset":21596,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":22348,"parent_record_offset":22324,"ancestor_record_offset":21596,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":21596,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":433,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":21596,"inlinee":13866,"source_line":246,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::thumbnails_startup
// 记录形式: compiled_procedure; instances=1
// 源码行: 270
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":433,"record_offset":2948,"rva":6075104,"va":5374784224}
// 编译实例 RVAs: 0x5cb2e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b2730
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t947176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e8e8730e00","size":5,"soft_fail":false,"target_va":5375731504,"va":5374784323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406efdf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1198139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e83b481200","size":5,"soft_fail":false,"target_va":5375983088,"va":5374784944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e949e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9213266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e852958c00","size":5,"soft_fail":false,"target_va":5383997920,"va":5374784649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28269351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e8275baf01","size":5,"soft_fail":false,"target_va":5403054112,"va":5374784756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::mip_levels
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1556386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e8a2bf1700","size":5,"soft_fail":false,"target_va":5376340928,"va":5374784537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1556003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e823be1700","size":5,"soft_fail":false,"target_va":5376340512,"va":5374784504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1556196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784224,"raw_hex":"e8e4be1700","size":5,"soft_fail":false,"target_va":5376340720,"va":5374784519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3400,"parent_record_offset":2948,"ancestor_record_offset":2948,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":433,"record_offset":3340,"parent_record_offset":2948,"ancestor_record_offset":2948,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":433,"record_offset":3116,"parent_record_offset":2948,"ancestor_record_offset":2948,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":3916,"parent_record_offset":3876,"ancestor_record_offset":2948,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":3844,"parent_record_offset":3820,"ancestor_record_offset":2948,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":433,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2948,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3876,"parent_record_offset":3484,"ancestor_record_offset":2948,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3484,"parent_record_offset":3456,"ancestor_record_offset":2948,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3792,"parent_record_offset":3484,"ancestor_record_offset":2948,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3820,"parent_record_offset":3484,"ancestor_record_offset":2948,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":3948,"parent_record_offset":3484,"ancestor_record_offset":2948,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":433,"record_offset":3456,"parent_record_offset":3400,"ancestor_record_offset":2948,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":433,"record_offset":3368,"parent_record_offset":3340,"ancestor_record_offset":2948,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":3740,"parent_record_offset":3704,"ancestor_record_offset":2948,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter_map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter_map.rs
//     evidence={"module":433,"record_offset":3536,"parent_record_offset":3484,"ancestor_record_offset":2948,"inlinee":8593,"source_line":63,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":433,"record_offset":3704,"parent_record_offset":3672,"ancestor_record_offset":2948,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":433,"record_offset":3672,"parent_record_offset":3592,"ancestor_record_offset":2948,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":433,"record_offset":3428,"parent_record_offset":3400,"ancestor_record_offset":2948,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":3564,"parent_record_offset":3536,"ancestor_record_offset":2948,"inlinee":8594,"source_line":2898,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find_map::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":3636,"parent_record_offset":3592,"ancestor_record_offset":2948,"inlinee":8630,"source_line":2905,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":3592,"parent_record_offset":3564,"ancestor_record_offset":2948,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::thumbnails_startup::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 317
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF27A
// 首条位置: {"module":1330,"inlinee":62074}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$40::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":21004,"parent_record_offset":19876,"ancestor_record_offset":19876,"inlinee":5366,"source_line":2485,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$41::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1330,"record_offset":21036,"parent_record_offset":19876,"ancestor_record_offset":19876,"inlinee":5368,"source_line":2496,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":19976,"parent_record_offset":19876,"ancestor_record_offset":19876,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1330,"record_offset":20228,"parent_record_offset":19876,"ancestor_record_offset":19876,"inlinee":4772,"source_line":631,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1330,"record_offset":19920,"parent_record_offset":19876,"ancestor_record_offset":19876,"inlinee":22680,"source_line":43,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":20492,"parent_record_offset":20468,"ancestor_record_offset":19876,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1330,"record_offset":20468,"parent_record_offset":20412,"ancestor_record_offset":19876,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":20440,"parent_record_offset":20412,"ancestor_record_offset":19876,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1330,"record_offset":20412,"parent_record_offset":20384,"ancestor_record_offset":19876,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1330,"record_offset":19944,"parent_record_offset":19920,"ancestor_record_offset":19876,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":20136,"parent_record_offset":20012,"ancestor_record_offset":19876,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":20164,"parent_record_offset":20012,"ancestor_record_offset":19876,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1330,"record_offset":20264,"parent_record_offset":20228,"ancestor_record_offset":19876,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":19876,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":19876,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::thumbnails_startup::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":1330,"record_offset":20012,"parent_record_offset":19976,"ancestor_record_offset":19876,"inlinee":62076,"source_line":320,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1330,"record_offset":20192,"parent_record_offset":20012,"ancestor_record_offset":19876,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1330,"record_offset":20052,"parent_record_offset":20012,"ancestor_record_offset":19876,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::thumbnails_startup::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 320
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF27C
// 首条位置: {"module":1330,"inlinee":62076}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1330,"record_offset":20136,"parent_record_offset":20012,"ancestor_record_offset":20012,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1_formatted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1330,"record_offset":20164,"parent_record_offset":20012,"ancestor_record_offset":20012,"inlinee":4755,"source_line":354,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":1330,"record_offset":20192,"parent_record_offset":20012,"ancestor_record_offset":20012,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":1330,"record_offset":20052,"parent_record_offset":20012,"ancestor_record_offset":20012,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":20076,"parent_record_offset":20052,"ancestor_record_offset":20012,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1330,"record_offset":20100,"parent_record_offset":20076,"ancestor_record_offset":20012,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/01330.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::render_session_selection_previews
// 记录形式: compiled_procedure; instances=1
// 源码行: 334
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":433,"record_offset":3992,"rva":6075856,"va":5374784976}
// 编译实例 RVAs: 0x5cb5d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$std..sys..pal..windows..time..SystemTime$u20$as$u20$core..cmp..PartialOrd$GT$::partial_cmp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2720156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e89c812900","size":5,"soft_fail":false,"target_va":5377510528,"va":5374790367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14005b630
//     owner=unresolved_namespace; count=1
//     target_aliases=11 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5715390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e842caa8ff","size":5,"soft_fail":false,"target_va":5369083440,"va":5374798825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400eb810
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5125868","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e814c9b1ff","size":5,"soft_fail":false,"target_va":5369673744,"va":5374799607,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402c6b80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3169519","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e811a3cfff","size":5,"soft_fail":false,"target_va":5371620224,"va":5374789738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140360530
//     owner=unresolved_namespace; count=1
//     target_aliases=79 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2550021","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8fb16d9ff","size":5,"soft_fail":false,"target_va":5372249392,"va":5374799408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140370260
//     owner=unresolved_namespace; count=2
//     target_aliases=89 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2483724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f419daff","size":5,"soft_fail":false,"target_va":5372314208,"va":5374797927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=4
//     target_aliases=16 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2167167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e881eedeff","size":5,"soft_fail":false,"target_va":5372625024,"va":5374792186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2040110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8d2dee0ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5374789801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=18
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2038276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8fce5e0ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5374787983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140540a60
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-575807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8c136f7ff","size":5,"soft_fail":false,"target_va":5374216800,"va":5374792602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405471c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-546823","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f9a7f7ff","size":5,"soft_fail":false,"target_va":5374243264,"va":5374790082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405af710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-119930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8862bfeff","size":5,"soft_fail":false,"target_va":5374670608,"va":5374790533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406299f0
//     owner=unresolved_namespace; count=6
//     target_aliases=28 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t383139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8a3d80500","size":5,"soft_fail":false,"target_va":5375171056,"va":5374787912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140644950
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t486936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8186e0700","size":5,"soft_fail":false,"target_va":5375281488,"va":5374794547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406eb110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1160626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8b2b51100","size":5,"soft_fail":false,"target_va":5375963408,"va":5374802777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ec140
//     owner=unresolved_namespace; count=20
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1166903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e837ce1100","size":5,"soft_fail":false,"target_va":5375967552,"va":5374800644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072a6e0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1431164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e87cd61500","size":5,"soft_fail":false,"target_va":5376222944,"va":5374791775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140744400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1537679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e88f761700","size":5,"soft_fail":false,"target_va":5376328704,"va":5374791020,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d774d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8037679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e82fa57a00","size":5,"soft_fail":false,"target_va":5382829264,"va":5374791580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d80e20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8075653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e885397b00","size":5,"soft_fail":false,"target_va":5382868512,"va":5374792854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ddb570
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8446505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e829e28000","size":5,"soft_fail":false,"target_va":5383239024,"va":5374792514,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6180
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8555356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e85c8b8200","size":5,"soft_fail":false,"target_va":5383348608,"va":5374793247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df64b0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8555947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8ab8d8200","size":5,"soft_fail":false,"target_va":5383349424,"va":5374793472,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8555327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e83f8b8200","size":5,"soft_fail":false,"target_va":5383349824,"va":5374794492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df6960
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8558163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e853968200","size":5,"soft_fail":false,"target_va":5383350624,"va":5374792456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141043600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10973714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e81272a700","size":5,"soft_fail":false,"target_va":5385762304,"va":5374788585,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14209b690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28115114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8aa00ad01","size":5,"soft_fail":false,"target_va":5402900112,"va":5374784993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28161219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8c3b4ad01","size":5,"soft_fail":false,"target_va":5402958208,"va":5374796984,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420ca6a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28301173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e875d7af01","size":5,"soft_fail":false,"target_va":5403092640,"va":5374791462,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420cad50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28303489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e881e0af01","size":5,"soft_fail":false,"target_va":5403094352,"va":5374790858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420caee0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28305587","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8b3e8af01","size":5,"soft_fail":false,"target_va":5403094752,"va":5374789160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28805080","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8d887b701","size":5,"soft_fail":false,"target_va":5403593344,"va":5374788259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=16
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28803323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8fb80b701","size":5,"soft_fail":false,"target_va":5403593360,"va":5374790032,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mod.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t28806896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f08eb701","size":5,"soft_fail":false,"target_va":5403594288,"va":5374787387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28181366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e87603ae01","size":5,"soft_fail":false,"target_va":5402978144,"va":5374796773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2740251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e81bd02900","size":5,"soft_fail":false,"target_va":5377529888,"va":5374789632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28129182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e89e37ad01","size":5,"soft_fail":false,"target_va":5402919072,"va":5374789885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2575380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8144c2700","size":5,"soft_fail":false,"target_va":5377364336,"va":5374788951,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28182245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8e506ae01","size":5,"soft_fail":false,"target_va":5402978912,"va":5374796662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=12
//     evidence={"assembly":"\tcallq\t28182294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e81607ae01","size":5,"soft_fail":false,"target_va":5402978880,"va":5374796581,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t28190290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e85226ae01","size":5,"soft_fail":false,"target_va":5402979152,"va":5374788857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28181887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e87f05ae01","size":5,"soft_fail":false,"target_va":5402978944,"va":5374797052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place$LT$image..error..ImageError$GT$::h821c9e82e908dcc6 (.llvm.14833637362440507379)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7702516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f4877500","size":5,"soft_fail":false,"target_va":5382500832,"va":5374798311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<image::dynimage::DynamicImage> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-7175","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f9e3ffff","size":5,"soft_fail":false,"target_va":5374784048,"va":5374791218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28183367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8470bae01","size":5,"soft_fail":false,"target_va":5402980128,"va":5374796756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28183224","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8b80aae01","size":5,"soft_fail":false,"target_va":5402980320,"va":5374797091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t906790","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e826d60d00","size":5,"soft_fail":false,"target_va":5375692416,"va":5374785621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t911643","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e81be90d00","size":5,"soft_fail":false,"target_va":5375704400,"va":5374792752,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_builder::DrawBuilder::new
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t908288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e800dc0d00","size":5,"soft_fail":false,"target_va":5375700688,"va":5374792395,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::new_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t603768","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e878360900","size":5,"soft_fail":false,"target_va":5375395536,"va":5374791763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::draw_stream::RasterPass::render
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t602009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8992f0900","size":5,"soft_fail":false,"target_va":5375398240,"va":5374796226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-532725","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e80bdff7ff","size":5,"soft_fail":false,"target_va":5374255184,"va":5374787904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t994788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8e42d0f00","size":5,"soft_fail":false,"target_va":5375789248,"va":5374794455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::dynimage::DynamicImage::to_rgba8
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1258844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e85c351300","size":5,"soft_fail":false,"target_va":5376049776,"va":5374790927,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] image::io::reader::Reader$LT$std..io..buffered..bufreader..BufReader$LT$std..fs..File$GT$$GT$::open_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1287918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8eea61300","size":5,"soft_fail":false,"target_va":5376078320,"va":5374790397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2563553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8e11d2700","size":5,"soft_fail":false,"target_va":5377353776,"va":5374790218,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t28172458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8aae0ad01","size":5,"soft_fail":false,"target_va":5402968992,"va":5374796529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t28174247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8a7e7ad01","size":5,"soft_fail":false,"target_va":5402970800,"va":5374796548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t1550094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e80ea71700","size":5,"soft_fail":false,"target_va":5376341760,"va":5374791661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::backtrace::Backtrace::capture
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2606479","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e88fc52700","size":5,"soft_fail":false,"target_va":5377395552,"va":5374789068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::Metadata::modified
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2618788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8a4f52700","size":5,"soft_fail":false,"target_va":5377408048,"va":5374789255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t2574423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e857482700","size":5,"soft_fail":false,"target_va":5377366224,"va":5374791796,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::panicking::panic_count::is_zero_slow_path
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28185009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8b111ae01","size":5,"soft_fail":false,"target_va":5402975328,"va":5374790314,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2651825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8b1762800","size":5,"soft_fail":false,"target_va":5377439776,"va":5374787946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t2652662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e8f6792800","size":5,"soft_fail":false,"target_va":5377440784,"va":5374788117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::stat
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t2684237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e84df52800","size":5,"soft_fail":false,"target_va":5377473264,"va":5374789022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::mutex::futex::Mutex::wake
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28185439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e85f13ae01","size":5,"soft_fail":false,"target_va":5402975696,"va":5374790252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::sync::once::queue::Once::call
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t28185984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5374784976,"raw_hex":"e88015ae01","size":5,"soft_fail":false,"target_va":5402975712,"va":5374789723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":433,"record_offset":15668,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":5800,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":433,"record_offset":28912,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":27696,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":7292,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":28312,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4556,"source_line":2154,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":12128,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":27612,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::read_texture
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":433,"record_offset":27480,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_builder::impl$2::uniform
//     owner=private_source_ownership_candidate; count=7
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_builder.rs
//     evidence={"module":433,"record_offset":27404,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::DrawPacket::new
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":433,"record_offset":26836,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":21986,"source_line":105,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::pass_builder::draw_stream::RasterPass::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":433,"record_offset":26656,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7327,"source_line":314,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::session_selection::ThumbnailTextures::extract_thumbnails_to_display
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":15724,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":24986,"source_line":72,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":433,"record_offset":4236,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":9932,"source_line":34,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":433,"record_offset":13912,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":9929,"source_line":41,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":433,"record_offset":28368,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":433,"record_offset":11948,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":433,"record_offset":28740,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7340,"source_line":626,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":433,"record_offset":27528,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7336,"source_line":617,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24192,"parent_record_offset":24168,"ancestor_record_offset":3992,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24216,"parent_record_offset":24192,"ancestor_record_offset":3992,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17484,"parent_record_offset":17460,"ancestor_record_offset":3992,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":24168,"parent_record_offset":24140,"ancestor_record_offset":3992,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":433,"record_offset":17460,"parent_record_offset":17404,"ancestor_record_offset":3992,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::set::BTreeSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\collections\btree\set.rs
//     evidence={"module":433,"record_offset":25540,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":8697,"source_line":902,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":433,"record_offset":23616,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":433,"record_offset":23664,"parent_record_offset":23640,"ancestor_record_offset":3992,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":25148,"parent_record_offset":25124,"ancestor_record_offset":3992,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17432,"parent_record_offset":17404,"ancestor_record_offset":3992,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":25124,"parent_record_offset":25096,"ancestor_record_offset":3992,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24644,"parent_record_offset":24608,"ancestor_record_offset":3992,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24576,"parent_record_offset":24552,"ancestor_record_offset":3992,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24140,"parent_record_offset":24112,"ancestor_record_offset":3992,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":3992,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":24288,"parent_record_offset":24112,"ancestor_record_offset":3992,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":433,"record_offset":17404,"parent_record_offset":17376,"ancestor_record_offset":3992,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":24048,"parent_record_offset":24020,"ancestor_record_offset":3992,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":24020,"parent_record_offset":23992,"ancestor_record_offset":3992,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23964,"parent_record_offset":23920,"ancestor_record_offset":3992,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23992,"parent_record_offset":23964,"ancestor_record_offset":3992,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":433,"record_offset":23920,"parent_record_offset":23892,"ancestor_record_offset":3992,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":433,"record_offset":23892,"parent_record_offset":23864,"ancestor_record_offset":3992,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::push_str
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":24444,"parent_record_offset":24408,"ancestor_record_offset":3992,"inlinee":4956,"source_line":1066,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$24::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":24408,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":24636,"source_line":2430,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":433,"record_offset":23864,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24516,"parent_record_offset":24492,"ancestor_record_offset":3992,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24608,"parent_record_offset":24516,"ancestor_record_offset":3992,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24468,"parent_record_offset":24444,"ancestor_record_offset":3992,"inlinee":4270,"source_line":2537,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24676,"parent_record_offset":24516,"ancestor_record_offset":3992,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24552,"parent_record_offset":24516,"ancestor_record_offset":3992,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":24084,"parent_record_offset":24048,"ancestor_record_offset":3992,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":16364,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":26360,"parent_record_offset":26336,"ancestor_record_offset":3992,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":433,"record_offset":21268,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":6287,"source_line":2952,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$4::spec_extend
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":433,"record_offset":24492,"parent_record_offset":24468,"ancestor_record_offset":3992,"inlinee":4272,"source_line":53,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::from_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/error.rs
//     evidence={"module":433,"record_offset":22028,"parent_record_offset":21984,"ancestor_record_offset":3992,"inlinee":5381,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$0::ext_context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21984,"parent_record_offset":21952,"ancestor_record_offset":3992,"inlinee":5405,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::impl$0::context
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"anyhow","version":"1.0.86","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/anyhow-1.0.86/src/context.rs
//     evidence={"module":433,"record_offset":21952,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":10988,"source_line":46,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":433,"record_offset":15692,"parent_record_offset":15668,"ancestor_record_offset":3992,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::component::Components::get_resource_id
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/component.rs
//     evidence={"module":433,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":3992,"inlinee":5560,"source_line":751,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":433,"record_offset":15472,"parent_record_offset":15448,"ancestor_record_offset":3992,"inlinee":4387,"source_line":394,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":433,"record_offset":15448,"parent_record_offset":15424,"ancestor_record_offset":3992,"inlinee":4386,"source_line":363,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":433,"record_offset":5724,"parent_record_offset":5692,"ancestor_record_offset":3992,"inlinee":4380,"source_line":92,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":433,"record_offset":5668,"parent_record_offset":5280,"ancestor_record_offset":3992,"inlinee":5566,"source_line":85,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":433,"record_offset":15340,"parent_record_offset":14956,"ancestor_record_offset":3992,"inlinee":4377,"source_line":112,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":433,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":3992,"inlinee":4378,"source_line":69,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":433,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":3992,"inlinee":4359,"source_line":285,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/resource.rs
//     evidence={"module":433,"record_offset":15424,"parent_record_offset":15364,"ancestor_record_offset":3992,"inlinee":4385,"source_line":113,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":433,"record_offset":5356,"parent_record_offset":5332,"ancestor_record_offset":3992,"inlinee":4363,"source_line":56,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":433,"record_offset":5332,"parent_record_offset":5308,"ancestor_record_offset":3992,"inlinee":4361,"source_line":369,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$10::get::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":433,"record_offset":5448,"parent_record_offset":5384,"ancestor_record_offset":3992,"inlinee":4368,"source_line":58,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":433,"record_offset":5536,"parent_record_offset":5512,"ancestor_record_offset":3992,"inlinee":4370,"source_line":370,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/mod.rs
//     evidence={"module":433,"record_offset":15540,"parent_record_offset":15516,"ancestor_record_offset":3992,"inlinee":4389,"source_line":2074,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":15516,"parent_record_offset":14956,"ancestor_record_offset":3992,"inlinee":4388,"source_line":252,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":4264,"parent_record_offset":4236,"ancestor_record_offset":3992,"inlinee":5559,"source_line":311,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":5280,"parent_record_offset":4264,"ancestor_record_offset":3992,"inlinee":5565,"source_line":358,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":13944,"parent_record_offset":13912,"ancestor_record_offset":3992,"inlinee":7123,"source_line":416,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/world/unsafe_world_cell.rs
//     evidence={"module":433,"record_offset":14956,"parent_record_offset":13944,"ancestor_record_offset":3992,"inlinee":4383,"source_line":440,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::any::impl$7::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\any.rs
//     evidence={"module":433,"record_offset":5012,"parent_record_offset":4988,"ancestor_record_offset":3992,"inlinee":4165,"source_line":616,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":433,"record_offset":27232,"parent_record_offset":27204,"ancestor_record_offset":3992,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":433,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":3992,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":433,"record_offset":27260,"parent_record_offset":27204,"ancestor_record_offset":3992,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":433,"record_offset":25508,"parent_record_offset":25484,"ancestor_record_offset":3992,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::clone::impls::impl$19::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\clone.rs
//     evidence={"module":433,"record_offset":18480,"parent_record_offset":18380,"ancestor_record_offset":3992,"inlinee":4516,"source_line":219,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22928,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":22196,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":4202,"source_line":1171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$25::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":5060,"parent_record_offset":5036,"ancestor_record_offset":3992,"inlinee":4169,"source_line":1469,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":433,"record_offset":4988,"parent_record_offset":4964,"ancestor_record_offset":3992,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":433,"record_offset":22300,"parent_record_offset":22276,"ancestor_record_offset":3992,"inlinee":4750,"source_line":701,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":433,"record_offset":26688,"parent_record_offset":26656,"ancestor_record_offset":3992,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":433,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":3992,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":433,"record_offset":4564,"parent_record_offset":4540,"ancestor_record_offset":3992,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15936,"parent_record_offset":15876,"ancestor_record_offset":3992,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":433,"record_offset":15996,"parent_record_offset":15876,"ancestor_record_offset":3992,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":16024,"parent_record_offset":15876,"ancestor_record_offset":3992,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":433,"record_offset":22956,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":433,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":3992,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\rev.rs
//     evidence={"module":433,"record_offset":18684,"parent_record_offset":18652,"ancestor_record_offset":3992,"inlinee":4521,"source_line":20,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\take.rs
//     evidence={"module":433,"record_offset":16928,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":7708,"source_line":38,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":3992,"inlinee":6059,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16632,"parent_record_offset":16608,"ancestor_record_offset":3992,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16804,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":8501,"source_line":84,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16656,"parent_record_offset":16632,"ancestor_record_offset":3992,"inlinee":11986,"source_line":300,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":433,"record_offset":16832,"parent_record_offset":16804,"ancestor_record_offset":3992,"inlinee":8503,"source_line":307,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":433,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":3992,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":433,"record_offset":26384,"parent_record_offset":26360,"ancestor_record_offset":3992,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":433,"record_offset":16056,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":26336,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::rev
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":18652,"parent_record_offset":18336,"ancestor_record_offset":3992,"inlinee":4519,"source_line":3348,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":433,"record_offset":16608,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":433,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":3992,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":433,"record_offset":31372,"parent_record_offset":31344,"ancestor_record_offset":3992,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":433,"record_offset":4676,"parent_record_offset":4652,"ancestor_record_offset":3992,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":433,"record_offset":28396,"parent_record_offset":28368,"ancestor_record_offset":3992,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":21532,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":5472,"parent_record_offset":5448,"ancestor_record_offset":3992,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":5248,"parent_record_offset":4292,"ancestor_record_offset":3992,"inlinee":5564,"source_line":1863,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20760,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":20732,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":5512,"parent_record_offset":5332,"ancestor_record_offset":3992,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18000,"parent_record_offset":17968,"ancestor_record_offset":3992,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":3992,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":5768,"parent_record_offset":4236,"ancestor_record_offset":3992,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$15::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":28336,"parent_record_offset":28312,"ancestor_record_offset":3992,"inlinee":4558,"source_line":2174,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":433,"record_offset":18416,"parent_record_offset":18380,"ancestor_record_offset":3992,"inlinee":4512,"source_line":2008,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":5616,"parent_record_offset":5592,"ancestor_record_offset":3992,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":24324,"parent_record_offset":24048,"ancestor_record_offset":3992,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":433,"record_offset":16752,"parent_record_offset":16728,"ancestor_record_offset":3992,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=74
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":17244,"parent_record_offset":17032,"ancestor_record_offset":3992,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":433,"record_offset":16572,"parent_record_offset":16548,"ancestor_record_offset":3992,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":433,"record_offset":4900,"parent_record_offset":4876,"ancestor_record_offset":3992,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":433,"record_offset":4820,"parent_record_offset":4796,"ancestor_record_offset":3992,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":21496,"parent_record_offset":21472,"ancestor_record_offset":3992,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16728,"parent_record_offset":16704,"ancestor_record_offset":3992,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":433,"record_offset":16328,"parent_record_offset":16280,"ancestor_record_offset":3992,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":24244,"parent_record_offset":24216,"ancestor_record_offset":3992,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":433,"record_offset":27332,"parent_record_offset":27308,"ancestor_record_offset":3992,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":23820,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":12064,"parent_record_offset":11976,"ancestor_record_offset":3992,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":3992,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":21908,"parent_record_offset":21880,"ancestor_record_offset":3992,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22500,"parent_record_offset":22252,"ancestor_record_offset":3992,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":433,"record_offset":22536,"parent_record_offset":22252,"ancestor_record_offset":3992,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18592,"parent_record_offset":18568,"ancestor_record_offset":3992,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":433,"record_offset":18616,"parent_record_offset":18592,"ancestor_record_offset":3992,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":5384,"parent_record_offset":5356,"ancestor_record_offset":3992,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":16524,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":433,"record_offset":21292,"parent_record_offset":21268,"ancestor_record_offset":3992,"inlinee":8499,"source_line":1053,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16392,"parent_record_offset":16364,"ancestor_record_offset":3992,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":5420,"parent_record_offset":5384,"ancestor_record_offset":3992,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":5592,"parent_record_offset":5560,"ancestor_record_offset":3992,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16456,"parent_record_offset":16420,"ancestor_record_offset":3992,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":433,"record_offset":16420,"parent_record_offset":16392,"ancestor_record_offset":3992,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":16548,"parent_record_offset":16524,"ancestor_record_offset":3992,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":21316,"parent_record_offset":21292,"ancestor_record_offset":3992,"inlinee":8500,"source_line":212,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":21472,"parent_record_offset":21408,"ancestor_record_offset":3992,"inlinee":6387,"source_line":101,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":433,"record_offset":21092,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16864,"parent_record_offset":16832,"ancestor_record_offset":3992,"inlinee":8504,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":21204,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":3992,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":20668,"parent_record_offset":20640,"ancestor_record_offset":3992,"inlinee":6147,"source_line":372,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":16280,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::position
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":433,"record_offset":17848,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":9708,"source_line":331,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":25976,"parent_record_offset":25800,"ancestor_record_offset":3992,"inlinee":6715,"source_line":637,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicPtr::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":23756,"parent_record_offset":23732,"ancestor_record_offset":3992,"inlinee":6265,"source_line":1431,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":15564,"parent_record_offset":15540,"ancestor_record_offset":3992,"inlinee":4390,"source_line":2413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":12216,"parent_record_offset":12184,"ancestor_record_offset":3992,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":12008,"parent_record_offset":11976,"ancestor_record_offset":3992,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26064,"parent_record_offset":26036,"ancestor_record_offset":3992,"inlinee":8243,"source_line":2468,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":3992,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":12240,"parent_record_offset":12216,"ancestor_record_offset":3992,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":3992,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":15588,"parent_record_offset":15564,"ancestor_record_offset":3992,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26000,"parent_record_offset":25976,"ancestor_record_offset":3992,"inlinee":5323,"source_line":3288,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":433,"record_offset":26088,"parent_record_offset":26064,"ancestor_record_offset":3992,"inlinee":8188,"source_line":3319,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] core::tuple::impl$11::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\tuple.rs
//     evidence={"module":433,"record_offset":5036,"parent_record_offset":5012,"ancestor_record_offset":3992,"inlinee":4167,"source_line":31,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$14::default
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":433,"record_offset":26880,"parent_record_offset":26836,"ancestor_record_offset":3992,"inlinee":21988,"source_line":137,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":433,"record_offset":26764,"parent_record_offset":26736,"ancestor_record_offset":3992,"inlinee":23122,"source_line":89,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::RasterFramebufferBinding::new_color
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":433,"record_offset":26736,"parent_record_offset":26712,"ancestor_record_offset":3992,"inlinee":6964,"source_line":145,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::framebuffer_binding::impl$5::from
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/framebuffer_binding.rs
//     evidence={"module":433,"record_offset":26712,"parent_record_offset":26688,"ancestor_record_offset":3992,"inlinee":7000,"source_line":179,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SavedSession::screenshot_path
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":433,"record_offset":17032,"parent_record_offset":16112,"ancestor_record_offset":3992,"inlinee":23028,"source_line":322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::ThumbnailTextures::assign_pool_indices
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":16112,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":24988,"source_line":147,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::ThumbnailTextures::find_backed_thumbnail_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":15876,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":24987,"source_line":123,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":17968,"parent_record_offset":17848,"ancestor_record_offset":3992,"inlinee":24990,"source_line":173,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":18032,"parent_record_offset":18000,"ancestor_record_offset":3992,"inlinee":24992,"source_line":177,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":19304,"parent_record_offset":19248,"ancestor_record_offset":3992,"inlinee":24995,"source_line":203,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::render_loop::session_selection::refresh_screenshot
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/session_selection/mod.rs
//     evidence={"module":433,"record_offset":21596,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":24994,"source_line":233,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/lib.rs
//     evidence={"module":433,"record_offset":4964,"parent_record_offset":4940,"ancestor_record_offset":3992,"inlinee":4854,"source_line":171,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":433,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3992,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":433,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3992,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/map.rs
//     evidence={"module":433,"record_offset":4940,"parent_record_offset":4744,"ancestor_record_offset":3992,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::as_ptr
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4852,"parent_record_offset":4768,"ancestor_record_offset":3992,"inlinee":4850,"source_line":513,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::Bucket::from_base_index
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4796,"parent_record_offset":4768,"ancestor_record_offset":3992,"inlinee":4848,"source_line":355,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":5196,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4857,"source_line":175,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::bucket
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4768,"parent_record_offset":4744,"ancestor_record_offset":3992,"inlinee":4846,"source_line":995,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::find
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4392,"parent_record_offset":4368,"ancestor_record_offset":3992,"inlinee":4826,"source_line":1454,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTable::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4368,"parent_record_offset":4340,"ancestor_record_offset":3992,"inlinee":5563,"source_line":1477,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4416,"parent_record_offset":4392,"ancestor_record_offset":3992,"inlinee":4828,"source_line":2166,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":433,"record_offset":4624,"parent_record_offset":4596,"ancestor_record_offset":3992,"inlinee":4839,"source_line":49,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":433,"record_offset":4652,"parent_record_offset":4624,"ancestor_record_offset":3992,"inlinee":4840,"source_line":74,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::BitMask::remove_lowest_bit
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":433,"record_offset":4712,"parent_record_offset":4596,"ancestor_record_offset":3992,"inlinee":4843,"source_line":37,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::bitmask::impl$3::next
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/bitmask.rs
//     evidence={"module":433,"record_offset":4596,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4837,"source_line":128,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/mod.rs
//     evidence={"module":433,"record_offset":4744,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4845,"source_line":1464,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::load
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":433,"record_offset":4456,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4831,"source_line":52,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_byte
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":433,"record_offset":4540,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4834,"source_line":79,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::inner::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"hashbrown","version":"0.14.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/hashbrown-0.14.5/src/raw/sse2.rs
//     evidence={"module":433,"record_offset":5112,"parent_record_offset":4416,"ancestor_record_offset":3992,"inlinee":4855,"source_line":97,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] image::dynimage::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/dynimage.rs
//     evidence={"module":433,"record_offset":22252,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":7596,"source_line":1083,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] image::io::reader::Reader::open
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/image-0.25.1/src/io/reader.rs
//     evidence={"module":433,"record_offset":22276,"parent_record_offset":22252,"ancestor_record_offset":3992,"inlinee":7590,"source_line":293,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":433,"record_offset":12156,"parent_record_offset":12128,"ancestor_record_offset":3992,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":433,"record_offset":22984,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":433,"record_offset":22844,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":433,"record_offset":27576,"parent_record_offset":27552,"ancestor_record_offset":3992,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":433,"record_offset":5560,"parent_record_offset":5536,"ancestor_record_offset":3992,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":433,"record_offset":11976,"parent_record_offset":11948,"ancestor_record_offset":3992,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":433,"record_offset":12184,"parent_record_offset":12156,"ancestor_record_offset":3992,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":433,"record_offset":28764,"parent_record_offset":28740,"ancestor_record_offset":3992,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":433,"record_offset":27552,"parent_record_offset":27528,"ancestor_record_offset":3992,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":433,"record_offset":26788,"parent_record_offset":26764,"ancestor_record_offset":3992,"inlinee":7325,"source_line":326,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":433,"record_offset":26912,"parent_record_offset":26880,"ancestor_record_offset":3992,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":433,"record_offset":15968,"parent_record_offset":15876,"ancestor_record_offset":3992,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$55::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17120,"parent_record_offset":17096,"ancestor_record_offset":3992,"inlinee":4494,"source_line":1560,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17168,"parent_record_offset":17144,"ancestor_record_offset":3992,"inlinee":4498,"source_line":562,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\ffi\os_str.rs
//     evidence={"module":433,"record_offset":17144,"parent_record_offset":17120,"ancestor_record_offset":3992,"inlinee":4496,"source_line":580,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\fs.rs
//     evidence={"module":433,"record_offset":21880,"parent_record_offset":21852,"ancestor_record_offset":3992,"inlinee":4767,"source_line":1926,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":433,"record_offset":26976,"parent_record_offset":26944,"ancestor_record_offset":3992,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":433,"record_offset":27076,"parent_record_offset":27040,"ancestor_record_offset":3992,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":433,"record_offset":27204,"parent_record_offset":27040,"ancestor_record_offset":3992,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":433,"record_offset":26944,"parent_record_offset":26912,"ancestor_record_offset":3992,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panic_count::count_is_zero
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":433,"record_offset":25880,"parent_record_offset":25856,"ancestor_record_offset":3992,"inlinee":6703,"source_line":435,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::panicking::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\panicking.rs
//     evidence={"module":433,"record_offset":25856,"parent_record_offset":25832,"ancestor_record_offset":3992,"inlinee":6701,"source_line":586,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17212,"parent_record_offset":17032,"ancestor_record_offset":3992,"inlinee":4747,"source_line":2551,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::metadata
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":21852,"parent_record_offset":21596,"ancestor_record_offset":3992,"inlinee":7564,"source_line":2750,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17096,"parent_record_offset":17072,"ancestor_record_offset":3992,"inlinee":4492,"source_line":2041,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$112::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18380,"parent_record_offset":18336,"ancestor_record_offset":3992,"inlinee":4510,"source_line":594,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18336,"parent_record_offset":18056,"ancestor_record_offset":3992,"inlinee":4500,"source_line":992,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$44::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":17072,"parent_record_offset":17032,"ancestor_record_offset":3992,"inlinee":4490,"source_line":1780,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$57::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":3992,"inlinee":4680,"source_line":1914,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$79::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":22324,"parent_record_offset":22300,"ancestor_record_offset":3992,"inlinee":4752,"source_line":3182,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$85::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18448,"parent_record_offset":18416,"ancestor_record_offset":3992,"inlinee":4514,"source_line":141,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$94::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18508,"parent_record_offset":18380,"ancestor_record_offset":3992,"inlinee":4518,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$96::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\path.rs
//     evidence={"module":433,"record_offset":18540,"parent_record_offset":18336,"ancestor_record_offset":3992,"inlinee":4502,"source_line":376,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$13::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":433,"record_offset":25484,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":10882,"source_line":540,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::mutex::impl$14::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\mutex.rs
//     evidence={"module":433,"record_offset":25764,"parent_record_offset":25736,"ancestor_record_offset":3992,"inlinee":8241,"source_line":548,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::once::Once::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\once.rs
//     evidence={"module":433,"record_offset":23700,"parent_record_offset":15724,"ancestor_record_offset":3992,"inlinee":17118,"source_line":139,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sync::poison::Flag::done
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sync\poison.rs
//     evidence={"module":433,"record_offset":25800,"parent_record_offset":25764,"ancestor_record_offset":3992,"inlinee":6706,"source_line":52,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::mutex::futex::Mutex::unlock
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\mutex\futex.rs
//     evidence={"module":433,"record_offset":26036,"parent_record_offset":25764,"ancestor_record_offset":3992,"inlinee":8242,"source_line":94,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::sync::once::queue::Once::is_completed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\sync\once\queue.rs
//     evidence={"module":433,"record_offset":23732,"parent_record_offset":23700,"ancestor_record_offset":3992,"inlinee":6263,"source_line":117,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":433,"record_offset":27104,"parent_record_offset":27076,"ancestor_record_offset":3992,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":433,"record_offset":27136,"parent_record_offset":27104,"ancestor_record_offset":3992,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":433,"record_offset":27040,"parent_record_offset":27008,"ancestor_record_offset":3992,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":433,"record_offset":27008,"parent_record_offset":26976,"ancestor_record_offset":3992,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::panicking
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\mod.rs
//     evidence={"module":433,"record_offset":25832,"parent_record_offset":25800,"ancestor_record_offset":3992,"inlinee":6699,"source_line":796,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$31::partial_cmp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\time.rs
//     evidence={"module":433,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":3992,"inlinee":13866,"source_line":246,"archive":"_pdb/A/modules/00433.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::systems::render_loop::session_selection::render_session_selection_previews::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 414
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF079
// 首条位置: {"module":1308,"inlinee":61561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1308,"record_offset":293848,"parent_record_offset":293704,"ancestor_record_offset":293704,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1308,"record_offset":293736,"parent_record_offset":293704,"ancestor_record_offset":293704,"inlinee":10010,"source_line":120,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":293784,"parent_record_offset":293760,"ancestor_record_offset":293704,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1308,"record_offset":293808,"parent_record_offset":293784,"ancestor_record_offset":293704,"inlinee":14887,"source_line":1555,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1308,"record_offset":293760,"parent_record_offset":293736,"ancestor_record_offset":293704,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":293876,"parent_record_offset":293848,"ancestor_record_offset":293704,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01308.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1308,"record_offset":293904,"parent_record_offset":293876,"ancestor_record_offset":293704,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01308.jsonl.gz"}
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
// 完整结构及原始类型记录: mod.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::session_selection::PooledThumbnail
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::session_selection::PooledThumbnail>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::session_selection::PooledThumbnail"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<country_core::systems::render_loop::session_selection::PooledThumbnail,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["country_core::systems::render_loop::session_selection::PooledThumbnail", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$14::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::game_loop::GameLoopPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::main_camera::MainCameraPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::Ui2dPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::ToolbarPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::session::SessionFlowPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::MenuFlowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::painterly::PainterlyPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::screenshot::ScreenshotPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::lut_gallery::LutGalleryPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin>", "constructor": "bevy_app::plugin::sealed::impl$14::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::game_loop::GameLoopPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::main_camera::MainCameraPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::Ui2dPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::mei::core::MeiPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::ToolbarPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::session::SessionFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "toolbar::menu::MenuFlowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::painterly::PainterlyPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::screenshot::ScreenshotPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::session_selection::SessionThumbnailsPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::render_loop::lut_gallery::LutGalleryPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::gui::settings::SettingsMenuPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::systems::render_loop::session_selection::ThumbnailTextures
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 33
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>", "constructor": "bevy_ecs::change_detection::ResMut", "arguments": ["country_core::systems::render_loop::session_selection::ThumbnailTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::schedule::schedule::Schedules::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::systems::render_loop::session_selection::ThumbnailTextures>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::systems::render_loop::session_selection::ThumbnailTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>,bevy_app::main_schedule::PreStartup,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_app::sub_app::SubApp::add_systems", "arguments": ["tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "bevy_app::main_schedule::PreStartup", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::systems::render_loop::session_selection::ThumbnailTextures>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::systems::render_loop::session_selection::ThumbnailTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::default_system_sets<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::default_system_sets", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::SystemMeta::new<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::SystemMeta::new", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::initialize<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::initialize", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::run_unsafe", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::get_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::get_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::has_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::has_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_exclusive<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_exclusive", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::set_last_run<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::set_last_run", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::apply_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::queue_deferred", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::check_change_tick<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::check_change_tick", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::name<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::name", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::is_send<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>", "constructor": "bevy_ecs::system::function_system::impl$7::is_send", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)", "void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::render_loop::session_selection::ThumbnailTextures>", "constructor": "bevy_ecs::component::ComponentDescriptor::drop_ptr", "arguments": ["country_core::systems::render_loop::session_selection::ThumbnailTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::impl$7::init_state<country_core::systems::render_loop::session_selection::ThumbnailTextures>", "constructor": "bevy_ecs::system::system_param::impl$7::init_state", "arguments": ["country_core::systems::render_loop::session_selection::ThumbnailTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/country-core/src/systems/render_loop/session_selection/mod.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: mod.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// ------------------------------------------------------------------
// [PDB 函数上下文] country_core::systems::render_loop::session_selection::render_session_selection_previews
// [资源接口动作；资源实例 unknown] {"role":"uniform_input_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::uniform","relationship":"pdb_inline_direct","evidence":{"module":433,"record_offset":27404,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":23127,"source_line":461,"archive":"_pdb/A/modules/00433.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// [资源接口动作；资源实例 unknown] {"role":"texture_read_api","symbol":"country_core::render::pass_builder::draw_builder::impl$2::read_texture","relationship":"pdb_inline_direct","evidence":{"module":433,"record_offset":27480,"parent_record_offset":3992,"ancestor_record_offset":3992,"inlinee":7334,"source_line":483,"archive":"_pdb/A/modules/00433.jsonl.gz"},"resource_instance":"unknown","source_level_arguments":"unknown","scope":"PDB inline or physical native dependency; node identity/dataflow not inferred"}
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":3133120,"caller_aliases":{"alias_group_rva":3133120,"count":1,"display_examples":["tiny_glade::render_loop::compile_frame_render_graph"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":3144467,"position_among_mapped_render_targets":60,"mapped_render_target_count":70,"previous_callsite":3144143,"next_callsite":3144500,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/lut_gallery.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":14958960,"caller_aliases":{"alias_group_rva":14958960,"count":1,"display_examples":["bevy_ecs::system::system::System::run"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":14959542,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":15287824,"caller_aliases":{"alias_group_rva":15287824,"count":1,"display_examples":["bevy_ecs::system::function_system::impl$7::run_unsafe"],"complete_catalog":"_pdb/A/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":15288315,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 9; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>
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
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::ThumbnailTextures::assign_pool_indices
// 信息 ID: c482b77010fe9d68d09e07d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=16112; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::ThumbnailTextures::extract_thumbnails_to_display
// 信息 ID: e923389606e492d756ba207c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=15724; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::ThumbnailTextures::find_backed_thumbnail_range
// 信息 ID: e1c59419b5fe451a0bcc68ad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=15876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$0::build
// 信息 ID: 4fc6b4bc50fd869fe7cb9a36
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0
// 信息 ID: 134c07b7d13fdf345955557f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=17968; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$0::closure$0
// 信息 ID: 22aeff42cbf5191a27600b49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=18032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices::closure$1
// 信息 ID: a50f64cbdd4bb3f282ffca75
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::systems::render_loop::session_selection::impl$1::assign_pool_indices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=19304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display::closure$0
// 信息 ID: edfa954ae5c61eb9cb16f5fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<ref$<rhapsody::raw::texture::Texture> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<ref$<rhapsody::raw::texture::Texture> > >,core::iter::adapters::map::Map<core::slice::iter::Iter<usize>,country_core::systems::render_loop::session_selection::impl$1::extract_thumbnails_to_display::closure_env$0> >; module=1308, inline_record=149680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::impl$2::default
// 信息 ID: 05254362450519d52fc0ea05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::impl$0::build; module=433, inline_record=2920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::refresh_screenshot
// 信息 ID: e6c15e7710067a3bd1543b45
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=21596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::render_session_selection_previews
// 信息 ID: ae90d5f307fe312566bf4d5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::render_loop::compile_frame_render_graph @ 0x2ffb13 -> 0x5cb5d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::render_session_selection_previews::closure$0
// 信息 ID: 88643da7e41f6cc014b833b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::render_session_selection_previews", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<country_core::resources::session::SavedSession>,enum2$<core::option::Option<ref$<rhapsody::raw::texture::Texture> > > >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<country_core::resources::session::SavedSession> >,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<tuple$<ref$<country_core::resources::session::SavedSession>,enum2$<core::option::Option<ref$<rhapsody::raw::texture::Texture> > > >,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<country_core::resources::session::SavedSession> >,country_core::systems::render_loop::session_selection::render_session_selection_previews::closure_env$0> >; module=1308, inline_record=293704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::thumbnails_startup
// 信息 ID: ce0c821bf746412963bd0424
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e119f1ee10af1b15a1caac63: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> > @ 0xe443b6 -> 0x5cb2e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::change_detection::ResMut<country_core::systems::render_loop::session_selection::ThumbnailTextures>)> @ 0xe947fb -> 0x5cb2e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::thumbnails_startup::closure$0
// 信息 ID: a842611fca3f458b80168864
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::thumbnails_startup", "captures": "unknown"}
// [闭包环境类型引用] country_core::systems::render_loop::session_selection::thumbnails_startup::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<usize>,country_core::systems::render_loop::session_selection::thumbnails_startup::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<usize>,country_core::systems::render_loop::session_selection::thumbnails_startup::closure_env$0>; module=1330, inline_record=19876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::systems::render_loop::session_selection::thumbnails_startup::closure$0::closure$0
// 信息 ID: ccf97c0f9fbc98a72da13ff8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::systems::render_loop::session_selection::thumbnails_startup", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<usize>,country_core::systems::render_loop::session_selection::thumbnails_startup::closure_env$0>; module=1330, inline_record=20012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
