// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/tiny-glade/src/render_loop.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 560d1cb1e0bb9315cfe88a20e04f8c7b796b5bc26d17ffd2847d6f2867af7c84
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: render_loop.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 57, 66, 197
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.7.4"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use image; // namespace candidate
//   package identities: [{"name":"image","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.1"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use tiny_glade; // namespace candidate
//   package identities: [{"name":"tiny-glade","source":"local_or_path","version":"0.2.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3908
// 首条位置: {"module":197,"inlinee":14600}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x270E
// 首条位置: {"module":66,"inlinee":9998}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":6804,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":7948,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":5872,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":11416,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":10340,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":66,"record_offset":19552,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":66,"record_offset":19316,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":25132,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":19524,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":24936,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":16904,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":10152,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":66,"record_offset":12968,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":9556,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":9712,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":19348,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":19232,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":6916,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":9768,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":11288,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":21480,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":14640,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10014,"source_line":139,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::screenshot::next_requested_screenshot_stage
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/screenshot.rs
//     evidence={"module":66,"record_offset":17432,"parent_record_offset":5312,"ancestor_record_offset":5312,"inlinee":10018,"source_line":511,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":5312,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":24532,"parent_record_offset":24476,"ancestor_record_offset":5312,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6336,"parent_record_offset":6312,"ancestor_record_offset":5312,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24504,"parent_record_offset":24476,"ancestor_record_offset":5312,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24476,"parent_record_offset":24452,"ancestor_record_offset":5312,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6384,"parent_record_offset":6360,"ancestor_record_offset":5312,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":6360,"parent_record_offset":6336,"ancestor_record_offset":5312,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":24452,"parent_record_offset":24428,"ancestor_record_offset":5312,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17716,"parent_record_offset":17692,"ancestor_record_offset":5312,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17692,"parent_record_offset":17668,"ancestor_record_offset":5312,"inlinee":5037,"source_line":1593,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":5312,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6288,"parent_record_offset":6248,"ancestor_record_offset":5312,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17644,"parent_record_offset":17432,"ancestor_record_offset":5312,"inlinee":7678,"source_line":3482,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":5312,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":17668,"parent_record_offset":17644,"ancestor_record_offset":5312,"inlinee":5310,"source_line":3273,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":6828,"parent_record_offset":6804,"ancestor_record_offset":5312,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6636,"parent_record_offset":6612,"ancestor_record_offset":5312,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6612,"parent_record_offset":6588,"ancestor_record_offset":5312,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":6560,"parent_record_offset":6528,"ancestor_record_offset":5312,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":8564,"parent_record_offset":8028,"ancestor_record_offset":5312,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6504,"parent_record_offset":5964,"ancestor_record_offset":5312,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":5312,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":5996,"parent_record_offset":5964,"ancestor_record_offset":5312,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":8648,"parent_record_offset":8588,"ancestor_record_offset":5312,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":6588,"parent_record_offset":6528,"ancestor_record_offset":5312,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5312,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5312,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=17
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":5312,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":6704,"parent_record_offset":6680,"ancestor_record_offset":5312,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":6680,"parent_record_offset":5964,"ancestor_record_offset":5312,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":5312,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":7972,"parent_record_offset":7948,"ancestor_record_offset":5312,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":8028,"parent_record_offset":7972,"ancestor_record_offset":5312,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5896,"parent_record_offset":5872,"ancestor_record_offset":5312,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":5964,"parent_record_offset":5896,"ancestor_record_offset":5312,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":66,"record_offset":19668,"parent_record_offset":19576,"ancestor_record_offset":5312,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":5312,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":66,"record_offset":14756,"parent_record_offset":14640,"ancestor_record_offset":5312,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":11356,"parent_record_offset":11288,"ancestor_record_offset":5312,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":9884,"parent_record_offset":9848,"ancestor_record_offset":5312,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":9996,"parent_record_offset":9968,"ancestor_record_offset":5312,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":66,"record_offset":19692,"parent_record_offset":19668,"ancestor_record_offset":5312,"inlinee":4566,"source_line":720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":66,"record_offset":19636,"parent_record_offset":19612,"ancestor_record_offset":5312,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":6136,"parent_record_offset":6072,"ancestor_record_offset":5312,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":66,"record_offset":19612,"parent_record_offset":19576,"ancestor_record_offset":5312,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":66,"record_offset":19576,"parent_record_offset":19552,"ancestor_record_offset":5312,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":6160,"parent_record_offset":6136,"ancestor_record_offset":5312,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":17992,"parent_record_offset":17432,"ancestor_record_offset":5312,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":6200,"parent_record_offset":6020,"ancestor_record_offset":5312,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":13772,"parent_record_offset":12968,"ancestor_record_offset":5312,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":5312,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14844,"parent_record_offset":14812,"ancestor_record_offset":5312,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14872,"parent_record_offset":14812,"ancestor_record_offset":5312,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":66,"record_offset":19716,"parent_record_offset":19692,"ancestor_record_offset":5312,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":9920,"parent_record_offset":9768,"ancestor_record_offset":5312,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17952,"parent_record_offset":17928,"ancestor_record_offset":5312,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":66,"record_offset":17876,"parent_record_offset":17840,"ancestor_record_offset":5312,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":10020,"parent_record_offset":9996,"ancestor_record_offset":5312,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":9848,"parent_record_offset":9816,"ancestor_record_offset":5312,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":5312,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":5312,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6108,"parent_record_offset":6072,"ancestor_record_offset":5312,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":6428,"parent_record_offset":6248,"ancestor_record_offset":5312,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::IterMut::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17928,"parent_record_offset":17904,"ancestor_record_offset":5312,"inlinee":5339,"source_line":99,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17840,"parent_record_offset":17432,"ancestor_record_offset":5312,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$194::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":66,"record_offset":17904,"parent_record_offset":17840,"ancestor_record_offset":5312,"inlinee":5337,"source_line":440,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":5312,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":10272,"parent_record_offset":10208,"ancestor_record_offset":5312,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":9620,"parent_record_offset":9584,"ancestor_record_offset":5312,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":14696,"parent_record_offset":14672,"ancestor_record_offset":5312,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":5312,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":9644,"parent_record_offset":9620,"ancestor_record_offset":5312,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":5312,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":17488,"parent_record_offset":17432,"ancestor_record_offset":5312,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":5312,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14900,"parent_record_offset":14812,"ancestor_record_offset":5312,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14788,"parent_record_offset":14640,"ancestor_record_offset":5312,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":5312,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14924,"parent_record_offset":14900,"ancestor_record_offset":5312,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14672,"parent_record_offset":14640,"ancestor_record_offset":5312,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":5312,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":10208,"parent_record_offset":10180,"ancestor_record_offset":5312,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":9676,"parent_record_offset":9584,"ancestor_record_offset":5312,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":10244,"parent_record_offset":10208,"ancestor_record_offset":5312,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":9816,"parent_record_offset":9768,"ancestor_record_offset":5312,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":5312,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":5312,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":11328,"parent_record_offset":11288,"ancestor_record_offset":5312,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":16968,"parent_record_offset":16940,"ancestor_record_offset":5312,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":11384,"parent_record_offset":11288,"ancestor_record_offset":5312,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":16940,"parent_record_offset":16904,"ancestor_record_offset":5312,"inlinee":10015,"source_line":199,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":17140,"parent_record_offset":17104,"ancestor_record_offset":5312,"inlinee":10016,"source_line":207,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 139
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x271E
// 首条位置: {"module":66,"inlinee":10014}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":66,"record_offset":14756,"parent_record_offset":14640,"ancestor_record_offset":14640,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14788,"parent_record_offset":14640,"ancestor_record_offset":14640,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14672,"parent_record_offset":14640,"ancestor_record_offset":14640,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14844,"parent_record_offset":14812,"ancestor_record_offset":14640,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":66,"record_offset":14872,"parent_record_offset":14812,"ancestor_record_offset":14640,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":14696,"parent_record_offset":14672,"ancestor_record_offset":14640,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":14720,"parent_record_offset":14696,"ancestor_record_offset":14640,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14900,"parent_record_offset":14812,"ancestor_record_offset":14640,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":66,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":14640,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":66,"record_offset":14924,"parent_record_offset":14900,"ancestor_record_offset":14640,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x271F
// 首条位置: {"module":66,"inlinee":10015}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":16968,"parent_record_offset":16940,"ancestor_record_offset":16940,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":17036,"parent_record_offset":16968,"ancestor_record_offset":16940,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":17008,"parent_record_offset":16968,"ancestor_record_offset":16940,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":17064,"parent_record_offset":16968,"ancestor_record_offset":16940,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2720
// 首条位置: {"module":66,"inlinee":10016}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":17168,"parent_record_offset":17140,"ancestor_record_offset":17140,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":17236,"parent_record_offset":17168,"ancestor_record_offset":17140,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":17208,"parent_record_offset":17168,"ancestor_record_offset":17140,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":17264,"parent_record_offset":17168,"ancestor_record_offset":17140,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::execute_presentation_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 409
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3912
// 首条位置: {"module":197,"inlinee":14610}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":197,"record_offset":19224,"parent_record_offset":19172,"ancestor_record_offset":19172,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::execute_presentation_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 427
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2730
// 首条位置: {"module":66,"inlinee":10032}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":43340,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43508,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42436,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":47860,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":45668,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45072,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45228,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":43452,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":45284,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45912,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_owned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45856,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":9691,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::import_unowned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":45884,"parent_record_offset":42268,"ancestor_record_offset":42268,"inlinee":9692,"source_line":55,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42876,"parent_record_offset":42852,"ancestor_record_offset":42268,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42924,"parent_record_offset":42900,"ancestor_record_offset":42268,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":42900,"parent_record_offset":42876,"ancestor_record_offset":42268,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42852,"parent_record_offset":42828,"ancestor_record_offset":42268,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42828,"parent_record_offset":42788,"ancestor_record_offset":42268,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":42788,"parent_record_offset":42764,"ancestor_record_offset":42268,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":43364,"parent_record_offset":43340,"ancestor_record_offset":42268,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43176,"parent_record_offset":43152,"ancestor_record_offset":42268,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43152,"parent_record_offset":43128,"ancestor_record_offset":42268,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":43100,"parent_record_offset":43068,"ancestor_record_offset":42268,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":44112,"parent_record_offset":43576,"ancestor_record_offset":42268,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43044,"parent_record_offset":42504,"ancestor_record_offset":42268,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43068,"parent_record_offset":43044,"ancestor_record_offset":42268,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":42536,"parent_record_offset":42504,"ancestor_record_offset":42268,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":44196,"parent_record_offset":44136,"ancestor_record_offset":42268,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":43128,"parent_record_offset":43068,"ancestor_record_offset":42268,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42584,"parent_record_offset":42560,"ancestor_record_offset":42268,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42560,"parent_record_offset":42536,"ancestor_record_offset":42268,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":42764,"parent_record_offset":42740,"ancestor_record_offset":42268,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":43244,"parent_record_offset":43220,"ancestor_record_offset":42268,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43220,"parent_record_offset":42504,"ancestor_record_offset":42268,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42464,"parent_record_offset":42436,"ancestor_record_offset":42268,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":43576,"parent_record_offset":43508,"ancestor_record_offset":42268,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":42504,"parent_record_offset":42436,"ancestor_record_offset":42268,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":45252,"parent_record_offset":45228,"ancestor_record_offset":42268,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":45400,"parent_record_offset":45364,"ancestor_record_offset":42268,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":45512,"parent_record_offset":45484,"ancestor_record_offset":42268,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":42676,"parent_record_offset":42612,"ancestor_record_offset":42268,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":42700,"parent_record_offset":42676,"ancestor_record_offset":42268,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":42740,"parent_record_offset":42560,"ancestor_record_offset":42268,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":44324,"parent_record_offset":44296,"ancestor_record_offset":42268,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":45436,"parent_record_offset":45284,"ancestor_record_offset":42268,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":45536,"parent_record_offset":45512,"ancestor_record_offset":42268,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":45364,"parent_record_offset":45332,"ancestor_record_offset":42268,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":42612,"parent_record_offset":42584,"ancestor_record_offset":42268,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42992,"parent_record_offset":42968,"ancestor_record_offset":42268,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42648,"parent_record_offset":42612,"ancestor_record_offset":42268,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":42968,"parent_record_offset":42788,"ancestor_record_offset":42268,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":43268,"parent_record_offset":43244,"ancestor_record_offset":42268,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45788,"parent_record_offset":45724,"ancestor_record_offset":42268,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45136,"parent_record_offset":45100,"ancestor_record_offset":42268,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45812,"parent_record_offset":45788,"ancestor_record_offset":42268,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":45160,"parent_record_offset":45136,"ancestor_record_offset":42268,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":43292,"parent_record_offset":43268,"ancestor_record_offset":42268,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":45696,"parent_record_offset":45668,"ancestor_record_offset":42268,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":45100,"parent_record_offset":45072,"ancestor_record_offset":42268,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":45724,"parent_record_offset":45696,"ancestor_record_offset":42268,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":45192,"parent_record_offset":45100,"ancestor_record_offset":42268,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":45760,"parent_record_offset":45724,"ancestor_record_offset":42268,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":45332,"parent_record_offset":45284,"ancestor_record_offset":42268,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":43476,"parent_record_offset":43452,"ancestor_record_offset":42268,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":45484,"parent_record_offset":45460,"ancestor_record_offset":42268,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 487
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3909
// 首条位置: {"module":197,"inlinee":14601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 496
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x272A
// 首条位置: {"module":66,"inlinee":10026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":29212,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":29380,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":28280,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":36928,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":37876,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::get_or_insert_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":31772,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":7077,"source_line":1691,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":39832,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":31584,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":66,"record_offset":36092,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":30988,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":31144,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::descriptor_set::impl$1::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":66,"record_offset":29324,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":10000,"source_line":11,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstants::push
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":31200,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":10008,"source_line":144,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":35964,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::export
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":66,"record_offset":37904,"parent_record_offset":28068,"ancestor_record_offset":28068,"inlinee":9695,"source_line":75,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":28068,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33168,"parent_record_offset":33112,"ancestor_record_offset":28068,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28744,"parent_record_offset":28720,"ancestor_record_offset":28068,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33140,"parent_record_offset":33112,"ancestor_record_offset":28068,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":28068,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28792,"parent_record_offset":28768,"ancestor_record_offset":28068,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":28768,"parent_record_offset":28744,"ancestor_record_offset":28068,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33088,"parent_record_offset":33064,"ancestor_record_offset":28068,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":28068,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28696,"parent_record_offset":28656,"ancestor_record_offset":28068,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":28656,"parent_record_offset":28632,"ancestor_record_offset":28068,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":29236,"parent_record_offset":29212,"ancestor_record_offset":28068,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":34920,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":29044,"parent_record_offset":29020,"ancestor_record_offset":28068,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":29020,"parent_record_offset":28996,"ancestor_record_offset":28068,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":28968,"parent_record_offset":28936,"ancestor_record_offset":28068,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":29996,"parent_record_offset":29460,"ancestor_record_offset":28068,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28912,"parent_record_offset":28372,"ancestor_record_offset":28068,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28936,"parent_record_offset":28912,"ancestor_record_offset":28068,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28404,"parent_record_offset":28372,"ancestor_record_offset":28068,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":30080,"parent_record_offset":30020,"ancestor_record_offset":28068,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":28996,"parent_record_offset":28936,"ancestor_record_offset":28068,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":28068,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":28068,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":28632,"parent_record_offset":28608,"ancestor_record_offset":28068,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":29112,"parent_record_offset":29088,"ancestor_record_offset":28068,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29088,"parent_record_offset":28372,"ancestor_record_offset":28068,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28332,"parent_record_offset":28304,"ancestor_record_offset":28068,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29404,"parent_record_offset":29380,"ancestor_record_offset":28068,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":29460,"parent_record_offset":29404,"ancestor_record_offset":28068,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28304,"parent_record_offset":28280,"ancestor_record_offset":28068,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":28372,"parent_record_offset":28304,"ancestor_record_offset":28068,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":66,"record_offset":31168,"parent_record_offset":31144,"ancestor_record_offset":28068,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":66,"record_offset":36032,"parent_record_offset":35964,"ancestor_record_offset":28068,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":31316,"parent_record_offset":31280,"ancestor_record_offset":28068,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":31428,"parent_record_offset":31400,"ancestor_record_offset":28068,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":28544,"parent_record_offset":28480,"ancestor_record_offset":28068,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":28568,"parent_record_offset":28544,"ancestor_record_offset":28068,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":28608,"parent_record_offset":28428,"ancestor_record_offset":28068,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":35292,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":29432,"parent_record_offset":29404,"ancestor_record_offset":28068,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":31352,"parent_record_offset":31200,"ancestor_record_offset":28068,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":31452,"parent_record_offset":31428,"ancestor_record_offset":28068,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":66,"record_offset":32728,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":31280,"parent_record_offset":31248,"ancestor_record_offset":28068,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":32700,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":28480,"parent_record_offset":28452,"ancestor_record_offset":28068,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":28068,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28516,"parent_record_offset":28480,"ancestor_record_offset":28068,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":28836,"parent_record_offset":28656,"ancestor_record_offset":28068,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":29136,"parent_record_offset":29112,"ancestor_record_offset":28068,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31704,"parent_record_offset":31640,"ancestor_record_offset":28068,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31052,"parent_record_offset":31016,"ancestor_record_offset":28068,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31728,"parent_record_offset":31704,"ancestor_record_offset":28068,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":31076,"parent_record_offset":31052,"ancestor_record_offset":28068,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":29160,"parent_record_offset":29136,"ancestor_record_offset":28068,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":66,"record_offset":32868,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":5993,"source_line":688,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":66,"record_offset":31612,"parent_record_offset":31584,"ancestor_record_offset":28068,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":31016,"parent_record_offset":30988,"ancestor_record_offset":28068,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":66,"record_offset":31640,"parent_record_offset":31612,"ancestor_record_offset":28068,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":31108,"parent_record_offset":31016,"ancestor_record_offset":28068,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":66,"record_offset":31676,"parent_record_offset":31640,"ancestor_record_offset":28068,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":31248,"parent_record_offset":31200,"ancestor_record_offset":28068,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":29348,"parent_record_offset":29324,"ancestor_record_offset":28068,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":31400,"parent_record_offset":31376,"ancestor_record_offset":28068,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":34976,"parent_record_offset":31812,"ancestor_record_offset":28068,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":35032,"parent_record_offset":34976,"ancestor_record_offset":28068,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":66,"record_offset":36060,"parent_record_offset":35964,"ancestor_record_offset":28068,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":66,"record_offset":31812,"parent_record_offset":31772,"ancestor_record_offset":28068,"inlinee":10028,"source_line":526,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 526
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x272C
// 首条位置: {"module":66,"inlinee":10028}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":34920,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":31924,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":34028,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":35292,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":32836,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":33828,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":66,"record_offset":32728,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::first
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":32700,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":4740,"source_line":153,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] image::buffer_::ImageBuffer::into_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"image","version":"0.25.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/image-0.25.1/src/buffer.rs
//     evidence={"module":66,"record_offset":32868,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":5993,"source_line":688,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":34976,"parent_record_offset":31812,"ancestor_record_offset":31812,"inlinee":10030,"source_line":23,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33192,"parent_record_offset":33168,"ancestor_record_offset":31812,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":66,"record_offset":33168,"parent_record_offset":33112,"ancestor_record_offset":31812,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":32348,"parent_record_offset":32324,"ancestor_record_offset":31812,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33140,"parent_record_offset":33112,"ancestor_record_offset":31812,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33112,"parent_record_offset":33088,"ancestor_record_offset":31812,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":32396,"parent_record_offset":32372,"ancestor_record_offset":31812,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":32372,"parent_record_offset":32348,"ancestor_record_offset":31812,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":66,"record_offset":33088,"parent_record_offset":33064,"ancestor_record_offset":31812,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":32324,"parent_record_offset":32300,"ancestor_record_offset":31812,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":32300,"parent_record_offset":32260,"ancestor_record_offset":31812,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":66,"record_offset":32260,"parent_record_offset":32236,"ancestor_record_offset":31812,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":66,"record_offset":34944,"parent_record_offset":34920,"ancestor_record_offset":31812,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":31812,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":32624,"parent_record_offset":32600,"ancestor_record_offset":31812,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":66,"record_offset":32572,"parent_record_offset":32540,"ancestor_record_offset":31812,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":32516,"parent_record_offset":31980,"ancestor_record_offset":31812,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":34624,"parent_record_offset":34084,"ancestor_record_offset":31812,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":32540,"parent_record_offset":32516,"ancestor_record_offset":31812,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":32008,"parent_record_offset":31980,"ancestor_record_offset":31812,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":32600,"parent_record_offset":32540,"ancestor_record_offset":31812,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":66,"record_offset":34708,"parent_record_offset":34648,"ancestor_record_offset":31812,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":32056,"parent_record_offset":32032,"ancestor_record_offset":31812,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":32032,"parent_record_offset":32008,"ancestor_record_offset":31812,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":66,"record_offset":32236,"parent_record_offset":32212,"ancestor_record_offset":31812,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":66,"record_offset":34824,"parent_record_offset":34800,"ancestor_record_offset":31812,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":34800,"parent_record_offset":34084,"ancestor_record_offset":31812,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":31980,"parent_record_offset":31924,"ancestor_record_offset":31812,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":66,"record_offset":34084,"parent_record_offset":34028,"ancestor_record_offset":31812,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":66,"record_offset":32148,"parent_record_offset":32084,"ancestor_record_offset":31812,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":32172,"parent_record_offset":32148,"ancestor_record_offset":31812,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":32212,"parent_record_offset":32032,"ancestor_record_offset":31812,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":66,"record_offset":31952,"parent_record_offset":31924,"ancestor_record_offset":31812,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=22
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":32768,"parent_record_offset":32728,"ancestor_record_offset":31812,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":32084,"parent_record_offset":32056,"ancestor_record_offset":31812,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":32464,"parent_record_offset":32440,"ancestor_record_offset":31812,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":32120,"parent_record_offset":32084,"ancestor_record_offset":31812,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":66,"record_offset":32440,"parent_record_offset":32260,"ancestor_record_offset":31812,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":34848,"parent_record_offset":34824,"ancestor_record_offset":31812,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":66,"record_offset":34872,"parent_record_offset":34848,"ancestor_record_offset":31812,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":35004,"parent_record_offset":34976,"ancestor_record_offset":31812,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":66,"record_offset":35032,"parent_record_offset":34976,"ancestor_record_offset":31812,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::impl$3::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3906
// 首条位置: {"module":197,"inlinee":14598}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":4992,"rva":5144592,"va":5373853712}
// 编译实例 RVAs: 0x4e8010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t8993138","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e872398900","size":5,"soft_fail":false,"target_va":5382847888,"va":5373854745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t8787621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a5168600","size":5,"soft_fail":false,"target_va":5382644528,"va":5373856902,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Sub$GT$::sub
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8889703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e867a58700","size":5,"soft_fail":false,"target_va":5382748928,"va":5373859220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4908145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e88f1bb5ff","size":5,"soft_fail":false,"target_va":5368953728,"va":5373861868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004c400
//     owner=unresolved_namespace; count=2
//     target_aliases=9 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4847802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e84607b6ff","size":5,"soft_fail":false,"target_va":5369021440,"va":5373869237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004ceb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4833088","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c040b6ff","size":5,"soft_fail":false,"target_va":5369024176,"va":5373857259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14004f690
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4827264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e88057b6ff","size":5,"soft_fail":false,"target_va":5369034384,"va":5373861643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400a2580
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4490077","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a37cbbff","size":5,"soft_fail":false,"target_va":5369374080,"va":5373864152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4238319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e81154bfff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373854426,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400fa190
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4127013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8db06c1ff","size":5,"soft_fail":false,"target_va":5369733520,"va":5373860528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400fb5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4122715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a517c1ff","size":5,"soft_fail":false,"target_va":5369738672,"va":5373861382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140161cd0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3697732","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8bc93c7ff","size":5,"soft_fail":false,"target_va":5370158288,"va":5373856015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1401647b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3686975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c1bdc7ff","size":5,"soft_fail":false,"target_va":5370169264,"va":5373856234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140294cf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2444162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e87eb4daff","size":5,"soft_fail":false,"target_va":5371415792,"va":5373859949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b7890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2296988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e864f3dcff","size":5,"soft_fail":false,"target_va":5371558032,"va":5373855015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1402b8400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2293945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e847ffdcff","size":5,"soft_fail":false,"target_va":5371560960,"va":5373854900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1240
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t943984","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e870670e00","size":5,"soft_fail":false,"target_va":5374808640,"va":5373864651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d1490
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t948851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8737a0e00","size":5,"soft_fail":false,"target_va":5374809232,"va":5373860376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405d5690
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t958409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c99f0e00","size":5,"soft_fail":false,"target_va":5374826128,"va":5373867714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405dab70
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t982384","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e870fd0e00","size":5,"soft_fail":false,"target_va":5374847856,"va":5373865467,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ee5e0
//     owner=unresolved_namespace; count=2
//     target_aliases=6 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1062495","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e85f361000","size":5,"soft_fail":false,"target_va":5374928352,"va":5373865852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ef210
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1070670","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e84e561000","size":5,"soft_fail":false,"target_va":5374931472,"va":5373860797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f2450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1075264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e840681000","size":5,"soft_fail":false,"target_va":5374944336,"va":5373869067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f5a40
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1090087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e827a21000","size":5,"soft_fail":false,"target_va":5374958144,"va":5373868052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405f9780
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1107993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e819e81000","size":5,"soft_fail":false,"target_va":5374973824,"va":5373865826,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405faa90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1110432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a0f11000","size":5,"soft_fail":false,"target_va":5374978704,"va":5373868267,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405fdb80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1128167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8e7361100","size":5,"soft_fail":false,"target_va":5374991232,"va":5373863060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405ffcc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1133520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8d04b1100","size":5,"soft_fail":false,"target_va":5374999744,"va":5373866219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140609f60
//     owner=unresolved_namespace; count=7
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1185337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e839161200","size":5,"soft_fail":false,"target_va":5375041376,"va":5373856034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1224156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8dcad1200","size":5,"soft_fail":false,"target_va":5375081504,"va":5373857343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=3
//     target_aliases=69 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1659333","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c5511900","size":5,"soft_fail":false,"target_va":5375516032,"va":5373856694,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5be90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6756795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8bb196700","size":5,"soft_fail":false,"target_va":5380619920,"va":5373863120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd63f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7260292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e884c86e00","size":5,"soft_fail":false,"target_va":5381121008,"va":5373860711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7308453","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a5846f00","size":5,"soft_fail":false,"target_va":5381170352,"va":5373861894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bfd9d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7421395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8d33d7100","size":5,"soft_fail":false,"target_va":5381282256,"va":5373860856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bff4a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7429816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8b85e7100","size":5,"soft_fail":false,"target_va":5381289120,"va":5373859299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37431834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e81a2a3b02","size":5,"soft_fail":false,"target_va":5411285568,"va":5373853729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428a1640
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37449932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8cc703b02","size":5,"soft_fail":false,"target_va":5411313216,"va":5373863279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428ce090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37632195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c3383e02","size":5,"soft_fail":false,"target_va":5411496080,"va":5373863880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=15
//     target_aliases=1 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37991682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e802b54302","size":5,"soft_fail":false,"target_va":5411847168,"va":5373855481,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t497173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e815960700","size":5,"soft_fail":false,"target_va":5374357296,"va":5373860118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37678238","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e89eec3e02","size":5,"soft_fail":false,"target_va":5411541584,"va":5373863341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37678421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e855ed3e02","size":5,"soft_fail":false,"target_va":5411541808,"va":5373863382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37678418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e852ed3e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5373863353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37678251","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8abec3e02","size":5,"soft_fail":false,"target_va":5411541904,"va":5373863648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37679347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8f3f03e02","size":5,"soft_fail":false,"target_va":5411542880,"va":5373863528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8c3faffff","size":5,"soft_fail":false,"target_va":5373852512,"va":5373853848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37689430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e856183f02","size":5,"soft_fail":false,"target_va":5411543360,"va":5373853925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::pass_builder::pass_resources::PassResources::new_from
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4323461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e885f84100","size":5,"soft_fail":false,"target_va":5378183952,"va":5373860486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5053346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a21b4d00","size":5,"soft_fail":false,"target_va":5378908992,"va":5373855641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5053662","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8de1c4d00","size":5,"soft_fail":false,"target_va":5378909360,"va":5373855693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5053832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8881d4d00","size":5,"soft_fail":false,"target_va":5378909504,"va":5373855667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7531173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8a5ea7200","size":5,"soft_fail":false,"target_va":5381385536,"va":5373854358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7536362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8eafe7200","size":5,"soft_fail":false,"target_va":5381390640,"va":5373854273,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7523284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8d4cb7200","size":5,"soft_fail":false,"target_va":5381386480,"va":5373863191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t7530877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e87de97200","size":5,"soft_fail":false,"target_va":5381384704,"va":5373853822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7401262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e82eef7000","size":5,"soft_fail":false,"target_va":5381263232,"va":5373861965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::wait_idle_and_recycle_resources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7405044","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8f4fd7000","size":5,"soft_fail":false,"target_va":5381262944,"va":5373857895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::profiler::TimedFrame::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7250667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8eba26e00","size":5,"soft_fail":false,"target_va":5381110960,"va":5373860288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::profiler::TimedFrame::send_to_puffin
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7251548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e85ca66e00","size":5,"soft_fail":false,"target_va":5381111856,"va":5373860303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::stats::RenderStats::snapshot_and_reset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7441893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8e58d7100","size":5,"soft_fail":false,"target_va":5381303712,"va":5373861814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::extent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7496312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e878627200","size":5,"soft_fail":false,"target_va":5381352080,"va":5373855763,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7495652","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8e45f7200","size":5,"soft_fail":false,"target_va":5381351392,"va":5373855735,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::Swapchain::present
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7491591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e807507200","size":5,"soft_fail":false,"target_va":5381353344,"va":5373861748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::TrackedSwapchainTextureLease::image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7492737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e881547200","size":5,"soft_fail":false,"target_va":5381353760,"va":5373861018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::swapchain::TrackedSwapchainTextureLease::into_present_source
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7492252","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e89c527200","size":5,"soft_fail":false,"target_va":5381353840,"va":5373861583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::ExecutedRenderGraph::retire
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7340582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e826027000","size":5,"soft_fail":false,"target_va":5381201152,"va":5373860565,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::ExportedRenderGraphResources::take_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7340341","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e835017000","size":5,"soft_fail":false,"target_va":5381201024,"va":5373860678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutionRecorder::end_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7349432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8b8247000","size":5,"soft_fail":false,"target_va":5381211136,"va":5373861699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::begin_frame
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7345270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e876147000","size":5,"soft_fail":false,"target_va":5381204944,"va":5373859669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7342661","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8450a7000","size":5,"soft_fail":false,"target_va":5381204704,"va":5373862038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::executor::GraphExecutor::wait_gpu_queue_trim
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7340563","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e813027000","size":5,"soft_fail":false,"target_va":5381203584,"va":5373863016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::resource_pool::ResourcePool::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7334459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e83bea6f00","size":5,"soft_fail":false,"target_va":5381191408,"va":5373856944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8800902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e8864a8600","size":5,"soft_fail":false,"target_va":5382657856,"va":5373856949,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t8896801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e821c18700","size":5,"soft_fail":false,"target_va":5382753792,"va":5373856986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37673770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e82adb3e02","size":5,"soft_fail":false,"target_va":5411537104,"va":5373863329,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t8886423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e897988700","size":5,"soft_fail":false,"target_va":5382748432,"va":5373862004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t8888412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e85ca08700","size":5,"soft_fail":false,"target_va":5382747616,"va":5373859199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tiny_glade::screenshot::finish_screenshot_capture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1881459","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373853712,"raw_hex":"e88d4ae3ff","size":5,"soft_fail":false,"target_va":5371980320,"va":5373861774,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":8476,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":18168,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":6530,"source_line":2503,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":18540,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] arrayvec::arrayvec::impl$10::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":18404,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14607,"source_line":886,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":197,"record_offset":11812,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":197,"record_offset":7660,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":8096,"source_line":2015,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::world::World::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":197,"record_offset":10600,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":9174,"source_line":2033,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":10392,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":11604,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":20208,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14614,"source_line":908,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":197,"record_offset":22248,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":10300,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":10488,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":10332,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":18104,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":19028,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":8786,"source_line":1784,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":19436,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":18332,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":13452,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":197,"record_offset":10564,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":19896,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":197,"record_offset":20236,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14615,"source_line":927,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::game_config::impl$11::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":9368,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14586,"source_line":114,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::render_loop::draw_passes::draw_lists::impl$7::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/draw_passes/draw_lists.rs
//     evidence={"module":197,"record_offset":17680,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14603,"source_line":111,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":197,"record_offset":11636,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":11520,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":6676,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":5244,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":5700,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::executor::ExportedRenderGraphResources::take_exported
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/executor.rs
//     evidence={"module":197,"record_offset":19000,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":8798,"source_line":95,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::impl$18::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":11868,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14540,"source_line":79,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::SmolRingBuffer::push
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":19760,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":9494,"source_line":939,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_loading_screen_render_graph
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":10524,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14601,"source_line":487,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::compile_main_render_graph
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":10448,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14600,"source_line":69,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] tiny_glade::render_loop::execute_presentation_render_graph
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":19172,"parent_record_offset":4992,"ancestor_record_offset":4992,"inlinee":14610,"source_line":409,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":4992,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":197,"record_offset":10020,"parent_record_offset":9996,"ancestor_record_offset":4992,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":8120,"parent_record_offset":8096,"ancestor_record_offset":4992,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":18860,"parent_record_offset":18832,"ancestor_record_offset":4992,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":9996,"parent_record_offset":9972,"ancestor_record_offset":4992,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":8168,"parent_record_offset":8144,"ancestor_record_offset":4992,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":8144,"parent_record_offset":8120,"ancestor_record_offset":4992,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":9972,"parent_record_offset":9948,"ancestor_record_offset":4992,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":20980,"parent_record_offset":20944,"ancestor_record_offset":4992,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":197,"record_offset":20944,"parent_record_offset":20920,"ancestor_record_offset":4992,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":18584,"parent_record_offset":18540,"ancestor_record_offset":4992,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":8096,"parent_record_offset":8072,"ancestor_record_offset":4992,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":8072,"parent_record_offset":8032,"ancestor_record_offset":4992,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":8032,"parent_record_offset":8008,"ancestor_record_offset":4992,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::get_unchecked_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":18464,"parent_record_offset":18404,"ancestor_record_offset":4992,"inlinee":14609,"source_line":259,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":197,"record_offset":18436,"parent_record_offset":18404,"ancestor_record_offset":4992,"inlinee":14608,"source_line":113,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":197,"record_offset":11836,"parent_record_offset":11812,"ancestor_record_offset":4992,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":4992,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":4992,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":8344,"parent_record_offset":8312,"ancestor_record_offset":4992,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":8288,"parent_record_offset":7752,"ancestor_record_offset":4992,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_with_ticks
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":11220,"parent_record_offset":10680,"ancestor_record_offset":4992,"inlinee":4166,"source_line":134,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":8312,"parent_record_offset":8288,"ancestor_record_offset":4992,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":7780,"parent_record_offset":7752,"ancestor_record_offset":4992,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":8372,"parent_record_offset":8312,"ancestor_record_offset":4992,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_with_ticks::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":11304,"parent_record_offset":11244,"ancestor_record_offset":4992,"inlinee":4176,"source_line":141,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":7828,"parent_record_offset":7804,"ancestor_record_offset":4992,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":7804,"parent_record_offset":7780,"ancestor_record_offset":4992,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":8008,"parent_record_offset":7984,"ancestor_record_offset":4992,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::World::read_change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/mod.rs
//     evidence={"module":197,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":4992,"inlinee":4180,"source_line":2979,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::change_tick
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":11396,"parent_record_offset":10680,"ancestor_record_offset":4992,"inlinee":4179,"source_line":301,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":7712,"parent_record_offset":7684,"ancestor_record_offset":4992,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":7684,"parent_record_offset":7660,"ancestor_record_offset":4992,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":7752,"parent_record_offset":7684,"ancestor_record_offset":4992,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":10624,"parent_record_offset":10600,"ancestor_record_offset":4992,"inlinee":9175,"source_line":490,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_mut_by_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":10680,"parent_record_offset":10624,"ancestor_record_offset":4992,"inlinee":4174,"source_line":515,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$40::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\mod.rs
//     evidence={"module":197,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":4992,"inlinee":13979,"source_line":455,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":7012,"parent_record_offset":6984,"ancestor_record_offset":4992,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":12436,"parent_record_offset":12404,"ancestor_record_offset":4992,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":4992,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":7040,"parent_record_offset":7012,"ancestor_record_offset":4992,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":6960,"parent_record_offset":6932,"ancestor_record_offset":4992,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":4992,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":7156,"parent_record_offset":6984,"ancestor_record_offset":4992,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":197,"record_offset":7268,"parent_record_offset":7244,"ancestor_record_offset":4992,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":20160,"parent_record_offset":20124,"ancestor_record_offset":4992,"inlinee":14614,"source_line":908,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":19988,"parent_record_offset":19952,"ancestor_record_offset":4992,"inlinee":14612,"source_line":930,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::f64::impl$0::to_bits
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":20368,"parent_record_offset":20268,"ancestor_record_offset":4992,"inlinee":14617,"source_line":1096,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":197,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":4992,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":4992,"inlinee":4812,"source_line":486,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":18892,"parent_record_offset":18860,"ancestor_record_offset":4992,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":19796,"parent_record_offset":19760,"ancestor_record_offset":4992,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":197,"record_offset":5400,"parent_record_offset":5368,"ancestor_record_offset":4992,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":12464,"parent_record_offset":12404,"ancestor_record_offset":4992,"inlinee":6331,"source_line":1980,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":197,"record_offset":6828,"parent_record_offset":6784,"ancestor_record_offset":4992,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":7944,"parent_record_offset":7920,"ancestor_record_offset":4992,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":7984,"parent_record_offset":7804,"ancestor_record_offset":4992,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":12156,"parent_record_offset":12128,"ancestor_record_offset":4992,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":8616,"parent_record_offset":8588,"ancestor_record_offset":4992,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":9612,"parent_record_offset":9516,"ancestor_record_offset":4992,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":197,"record_offset":11692,"parent_record_offset":11660,"ancestor_record_offset":4992,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":197,"record_offset":11720,"parent_record_offset":11660,"ancestor_record_offset":4992,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":5648,"parent_record_offset":5620,"ancestor_record_offset":4992,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":197,"record_offset":7616,"parent_record_offset":6784,"ancestor_record_offset":4992,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=73
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":7220,"parent_record_offset":6932,"ancestor_record_offset":4992,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":18500,"parent_record_offset":18464,"ancestor_record_offset":4992,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":12244,"parent_record_offset":12128,"ancestor_record_offset":4992,"inlinee":7403,"source_line":1581,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":197,"record_offset":13896,"parent_record_offset":13808,"ancestor_record_offset":4992,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":21012,"parent_record_offset":20980,"ancestor_record_offset":4992,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":4992,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":6088,"parent_record_offset":6064,"ancestor_record_offset":4992,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":197,"record_offset":6296,"parent_record_offset":6268,"ancestor_record_offset":4992,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":12320,"parent_record_offset":12292,"ancestor_record_offset":4992,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":12268,"parent_record_offset":12244,"ancestor_record_offset":4992,"inlinee":7404,"source_line":1212,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":7112,"parent_record_offset":7088,"ancestor_record_offset":4992,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":197,"record_offset":19224,"parent_record_offset":19172,"ancestor_record_offset":4992,"inlinee":5236,"source_line":771,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":7856,"parent_record_offset":7828,"ancestor_record_offset":4992,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":4992,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":7892,"parent_record_offset":7856,"ancestor_record_offset":4992,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":8212,"parent_record_offset":8032,"ancestor_record_offset":4992,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":6040,"parent_record_offset":6016,"ancestor_record_offset":4992,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":6064,"parent_record_offset":6040,"ancestor_record_offset":4992,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":6368,"parent_record_offset":6340,"ancestor_record_offset":4992,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":197,"record_offset":6016,"parent_record_offset":5900,"ancestor_record_offset":4992,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":197,"record_offset":5368,"parent_record_offset":5340,"ancestor_record_offset":4992,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":197,"record_offset":5728,"parent_record_offset":5700,"ancestor_record_offset":4992,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":197,"record_offset":5868,"parent_record_offset":5836,"ancestor_record_offset":4992,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":197,"record_offset":5836,"parent_record_offset":5800,"ancestor_record_offset":4992,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":5776,"parent_record_offset":5752,"ancestor_record_offset":4992,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":5800,"parent_record_offset":5776,"ancestor_record_offset":4992,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":5752,"parent_record_offset":5728,"ancestor_record_offset":4992,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":197,"record_offset":6340,"parent_record_offset":5800,"ancestor_record_offset":4992,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5340,"parent_record_offset":5308,"ancestor_record_offset":4992,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5308,"parent_record_offset":5280,"ancestor_record_offset":4992,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5500,"parent_record_offset":5468,"ancestor_record_offset":4992,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5596,"parent_record_offset":5500,"ancestor_record_offset":4992,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5468,"parent_record_offset":5444,"ancestor_record_offset":4992,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5280,"parent_record_offset":5244,"ancestor_record_offset":4992,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":197,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":4992,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":5900,"parent_record_offset":5868,"ancestor_record_offset":4992,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":6184,"parent_record_offset":5900,"ancestor_record_offset":4992,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":6156,"parent_record_offset":5900,"ancestor_record_offset":4992,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":197,"record_offset":5960,"parent_record_offset":5900,"ancestor_record_offset":4992,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":4992,"inlinee":4181,"source_line":2698,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":8504,"parent_record_offset":8476,"ancestor_record_offset":4992,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":11544,"parent_record_offset":11520,"ancestor_record_offset":4992,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":8528,"parent_record_offset":8504,"ancestor_record_offset":4992,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":11468,"parent_record_offset":11444,"ancestor_record_offset":4992,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":21040,"parent_record_offset":20980,"ancestor_record_offset":4992,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":21072,"parent_record_offset":20944,"ancestor_record_offset":4992,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::time::Duration::try_from_secs_f64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":197,"record_offset":20268,"parent_record_offset":20236,"ancestor_record_offset":4992,"inlinee":14616,"source_line":1678,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":10180,"parent_record_offset":9368,"ancestor_record_offset":4992,"inlinee":14592,"source_line":11,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::game_config::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/game_config.rs
//     evidence={"module":197,"record_offset":10208,"parent_record_offset":9368,"ancestor_record_offset":4992,"inlinee":14594,"source_line":68,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$13::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":9552,"parent_record_offset":9516,"ancestor_record_offset":4992,"inlinee":10688,"source_line":16,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$17::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":9640,"parent_record_offset":9612,"ancestor_record_offset":4992,"inlinee":10692,"source_line":33,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$22::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":9516,"parent_record_offset":9436,"ancestor_record_offset":4992,"inlinee":10686,"source_line":48,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$38::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":9436,"parent_record_offset":9368,"ancestor_record_offset":4992,"inlinee":10684,"source_line":99,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::render_config::impl$9::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/render_config.rs
//     evidence={"module":197,"record_offset":9580,"parent_record_offset":9552,"ancestor_record_offset":4992,"inlinee":10690,"source_line":7,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_settings::impl$0::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_settings.rs
//     evidence={"module":197,"record_offset":10116,"parent_record_offset":9368,"ancestor_record_offset":4992,"inlinee":14588,"source_line":4,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::ui_settings::impl$3::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/ui_settings.rs
//     evidence={"module":197,"record_offset":10144,"parent_record_offset":9368,"ancestor_record_offset":4992,"inlinee":14590,"source_line":5,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13808,"parent_record_offset":13776,"ancestor_record_offset":4992,"inlinee":6131,"source_line":2973,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":4992,"inlinee":6201,"source_line":2124,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13776,"parent_record_offset":13720,"ancestor_record_offset":4992,"inlinee":6130,"source_line":2943,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13748,"parent_record_offset":13720,"ancestor_record_offset":4992,"inlinee":6139,"source_line":2509,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13840,"parent_record_offset":13808,"ancestor_record_offset":4992,"inlinee":4811,"source_line":162,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":197,"record_offset":13408,"parent_record_offset":13384,"ancestor_record_offset":4992,"inlinee":6180,"source_line":3306,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":11748,"parent_record_offset":11660,"ancestor_record_offset":4992,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":197,"record_offset":11660,"parent_record_offset":11636,"ancestor_record_offset":4992,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":4992,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":6712,"parent_record_offset":6676,"ancestor_record_offset":4992,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":21172,"parent_record_offset":21140,"ancestor_record_offset":4992,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":21688,"parent_record_offset":21428,"ancestor_record_offset":4992,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":197,"record_offset":6856,"parent_record_offset":6828,"ancestor_record_offset":4992,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":6932,"parent_record_offset":6784,"ancestor_record_offset":4992,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":197,"record_offset":7192,"parent_record_offset":6932,"ancestor_record_offset":4992,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::init::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/init.rs
//     evidence={"module":197,"record_offset":9768,"parent_record_offset":9436,"ancestor_record_offset":4992,"inlinee":10696,"source_line":93,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":197,"record_offset":12604,"parent_record_offset":11908,"ancestor_record_offset":4992,"inlinee":8093,"source_line":300,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":197,"record_offset":11908,"parent_record_offset":11868,"ancestor_record_offset":4992,"inlinee":7397,"source_line":1406,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":197,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":4992,"inlinee":4794,"source_line":56,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__init
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":197,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":4992,"inlinee":7402,"source_line":83,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":197,"record_offset":12068,"parent_record_offset":12040,"ancestor_record_offset":4992,"inlinee":7399,"source_line":97,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":197,"record_offset":12404,"parent_record_offset":12004,"ancestor_record_offset":4992,"inlinee":4797,"source_line":72,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":197,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":4992,"inlinee":4792,"source_line":151,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":197,"record_offset":6884,"parent_record_offset":6856,"ancestor_record_offset":4992,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::initialize
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":197,"record_offset":12128,"parent_record_offset":12096,"ancestor_record_offset":4992,"inlinee":7400,"source_line":61,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":197,"record_offset":6784,"parent_record_offset":6748,"ancestor_record_offset":4992,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":197,"record_offset":6748,"parent_record_offset":6712,"ancestor_record_offset":4992,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::impl$3::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":10416,"parent_record_offset":10392,"ancestor_record_offset":4992,"inlinee":14598,"source_line":617,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":4992,"inlinee":14596,"source_line":668,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":18136,"parent_record_offset":18104,"ancestor_record_offset":4992,"inlinee":14604,"source_line":769,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":18368,"parent_record_offset":18332,"ancestor_record_offset":4992,"inlinee":14605,"source_line":778,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":19952,"parent_record_offset":19896,"ancestor_record_offset":4992,"inlinee":14611,"source_line":890,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] tiny_glade::render_loop::render::closure$5
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tiny-glade","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tiny-glade/src/render_loop.rs
//     evidence={"module":197,"record_offset":20124,"parent_record_offset":20036,"ancestor_record_offset":4992,"inlinee":14613,"source_line":895,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$16::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":197,"record_offset":9736,"parent_record_offset":9708,"ancestor_record_offset":4992,"inlinee":10694,"source_line":437,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 668
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3904
// 首条位置: {"module":197,"inlinee":14596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 769
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x390C
// 首条位置: {"module":197,"inlinee":14604}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 778
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x390D
// 首条位置: {"module":197,"inlinee":14605}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 890
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3913
// 首条位置: {"module":197,"inlinee":14611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":19988,"parent_record_offset":19952,"ancestor_record_offset":19952,"inlinee":14612,"source_line":930,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 895
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x3915
// 首条位置: {"module":197,"inlinee":14613}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f64::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f64.rs
//     evidence={"module":197,"record_offset":20160,"parent_record_offset":20124,"ancestor_record_offset":20124,"inlinee":14614,"source_line":908,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::handle_screenshot_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 917
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":23244,"rva":5160880,"va":5373870000}
// 编译实例 RVAs: 0x4ebfb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8774244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870000,"raw_hex":"e864e28500","size":5,"soft_fail":false,"target_va":5382644528,"va":5373870279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1645887","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870000,"raw_hex":"e83f1d1900","size":5,"soft_fail":false,"target_va":5375516032,"va":5373870140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8784731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870000,"raw_hex":"e85b0b8600","size":5,"soft_fail":false,"target_va":5382655024,"va":5373870288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":197,"record_offset":23472,"parent_record_offset":23244,"ancestor_record_offset":23244,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":23680,"parent_record_offset":23244,"ancestor_record_offset":23244,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":197,"record_offset":23504,"parent_record_offset":23244,"ancestor_record_offset":23244,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":23388,"parent_record_offset":23244,"ancestor_record_offset":23244,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":197,"record_offset":23560,"parent_record_offset":23528,"ancestor_record_offset":23244,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":197,"record_offset":23588,"parent_record_offset":23528,"ancestor_record_offset":23244,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":23244,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":197,"record_offset":23436,"parent_record_offset":23412,"ancestor_record_offset":23244,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":23616,"parent_record_offset":23528,"ancestor_record_offset":23244,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":197,"record_offset":23528,"parent_record_offset":23504,"ancestor_record_offset":23244,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":197,"record_offset":23640,"parent_record_offset":23616,"ancestor_record_offset":23244,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::SmolRingBuffer::push
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 939
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2516
// 首条位置: {"module":197,"inlinee":9494}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::num::impl$11::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":197,"record_offset":19796,"parent_record_offset":19760,"ancestor_record_offset":19760,"inlinee":4914,"source_line":1980,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: tiny_glade::render_loop::render_world
// 记录形式: compiled_procedure; instances=1
// 源码行: 955
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":197,"record_offset":23740,"rva":5161232,"va":5373870352}
// 编译实例 RVAs: 0x4ec110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dd6f0
//     owner=unresolved_namespace; count=4
//     target_aliases=19 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4254362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e86615bfff","size":5,"soft_fail":false,"target_va":5369616112,"va":5373870469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400fb890
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4133975","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8a9ebc0ff","size":5,"soft_fail":false,"target_va":5369739408,"va":5373873378,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d8bd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1128154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e826c9eeff","size":5,"soft_fail":false,"target_va":5372742608,"va":5373870757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d90f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1126884","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e81cceeeff","size":5,"soft_fail":false,"target_va":5372743920,"va":5373870799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d9610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1125586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e82ed3eeff","size":5,"soft_fail":false,"target_va":5372745232,"va":5373870813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403d9b30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1124288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e840d8eeff","size":5,"soft_fail":false,"target_va":5372746544,"va":5373870827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da050
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1122920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e898ddeeff","size":5,"soft_fail":false,"target_va":5372747856,"va":5373870771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da570
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1121622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8aae2eeff","size":5,"soft_fail":false,"target_va":5372749168,"va":5373870785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f6e50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8ebab0000","size":5,"soft_fail":false,"target_va":5373914704,"va":5373870688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1404f7450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t44030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8feab0000","size":5,"soft_fail":false,"target_va":5373916240,"va":5373872205,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ae8320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6275376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e830c15f00","size":5,"soft_fail":false,"target_va":5380145952,"va":5373870571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd6ce0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: render_loop.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7252130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8a2a86e00","size":5,"soft_fail":false,"target_va":5381123296,"va":5373871161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t8780440","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e898fa8500","size":5,"soft_fail":false,"target_va":5382655024,"va":5373874579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37667401","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e849c23e02","size":5,"soft_fail":false,"target_va":5411541776,"va":5373874370,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t37667701","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e875c33e02","size":5,"soft_fail":false,"target_va":5411542064,"va":5373874358,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37668412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e83cc63e02","size":5,"soft_fail":false,"target_va":5411542880,"va":5373874463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5559400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e868d45400","size":5,"soft_fail":false,"target_va":5379432048,"va":5373872643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5561107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e813db5400","size":5,"soft_fail":false,"target_va":5379434224,"va":5373873112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::apply_volumetrics::apply_volumetrics
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3897517","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8ad783b00","size":5,"soft_fail":false,"target_va":5377770688,"va":5373873166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::bin_lights::bin_lights
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5221802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8aaad4f00","size":5,"soft_fail":false,"target_va":5379094384,"va":5373872577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5463511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8d75d5300","size":5,"soft_fail":false,"target_va":5379334464,"va":5373870948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5466091","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8eb675300","size":5,"soft_fail":false,"target_va":5379337088,"va":5373870992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_path_masks::compute_path_masks
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4280717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e88d514100","size":5,"soft_fail":false,"target_va":5378151648,"va":5373870926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_rasterize_from_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5399429","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e885635200","size":5,"soft_fail":false,"target_va":5379270272,"va":5373870838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::compute_water::water_spawn_vegetation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5404156","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8fc755200","size":5,"soft_fail":false,"target_va":5379275120,"va":5373870959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::displace_rocky_terrain::displace_rocky_terrain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5581679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e86f2b5500","size":5,"soft_fail":false,"target_va":5379452544,"va":5373870860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof::calc_dof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5691490","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e862d85600","size":5,"soft_fail":false,"target_va":5379564912,"va":5373873417,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof::dof_prepare
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5690357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8f5d35600","size":5,"soft_fail":false,"target_va":5379563552,"va":5373873190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5419604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e854b25200","size":5,"soft_fail":false,"target_va":5379291264,"va":5373871655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5604127","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e81f835500","size":5,"soft_fail":false,"target_va":5379476688,"va":5373872556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4440383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e83fc14300","size":5,"soft_fail":false,"target_va":5378311920,"va":5373871532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4843300","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e824e74900","size":5,"soft_fail":false,"target_va":5378714352,"va":5373871047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4844753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8d1ec4900","size":5,"soft_fail":false,"target_va":5378716320,"va":5373871562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4831734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8f6b94900","size":5,"soft_fail":false,"target_va":5378702800,"va":5373871061,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::grass::grass_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5717140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8943c5700","size":5,"soft_fail":false,"target_va":5379588736,"va":5373871591,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::half_res::extract_half_res_depth_normals
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4234011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e81b9b4000","size":5,"soft_fail":false,"target_va":5378105728,"va":5373871712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::highlight_active_terrain_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5685523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e813c15600","size":5,"soft_fail":false,"target_va":5379556432,"va":5373870904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::leaf_particles::leaf_particles_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4229461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e855894000","size":5,"soft_fail":false,"target_va":5378100480,"va":5373871014,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_depth_value::mouse_depth_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5328870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8e64f5100","size":5,"soft_fail":false,"target_va":5379200544,"va":5373871669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4760969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e889a54800","size":5,"soft_fail":false,"target_va":5378631856,"va":5373870882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4639696","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8d0cb4600","size":5,"soft_fail":false,"target_va":5378510704,"va":5373871003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5530642","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e812645400","size":5,"soft_fail":false,"target_va":5379401584,"va":5373870937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4042266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e81aae3d00","size":5,"soft_fail":false,"target_va":5377913120,"va":5373870849,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5329765","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e865535100","size":5,"soft_fail":false,"target_va":5379202288,"va":5373872518,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::read_bush_positions_for_cpu
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3645878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8b6a13700","size":5,"soft_fail":false,"target_va":5377516864,"va":5373870981,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4850532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e864034a00","size":5,"soft_fail":false,"target_va":5378721408,"va":5373870871,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::reprojection::calculate_reprojection_map
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5376690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8b20a5200","size":5,"soft_fail":false,"target_va":5379248528,"va":5373871833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5780694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8d6345800","size":5,"soft_fail":false,"target_va":5379653312,"va":5373872613,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5494618","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e85ad75300","size":5,"soft_fail":false,"target_va":5379365648,"va":5373871025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::shader_refactoring::render_shader_refactoring
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4860370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8d2294a00","size":5,"soft_fail":false,"target_va":5378734576,"va":5373874201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4163905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e841893f00","size":5,"soft_fail":false,"target_va":5378034880,"va":5373870970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3701923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8a37c3800","size":5,"soft_fail":false,"target_va":5377573808,"va":5373871880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::stairs::stairs
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5752463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e88fc65700","size":5,"soft_fail":false,"target_va":5379623504,"va":5373871036,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::taa::calculate_taa
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5563575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8b7e45400","size":5,"soft_fail":false,"target_va":5379437296,"va":5373873716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::taa::calculate_tgsr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5565036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e86cea5400","size":5,"soft_fail":false,"target_va":5379439200,"va":5373874159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::terrain_lighting_texture::calc_terrain_lighting_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5656646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e846505600","size":5,"soft_fail":false,"target_va":5379527744,"va":5373871093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::compute_passes::visibility_queries::run_visibility_queries
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5516482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8c22c5400","size":5,"soft_fail":false,"target_va":5379388128,"va":5373871641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5133892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e844564e00","size":5,"soft_fail":false,"target_va":5379006640,"va":5373872743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5108862","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e87ef44d00","size":5,"soft_fail":false,"target_va":5378979760,"va":5373870893,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3452256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e860ad3400","size":5,"soft_fail":false,"target_va":5377323952,"va":5373871691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3438134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e836763400","size":5,"soft_fail":false,"target_va":5377309632,"va":5373871493,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3445815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e837943400","size":5,"soft_fail":false,"target_va":5377317440,"va":5373871620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5171032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e858e74e00","size":5,"soft_fail":false,"target_va":5379041952,"va":5373870915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5111425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e881fe4d00","size":5,"soft_fail":false,"target_va":5378982512,"va":5373871082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5075104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8a0704d00","size":5,"soft_fail":false,"target_va":5378949296,"va":5373874187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5074606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8ae6e4d00","size":5,"soft_fail":false,"target_va":5378947712,"va":5373873101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t5056481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8e1274d00","size":5,"soft_fail":false,"target_va":5378929440,"va":5373872954,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::extent_2d
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7245723","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e89b8f6e00","size":5,"soft_fail":false,"target_va":5381117840,"va":5373872112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::format
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7246340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e804926e00","size":5,"soft_fail":false,"target_va":5381116976,"va":5373870631,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::new_2d
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7245457","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8918e6e00","size":5,"soft_fail":false,"target_va":5381116576,"va":5373871114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::sampled
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7244721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8b18b6e00","size":5,"soft_fail":false,"target_va":5381116784,"va":5373872058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::storage
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7244735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8bf8b6e00","size":5,"soft_fail":false,"target_va":5381116816,"va":5373872076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::texture::TextureDesc::usage
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7246076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e8fc906e00","size":5,"soft_fail":false,"target_va":5381116736,"va":5373870655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::graph::RenderGraph::create_raw_resource
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7327850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373870352,"raw_hex":"e86ad06f00","size":5,"soft_fail":false,"target_va":5381199056,"va":5373871201,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%r10","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5373870352,"raw_hex":"41ffd2","size":3,"soft_fail":false,"va":5373873046,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":197,"record_offset":24760,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::is_winter
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":197,"record_offset":24732,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":7619,"source_line":274,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] country_core::utils::get_res
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/utils/mod.rs
//     evidence={"module":197,"record_offset":23888,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":10012,"source_line":34,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::recorder::RenderGraphRecorder::create
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/recorder.rs
//     evidence={"module":197,"record_offset":24832,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":9694,"source_line":34,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::resource::Handle::desc
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/resource.rs
//     evidence={"module":197,"record_offset":25920,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":8273,"source_line":315,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::as_ref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":26056,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":14620,"source_line":146,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::temporal::Temporal::split_pair
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/temporal.rs
//     evidence={"module":197,"record_offset":26024,"parent_record_offset":23740,"ancestor_record_offset":23740,"inlinee":14619,"source_line":158,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":24348,"parent_record_offset":24324,"ancestor_record_offset":23740,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":24396,"parent_record_offset":24372,"ancestor_record_offset":23740,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":197,"record_offset":24372,"parent_record_offset":24348,"ancestor_record_offset":23740,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":24324,"parent_record_offset":24300,"ancestor_record_offset":23740,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":24300,"parent_record_offset":24260,"ancestor_record_offset":23740,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":197,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":23740,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_ptr
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":24648,"parent_record_offset":24624,"ancestor_record_offset":23740,"inlinee":4178,"source_line":348,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_unchecked
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":24624,"parent_record_offset":24600,"ancestor_record_offset":23740,"inlinee":4177,"source_line":317,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_vec.rs
//     evidence={"module":197,"record_offset":24572,"parent_record_offset":24540,"ancestor_record_offset":23740,"inlinee":4169,"source_line":87,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::get_data
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":24516,"parent_record_offset":23980,"ancestor_record_offset":23740,"inlinee":8105,"source_line":107,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::ResourceData::is_present
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":24540,"parent_record_offset":24516,"ancestor_record_offset":23740,"inlinee":4167,"source_line":91,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::Resources::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":24008,"parent_record_offset":23980,"ancestor_record_offset":23740,"inlinee":4137,"source_line":341,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::resource::impl$1::get_data::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/resource.rs
//     evidence={"module":197,"record_offset":24600,"parent_record_offset":24540,"ancestor_record_offset":23740,"inlinee":8107,"source_line":108,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":24056,"parent_record_offset":24032,"ancestor_record_offset":23740,"inlinee":4141,"source_line":58,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::SparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":24032,"parent_record_offset":24008,"ancestor_record_offset":23740,"inlinee":4139,"source_line":395,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$15::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":197,"record_offset":24236,"parent_record_offset":24212,"ancestor_record_offset":23740,"inlinee":4151,"source_line":396,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::components
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":23944,"parent_record_offset":23916,"ancestor_record_offset":23740,"inlinee":8271,"source_line":271,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":23916,"parent_record_offset":23888,"ancestor_record_offset":23740,"inlinee":8097,"source_line":378,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell::get_resource_by_id
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/world/unsafe_world_cell.rs
//     evidence={"module":197,"record_offset":23980,"parent_record_offset":23916,"ancestor_record_offset":23740,"inlinee":8104,"source_line":432,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":197,"record_offset":24896,"parent_record_offset":24832,"ancestor_record_offset":23740,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":197,"record_offset":24148,"parent_record_offset":24084,"ancestor_record_offset":23740,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":24172,"parent_record_offset":24148,"ancestor_record_offset":23740,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":24212,"parent_record_offset":24032,"ancestor_record_offset":23740,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":24700,"parent_record_offset":23888,"ancestor_record_offset":23740,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$39::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":197,"record_offset":25140,"parent_record_offset":25112,"ancestor_record_offset":23740,"inlinee":4927,"source_line":2486,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":197,"record_offset":24800,"parent_record_offset":24760,"ancestor_record_offset":23740,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":197,"record_offset":24084,"parent_record_offset":24056,"ancestor_record_offset":23740,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":24464,"parent_record_offset":24440,"ancestor_record_offset":23740,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":24120,"parent_record_offset":24084,"ancestor_record_offset":23740,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":197,"record_offset":24440,"parent_record_offset":24260,"ancestor_record_offset":23740,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::texture::impl$26::clone
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":197,"record_offset":24868,"parent_record_offset":24832,"ancestor_record_offset":23740,"inlinee":9664,"source_line":67,"archive":"_pdb/B/modules/00197.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::util::impl$0::same
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/rg/util.rs
//     evidence={"module":197,"record_offset":24924,"parent_record_offset":24832,"ancestor_record_offset":23740,"inlinee":9666,"source_line":8,"archive":"_pdb/B/modules/00197.jsonl.gz"}
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
// 完整结构及原始类型记录: render_loop.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] tiny_glade::render_loop::SmolRingBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: render_loop.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === RENDER PASSES V1 BEGIN ===
// 渲染 Pass: crates/tiny-glade/src/render_loop.rs
// 范围: PDB/API 事实、Shader 声明、映射候选、未知分别标注。
// 完整记录: render_loop.rs.render-pass.json.gz
// 输入/输出: 具体参数、运行时资源句柄及完整返回资源 = unknown；可确认的 API 操作及 Shader 声明见下方。
// 资源依赖: 跨 Pass 实际 producer/consumer、资源版本、队列/屏障参数 = unknown。
// 调用顺序: 只记录同一物理父函数内唯一 PDB 目标名称的 callsite 地址排列；分支、循环和 GPU 执行先后 = unknown。
// 同地址别名目标候选 0 条：全部保留在附档，但是否调用本 Pass = unknown，不列为已确认 Pass 顺序边。
// Shader 二进制文件映射 = unknown（没有足够名称/类型关联；不按效果名称强行绑定）。
// [父函数静态调用位置] {"caller_entry_rva":881728,"caller_aliases":{"alias_group_rva":881728,"count":1,"display_examples":["std::sys::backtrace::__rust_begin_short_backtrace"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":883301,"position_among_mapped_render_targets":1,"mapped_render_target_count":1,"previous_callsite":null,"next_callsite":null,"previous_target_files":[],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1452793,"position_among_mapped_render_targets":4,"mapped_render_target_count":25,"previous_callsite":1452141,"next_callsite":1453902,"previous_target_files":["crates/country-core/src/systems/render_loop/compute_passes/primary_rt_debug.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/compute_passes/blur_pyramid.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1454247,"position_among_mapped_render_targets":7,"mapped_render_target_count":25,"previous_callsite":1454123,"next_callsite":1454313,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/final_post.rs"],"next_target_files":["crates/tiny-glade/src/render_loop.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1454313,"position_among_mapped_render_targets":8,"mapped_render_target_count":25,"previous_callsite":1454247,"next_callsite":1454349,"previous_target_files":["crates/tiny-glade/src/render_loop.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/menu_background.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1454864,"position_among_mapped_render_targets":13,"mapped_render_target_count":25,"previous_callsite":1454803,"next_callsite":1454909,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/logo_overlay.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/session_selection/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455274,"position_among_mapped_render_targets":22,"mapped_render_target_count":25,"previous_callsite":1455213,"next_callsite":1455317,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/custom_cursor_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":1449168,"caller_aliases":{"alias_group_rva":1449168,"count":1,"display_examples":["rhapsody::rg::state::RenderGraphState::compile"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":1455431,"position_among_mapped_render_targets":24,"mapped_render_target_count":25,"previous_callsite":1455317,"next_callsite":1455607,"previous_target_files":["crates/country-core/src/systems/render_loop/draw_passes/ui_pass.rs"],"next_target_files":["crates/country-core/src/systems/render_loop/mod.rs"],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// [父函数静态调用位置] {"caller_entry_rva":5084544,"caller_aliases":{"alias_group_rva":5084544,"count":1,"display_examples":["tiny_glade::game_loop::GameLoop::main_loop"],"complete_catalog":"_pdb/B/render-address-aliases.json.gz","exclusive_symbol":"unknown"},"callsite_rva":5105084,"position_among_mapped_render_targets":2,"mapped_render_target_count":2,"previous_callsite":5087825,"next_callsite":null,"previous_target_files":["crates/country-core/src/systems/render_loop/vermeer.rs"],"next_target_files":[],"ordering_status":"instruction_address_order_only; branches/loops/GPU schedule unknown","kind":"direct_call"}
// 全部直接引用方/目标别名、原指令字节及位置见同名 render-pass.json.gz；没有裁定同地址唯一符号。
// Shader 方程、启用条件、实际执行和完整渲染器实现 = unknown。
// === RENDER PASSES V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: render_loop.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::SmolRingBuffer::push
// 信息 ID: 4807831a2ca7297fed371e80
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=19760; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=19832; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph
// 信息 ID: 3490ab5c19ff3f83ecf24caf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=10524; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0
// 信息 ID: 06f79186a0fb4ae5dd5aff71
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_loading_screen_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0 <- rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=28068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_loading_screen_render_graph::closure$0::closure$0
// 信息 ID: 431dbaae1bfd35dbf13e3688
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_loading_screen_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=31812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph
// 信息 ID: e3a8e7e720f1f800f97e6385
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=10448; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=17736; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0
// 信息 ID: d76d96e3865530aa328098ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::compile_main_render_graph::closure_env$0 <- rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=5312; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$0
// 信息 ID: e0a9771ec197feb94e1a8a0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=16940; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::closure$1
// 信息 ID: 3a77882cdfa25bfc3e363e29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=17140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::compile_main_render_graph::closure$0::explain
// 信息 ID: 29ce18400bbc84ad6dfa5bf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::compile_main_render_graph", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=14640; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=15804; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=24612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::execute_presentation_render_graph
// 信息 ID: 187830c1a4423403c4bf0d7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=19172; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::execute_presentation_render_graph::closure$0
// 信息 ID: 869fcac0162900c43d8b0912
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::execute_presentation_render_graph", "captures": "unknown"}
// [闭包环境类型引用] tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0 <- rhapsody::rg::executor::GraphExecutionRecorder::execute<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0 <- rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=42268; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::handle_screenshot_error
// 信息 ID: 077aa43fc2eb8c1861193349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1630a7 -> 0x4ebfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1630e9 -> 0x4ebfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x1634aa -> 0x4ebfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163547 -> 0x4ebfb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x163310 -> 0x4ebfb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::impl$3::eq
// 信息 ID: ed4531dde30b5b01db4e1731
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=10416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render
// 信息 ID: 0efc356c60e6207991458152
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a65 -> 0x4e8010 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::game_loop::GameLoop::main_loop @ 0x4de5bc -> 0x4e8010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$0
// 信息 ID: 08530bd99901559e3dcddd00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=10360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$1
// 信息 ID: fae1e9638d91badc6a144672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=18136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$2
// 信息 ID: 9ee432199fd8b99b0384fb38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=18368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$4
// 信息 ID: 11d60a58ba91eb8b896f0e7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=19952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render::closure$5
// 信息 ID: 43982af3f1c14469e566d4d8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "tiny_glade::render_loop::render", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=20124; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: tiny_glade::render_loop::render_world
// 信息 ID: ad67590cf04a42fdd158cb29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x162af9 -> 0x4ec110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
