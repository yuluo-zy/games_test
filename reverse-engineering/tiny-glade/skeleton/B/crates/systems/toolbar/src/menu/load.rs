// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/toolbar/src/menu/load.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1535c7ca118a72d02029724616440be6cc660cfbf4f46d40e4adb5cd4527be0c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1181, 1205, 1225, 1243, 2753, 2759, 2762, 2845, 3956, 3963, 4539
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_state; // namespace candidate
//   package identities: [{"name":"bevy_state","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use lang_lib; // namespace candidate
//   package identities: [{"name":"lang-lib","source":"local_or_path","version":"0.1.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_text; // namespace candidate
//   package identities: [{"name":"tg-text","source":"local_or_path","version":"0.1.0"}]
// use time; // namespace candidate
//   package identities: [{"name":"time","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.3.36"}]
// use toolbar; // namespace candidate
//   package identities: [{"name":"bitflags","source":"PDB_registry_path_only","version":"1.3.2"},{"name":"toolbar","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// use uuid; // namespace candidate
//   package identities: [{"name":"uuid","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.15.1"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$3::render
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":5396,"rva":31585472,"va":5400294592}
// 编译实例 RVAs: 0x1e1f4c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25214906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e846407ffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400296405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_const::panic_const_div_by_zero
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11246850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e8029dab00","size":5,"soft_fail":false,"target_va":5411543520,"va":5400296665,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::gui::LoadMenuThumbnailMetrics::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21888213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e82b03b2fe","size":5,"soft_fail":false,"target_va":5378407536,"va":5400295744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11195727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e84fd5aa00","size":5,"soft_fail":false,"target_va":5411492368,"va":5400296636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t11198144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e8c0deaa00","size":5,"soft_fail":false,"target_va":5411494800,"va":5400296651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18472910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e83220e6fe","size":5,"soft_fail":false,"target_va":5381823600,"va":5400296505,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18472542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e8a221e6fe","size":5,"soft_fail":false,"target_va":5381824016,"va":5400296553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp_mut
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-18411298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400294592,"raw_hex":"e8de10e7fe","size":5,"soft_fail":false,"target_va":5381883504,"va":5400294797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2762,"record_offset":7364,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":5792,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2762,"record_offset":5664,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2762,"record_offset":5848,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2762,"record_offset":5692,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":2762,"record_offset":5820,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2762,"record_offset":6252,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":5764,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$8::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":5720,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":8810,"source_line":3197,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":6976,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":7004,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec2::BVec2::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec2.rs
//     evidence={"module":2762,"record_offset":6944,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":28496,"source_line":71,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::bool::bvec2::impl$2::bitand
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/bvec2.rs
//     evidence={"module":2762,"record_offset":6916,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":19943,"source_line":123,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmpge
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6788,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":19939,"source_line":280,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::cmple
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6844,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":19941,"source_line":302,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$18::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6108,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":10968,"source_line":1148,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6520,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6716,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$40::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":5996,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":19989,"source_line":1333,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6660,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$8::div
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":6052,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":19932,"source_line":1063,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2762,"record_offset":7392,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2762,"record_offset":5604,"parent_record_offset":5396,"ancestor_record_offset":5396,"inlinee":14896,"source_line":603,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":5396,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":7276,"parent_record_offset":7180,"ancestor_record_offset":5396,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":7212,"parent_record_offset":7180,"ancestor_record_offset":5396,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":7180,"parent_record_offset":7152,"ancestor_record_offset":5396,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":7152,"parent_record_offset":7124,"ancestor_record_offset":5396,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2762,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":5396,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2762,"record_offset":7100,"parent_record_offset":7076,"ancestor_record_offset":5396,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":2762,"record_offset":6312,"parent_record_offset":6252,"ancestor_record_offset":5396,"inlinee":4921,"source_line":208,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::ge
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":5396,"inlinee":19940,"source_line":1790,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":6876,"parent_record_offset":6844,"ancestor_record_offset":5396,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":7244,"parent_record_offset":7212,"ancestor_record_offset":5396,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2762,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5396,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2762,"record_offset":6136,"parent_record_offset":6108,"ancestor_record_offset":5396,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$241::div
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2762,"record_offset":6076,"parent_record_offset":6052,"ancestor_record_offset":5396,"inlinee":9344,"source_line":502,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2762,"record_offset":6164,"parent_record_offset":6108,"ancestor_record_offset":5396,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":7028,"parent_record_offset":7004,"ancestor_record_offset":5396,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":6424,"parent_record_offset":6312,"ancestor_record_offset":5396,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":6376,"parent_record_offset":6312,"ancestor_record_offset":5396,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":6464,"parent_record_offset":6424,"ancestor_record_offset":5396,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":7668,"parent_record_offset":7604,"ancestor_record_offset":5396,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":7456,"parent_record_offset":7420,"ancestor_record_offset":5396,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":5396,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":7480,"parent_record_offset":7456,"ancestor_record_offset":5396,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":2762,"record_offset":7576,"parent_record_offset":7548,"ancestor_record_offset":5396,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2762,"record_offset":7420,"parent_record_offset":7392,"ancestor_record_offset":5396,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":2762,"record_offset":7604,"parent_record_offset":7576,"ancestor_record_offset":5396,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2762,"record_offset":7512,"parent_record_offset":7420,"ancestor_record_offset":5396,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":2762,"record_offset":7640,"parent_record_offset":7604,"ancestor_record_offset":5396,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":184240,"rva":16542688,"va":5385251808}
// 编译实例 RVAs: 0xfc6be0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=7
//     target_aliases=7 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10575533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e853a15eff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385251864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9de80
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-167584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e86071fdff","size":5,"soft_fail":false,"target_va":5385084544,"va":5385252123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa0130
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-158694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e81a94fdff","size":5,"soft_fail":false,"target_va":5385093424,"va":5385252113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26287872","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e8001f9101","size":5,"soft_fail":false,"target_va":5411541808,"va":5385253931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26289403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e8fb249101","size":5,"soft_fail":false,"target_va":5411541776,"va":5385252368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26288246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e876209101","size":5,"soft_fail":false,"target_va":5411542296,"va":5385254045,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::widgets::PillButtonView::size
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9722455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251808,"raw_hex":"e8a9a56bff","size":5,"soft_fail":false,"target_va":5375530288,"va":5385252738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":190620,"parent_record_offset":184240,"ancestor_record_offset":184240,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":184396,"parent_record_offset":184240,"ancestor_record_offset":184240,"inlinee":20053,"source_line":374,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::inactive_opacity
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":185572,"parent_record_offset":184240,"ancestor_record_offset":184240,"inlinee":19879,"source_line":1013,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::with_node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":185660,"parent_record_offset":184240,"ancestor_record_offset":184240,"inlinee":62093,"source_line":230,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$53::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":188316,"parent_record_offset":184240,"ancestor_record_offset":184240,"inlinee":62054,"source_line":192,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":185240,"parent_record_offset":185216,"ancestor_record_offset":184240,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":184748,"parent_record_offset":184724,"ancestor_record_offset":184240,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":184832,"parent_record_offset":184808,"ancestor_record_offset":184240,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":184772,"parent_record_offset":184748,"ancestor_record_offset":184240,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":185308,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":184980,"parent_record_offset":184952,"ancestor_record_offset":184240,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":184952,"parent_record_offset":184924,"ancestor_record_offset":184240,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":193320,"parent_record_offset":193296,"ancestor_record_offset":184240,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":185540,"parent_record_offset":184396,"ancestor_record_offset":184240,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":185100,"parent_record_offset":185076,"ancestor_record_offset":184240,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":185728,"parent_record_offset":185696,"ancestor_record_offset":184240,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":184424,"parent_record_offset":184396,"ancestor_record_offset":184240,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":185264,"parent_record_offset":185240,"ancestor_record_offset":184240,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":184892,"parent_record_offset":184864,"ancestor_record_offset":184240,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":184924,"parent_record_offset":184892,"ancestor_record_offset":184240,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":184864,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":184724,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":185336,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":184808,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":184696,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":185216,"parent_record_offset":185192,"ancestor_record_offset":184240,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":184480,"parent_record_offset":184452,"ancestor_record_offset":184240,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":184508,"parent_record_offset":184480,"ancestor_record_offset":184240,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":185812,"parent_record_offset":185784,"ancestor_record_offset":184240,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":185784,"parent_record_offset":185756,"ancestor_record_offset":184240,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":185192,"parent_record_offset":185024,"ancestor_record_offset":184240,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":184540,"parent_record_offset":184508,"ancestor_record_offset":184240,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185132,"parent_record_offset":185048,"ancestor_record_offset":184240,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185076,"parent_record_offset":185048,"ancestor_record_offset":184240,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185428,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185048,"parent_record_offset":185024,"ancestor_record_offset":184240,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":184592,"parent_record_offset":184568,"ancestor_record_offset":184240,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":184568,"parent_record_offset":184508,"ancestor_record_offset":184240,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185872,"parent_record_offset":185812,"ancestor_record_offset":184240,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":184616,"parent_record_offset":184592,"ancestor_record_offset":184240,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":184668,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":185024,"parent_record_offset":184616,"ancestor_record_offset":184240,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":184452,"parent_record_offset":184396,"ancestor_record_offset":184240,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":185756,"parent_record_offset":185696,"ancestor_record_offset":184240,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":188392,"parent_record_offset":188340,"ancestor_record_offset":184240,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":185484,"parent_record_offset":184396,"ancestor_record_offset":184240,"inlinee":19917,"source_line":336,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":186828,"parent_record_offset":186796,"ancestor_record_offset":184240,"inlinee":20048,"source_line":321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":186884,"parent_record_offset":186796,"ancestor_record_offset":184240,"inlinee":20047,"source_line":316,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":185624,"parent_record_offset":185600,"ancestor_record_offset":184240,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":188340,"parent_record_offset":188316,"ancestor_record_offset":184240,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":185696,"parent_record_offset":185660,"ancestor_record_offset":184240,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":186852,"parent_record_offset":186828,"ancestor_record_offset":184240,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":185508,"parent_record_offset":185484,"ancestor_record_offset":184240,"inlinee":14896,"source_line":603,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":190644,"parent_record_offset":190620,"ancestor_record_offset":184240,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":190668,"parent_record_offset":190644,"ancestor_record_offset":184240,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":185600,"parent_record_offset":185572,"ancestor_record_offset":184240,"inlinee":19814,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":186796,"parent_record_offset":185660,"ancestor_record_offset":184240,"inlinee":62096,"source_line":1130,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":188164,"parent_record_offset":187036,"ancestor_record_offset":184240,"inlinee":62095,"source_line":1150,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":191924,"parent_record_offset":190792,"ancestor_record_offset":184240,"inlinee":62097,"source_line":1169,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":8156,"rva":31587968,"va":5400297088}
// 编译实例 RVAs: 0x1e1fe80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17458578","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e86e9af5fe","size":5,"soft_fail":false,"target_va":5382847888,"va":5400306461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-17551898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8e62df4fe","size":5,"soft_fail":false,"target_va":5382748704,"va":5400300597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=4
//     target_aliases=14 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30234239","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e881a932fe","size":5,"soft_fail":false,"target_va":5370066128,"va":5400300362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=7
//     target_aliases=7 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25621846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8aa0a79fe","size":5,"soft_fail":false,"target_va":5374676336,"va":5400298177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25573895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f9c579fe","size":5,"soft_fail":false,"target_va":5374728464,"va":5400302354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=6
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25568250","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e806dc79fe","size":5,"soft_fail":false,"target_va":5374729632,"va":5400297877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25572317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e823cc79fe","size":5,"soft_fail":false,"target_va":5374730016,"va":5400302328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25223598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8521e7ffe","size":5,"soft_fail":false,"target_va":5375081488,"va":5400305081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=28
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25218559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e801327ffe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400300058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24728871","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8d9aa86fe","size":5,"soft_fail":false,"target_va":5375573952,"va":5400302818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833230
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22992503","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e88929a1fe","size":5,"soft_fail":false,"target_va":5377307184,"va":5400299682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140877160
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22727180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f435a5fe","size":5,"soft_fail":false,"target_va":5377585504,"va":5400312679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21892007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e859f4b1fe","size":5,"soft_fail":false,"target_va":5378407120,"va":5400299122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a9fa70
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20450104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8c8f4c7fe","size":5,"soft_fail":false,"target_va":5379848816,"va":5400298915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b468f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19771894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e80a4ed2fe","size":5,"soft_fail":false,"target_va":5380532464,"va":5400304353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=8
//     target_aliases=5 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17472264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f864f5fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5400299987,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f992e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15234634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8b68917ff","size":5,"soft_fail":false,"target_va":5385065184,"va":5400299813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9c4f0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15220702","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e822c017ff","size":5,"soft_fail":false,"target_va":5385078000,"va":5400298697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d590
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15216218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8a6d117ff","size":5,"soft_fail":false,"target_va":5385082256,"va":5400298469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9dba0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15219029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8abc617ff","size":5,"soft_fail":false,"target_va":5385083808,"va":5400302832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15212270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e812e117ff","size":5,"soft_fail":false,"target_va":5385087344,"va":5400299609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa06a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15203326","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8020418ff","size":5,"soft_fail":false,"target_va":5385094816,"va":5400298137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa6ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15179828","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8cc5f18ff","size":5,"soft_fail":false,"target_va":5385120928,"va":5400300751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140faa000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15177622","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e86a6818ff","size":5,"soft_fail":false,"target_va":5385134080,"va":5400311697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb8070
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15111010","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e89e6c19ff","size":5,"soft_fail":false,"target_va":5385191536,"va":5400302541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb8600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15116187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8655819ff","size":5,"soft_fail":false,"target_va":5385192960,"va":5400309142,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fb9a80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15101676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8149119ff","size":5,"soft_fail":false,"target_va":5385198208,"va":5400299879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbb410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15103438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8328a19ff","size":5,"soft_fail":false,"target_va":5385204752,"va":5400308185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbd420
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15095742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e842a819ff","size":5,"soft_fail":false,"target_va":5385212960,"va":5400308697,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbe1a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15083223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e829d919ff","size":5,"soft_fail":false,"target_va":5385216416,"va":5400299634,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fc23e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15075386","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8c6f719ff","size":5,"soft_fail":false,"target_va":5385233376,"va":5400308757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fcf110
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15022499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e85dc61aff","size":5,"soft_fail":false,"target_va":5385285904,"va":5400308398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fe41e0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14937245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e863131cff","size":5,"soft_fail":false,"target_va":5385372128,"va":5400309368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14289aa40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10988458","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8aaaba700","size":5,"soft_fail":false,"target_va":5411285568,"va":5400297105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11545911","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8372db000","size":5,"soft_fail":false,"target_va":5411847152,"va":5400301236,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11542048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8201eb000","size":5,"soft_fail":false,"target_va":5411847168,"va":5400305115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6412","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f4e6ffff","size":5,"soft_fail":false,"target_va":5400294304,"va":5400300711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17464824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e80882f5fe","size":5,"soft_fail":false,"target_va":5382840208,"va":5400305027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t-25094172","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8e41781fe","size":5,"soft_fail":false,"target_va":5375203792,"va":5400297959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11231536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e83061ab00","size":5,"soft_fail":false,"target_va":5411540963,"va":5400309422,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-17645834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f6bef2fe","size":5,"soft_fail":false,"target_va":5382655024,"va":5400300853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-15012623","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f1ec1aff","size":5,"soft_fail":false,"target_va":5385288576,"va":5400301194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-14945423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e871f31bff","size":5,"soft_fail":false,"target_va":5385355664,"va":5400301082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11232241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f163ab00","size":5,"soft_fail":false,"target_va":5411541584,"va":5400309338,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15129254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e85a2519ff","size":5,"soft_fail":false,"target_va":5385172320,"va":5400301569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t11237409","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e82178ab00","size":5,"soft_fail":false,"target_va":5411541808,"va":5400304394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11242762","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e80a8dab00","size":5,"soft_fail":false,"target_va":5411541776,"va":5400299009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::session::SessionMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15128572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8042819ff","size":5,"soft_fail":false,"target_va":5385183920,"va":5400312487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::session::NextSessionKind> >
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-15116866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8be5519ff","size":5,"soft_fail":false,"target_va":5385184240,"va":5400301101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::Result::unwrap_or_default<country_core::resources::session::SessionMeta,anyhow::Error>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6154","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f6e7ffff","size":5,"soft_fail":false,"target_va":5400294160,"va":5400300309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11233730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8c269ab00","size":5,"soft_fail":false,"target_va":5411543120,"va":5400309385,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::sort::stable::driftsort_main
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t224425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8a96c0300","size":5,"soft_fail":false,"target_va":5400533872,"va":5400309442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4131","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8ddefffff","size":5,"soft_fail":false,"target_va":5400293088,"va":5400297214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11246064","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f099ab00","size":5,"soft_fail":false,"target_va":5411543360,"va":5400297291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22433664","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e880b0a9fe","size":5,"soft_fail":false,"target_va":5377866688,"va":5400300347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19777659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e88537d2fe","size":5,"soft_fail":false,"target_va":5380530560,"va":5400308214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19769801","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e83756d2fe","size":5,"soft_fail":false,"target_va":5380530176,"va":5400299972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::assets_root
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19769277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e84358d2fe","size":5,"soft_fail":false,"target_va":5380531056,"va":5400300328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::write_to_disk
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19775612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8843fd2fe","size":5,"soft_fail":false,"target_va":5380531136,"va":5400306743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_completion_checker::is_session_ok_to_load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20450070","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8eaf4c7fe","size":5,"soft_fail":false,"target_va":5379848832,"va":5400298897,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_game::parse_saved_game_data
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10157789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8230165ff","size":5,"soft_fail":false,"target_va":5390142720,"va":5400300504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-20162284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e81459ccfe","size":5,"soft_fail":false,"target_va":5380145968,"va":5400308247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19415118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8b2bfd7fe","size":5,"soft_fail":false,"target_va":5380887680,"va":5400302793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18912187","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8456cdffe","size":5,"soft_fail":false,"target_va":5381385536,"va":5400297718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18906998","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e88a80dffe","size":5,"soft_fail":false,"target_va":5381390640,"va":5400297633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18911933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8436ddffe","size":5,"soft_fail":false,"target_va":5381386480,"va":5400298408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-18912486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e81a6bdffe","size":5,"soft_fail":false,"target_va":5381384704,"va":5400297185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-17555946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8161ef4fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5400300021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-17555012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8bc21f4fe","size":5,"soft_fail":false,"target_va":5382745088,"va":5400300095,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t11295797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8355cac00","size":5,"soft_fail":false,"target_va":5411594832,"va":5400299030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-15020969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e857cc1aff","size":5,"soft_fail":false,"target_va":5385288736,"va":5400309700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t11227802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e89a52ab00","size":5,"soft_fail":false,"target_va":5411537104,"va":5400309297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-17552969","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8b729f4fe","size":5,"soft_fail":false,"target_va":5382747616,"va":5400300580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24756985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8073d86fe","size":5,"soft_fail":false,"target_va":5375540928,"va":5400297908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24752509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8834e86fe","size":5,"soft_fail":false,"target_va":5375549872,"va":5400302376,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::hover
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24761293","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8332c86fe","size":5,"soft_fail":false,"target_va":5375540448,"va":5400301736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::display_save_file_load_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10861","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e86d2a0000","size":5,"soft_fail":false,"target_va":5400312720,"va":5400301854,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::load_session_meta
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t12537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e8f9300000","size":5,"soft_fail":false,"target_va":5400312832,"va":5400300290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::show_contextual_buttons
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400297088,"raw_hex":"e86a330000","size":5,"soft_fail":false,"target_va":5400313120,"va":5400299953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2762,"record_offset":30236,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::sort_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":36236,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":7662,"source_line":298,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":2762,"record_offset":11288,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":6804,"source_line":2163,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":30944,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":36464,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2762,"record_offset":10880,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":2762,"record_offset":14080,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_state","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.16.0/src/state/resources.rs
//     evidence={"module":2762,"record_offset":17944,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":12393,"source_line":134,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":2762,"record_offset":26268,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":18576,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":36408,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":17036,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":18372,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":2762,"record_offset":17800,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":2762,"record_offset":35840,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":16016,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::filter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":13968,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":8083,"source_line":1483,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":19092,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$5::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":31728,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=35
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":12640,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2762,"record_offset":26168,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$6::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2762,"record_offset":35692,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":32591,"source_line":1768,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$33::default
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2762,"record_offset":17972,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":62090,"source_line":303,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":15448,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":9896,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":8464,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":8920,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":15672,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":15532,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16044,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":10524,"source_line":2057,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2762,"record_offset":15364,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::fullscreen
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":11000,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":10700,"source_line":434,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":15072,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":61863,"source_line":456,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::safe_frame
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":19564,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":10705,"source_line":445,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::wide_safe_frame
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":37380,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":12679,"source_line":451,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":22632,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":19852,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::default_focus
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":20104,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":20065,"source_line":1412,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":21268,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":19837,"source_line":374,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":21356,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":19831,"source_line":333,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":21444,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::abandon_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13856,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99751,"source_line":141,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::clicked_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13996,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99753,"source_line":137,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::hovered_item
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13912,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99752,"source_line":129,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":14904,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99754,"source_line":133,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::initial_focused_item
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13500,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99747,"source_line":145,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::lock
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13768,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99750,"source_line":185,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13592,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99748,"source_line":191,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::nav_scroll_speed
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":13680,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99749,"source_line":197,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":11376,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":62428,"source_line":230,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::ui::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":26568,"parent_record_offset":8156,"ancestor_record_offset":8156,"inlinee":99755,"source_line":728,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":8156,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":8156,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":8156,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27260,"parent_record_offset":27068,"ancestor_record_offset":8156,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":15928,"parent_record_offset":15872,"ancestor_record_offset":8156,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2762,"record_offset":26648,"parent_record_offset":26568,"ancestor_record_offset":8156,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2762,"record_offset":26696,"parent_record_offset":26672,"ancestor_record_offset":8156,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":11160,"parent_record_offset":11128,"ancestor_record_offset":8156,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27008,"parent_record_offset":26980,"ancestor_record_offset":8156,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":15900,"parent_record_offset":15872,"ancestor_record_offset":8156,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":15872,"parent_record_offset":15848,"ancestor_record_offset":8156,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":8156,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":8156,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27068,"parent_record_offset":27036,"ancestor_record_offset":8156,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27036,"parent_record_offset":27008,"ancestor_record_offset":8156,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":15848,"parent_record_offset":15824,"ancestor_record_offset":8156,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27112,"parent_record_offset":27068,"ancestor_record_offset":8156,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":31088,"parent_record_offset":31052,"ancestor_record_offset":8156,"inlinee":11743,"source_line":124,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26944,"parent_record_offset":26916,"ancestor_record_offset":8156,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26916,"parent_record_offset":26888,"ancestor_record_offset":8156,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26860,"parent_record_offset":26816,"ancestor_record_offset":8156,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26888,"parent_record_offset":26860,"ancestor_record_offset":8156,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":8156,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::stable_sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":36268,"parent_record_offset":36236,"ancestor_record_offset":8156,"inlinee":6036,"source_line":907,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2762,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":8156,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2762,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":8156,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":11128,"parent_record_offset":11092,"ancestor_record_offset":8156,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":31024,"parent_record_offset":30980,"ancestor_record_offset":8156,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":11092,"parent_record_offset":11024,"ancestor_record_offset":8156,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":26980,"parent_record_offset":26944,"ancestor_record_offset":8156,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":35864,"parent_record_offset":35840,"ancestor_record_offset":8156,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":36680,"parent_record_offset":36656,"ancestor_record_offset":8156,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":30980,"parent_record_offset":30944,"ancestor_record_offset":8156,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=19
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2762,"record_offset":10908,"parent_record_offset":10880,"ancestor_record_offset":8156,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2762,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":8156,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2762,"record_offset":14812,"parent_record_offset":14788,"ancestor_record_offset":8156,"inlinee":9991,"source_line":124,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":2762,"record_offset":14168,"parent_record_offset":14104,"ancestor_record_offset":8156,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2762,"record_offset":27140,"parent_record_offset":27112,"ancestor_record_offset":8156,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2762,"record_offset":27168,"parent_record_offset":27140,"ancestor_record_offset":8156,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":2762,"record_offset":17108,"parent_record_offset":17084,"ancestor_record_offset":8156,"inlinee":10090,"source_line":9,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\equality.rs
//     evidence={"module":2762,"record_offset":17132,"parent_record_offset":17108,"ancestor_record_offset":8156,"inlinee":9478,"source_line":146,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":2762,"record_offset":26384,"parent_record_offset":26292,"ancestor_record_offset":8156,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10232,"parent_record_offset":10204,"ancestor_record_offset":8156,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10284,"parent_record_offset":10260,"ancestor_record_offset":8156,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10260,"parent_record_offset":10232,"ancestor_record_offset":8156,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10180,"parent_record_offset":10152,"ancestor_record_offset":8156,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10204,"parent_record_offset":10180,"ancestor_record_offset":8156,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10376,"parent_record_offset":10204,"ancestor_record_offset":8156,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":2762,"record_offset":10488,"parent_record_offset":10464,"ancestor_record_offset":8156,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":18600,"parent_record_offset":18576,"ancestor_record_offset":8156,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":18476,"parent_record_offset":18452,"ancestor_record_offset":8156,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":12272,"parent_record_offset":12248,"ancestor_record_offset":8156,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2762,"record_offset":26732,"parent_record_offset":26568,"ancestor_record_offset":8156,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2762,"record_offset":11780,"parent_record_offset":11756,"ancestor_record_offset":8156,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":2762,"record_offset":11864,"parent_record_offset":11840,"ancestor_record_offset":8156,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2762,"record_offset":11804,"parent_record_offset":11780,"ancestor_record_offset":8156,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2762,"record_offset":12340,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2762,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":8156,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":2762,"record_offset":31580,"parent_record_offset":31540,"ancestor_record_offset":8156,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":2762,"record_offset":16880,"parent_record_offset":16740,"ancestor_record_offset":8156,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":2762,"record_offset":31540,"parent_record_offset":31088,"ancestor_record_offset":8156,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":2762,"record_offset":35888,"parent_record_offset":35864,"ancestor_record_offset":8156,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":2762,"record_offset":31680,"parent_record_offset":31088,"ancestor_record_offset":8156,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":2762,"record_offset":10308,"parent_record_offset":10284,"ancestor_record_offset":8156,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":8156,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":36556,"parent_record_offset":36520,"ancestor_record_offset":8156,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":26352,"parent_record_offset":26328,"ancestor_record_offset":8156,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":37220,"parent_record_offset":37192,"ancestor_record_offset":8156,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":2762,"record_offset":8620,"parent_record_offset":8588,"ancestor_record_offset":8156,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2762,"record_offset":12012,"parent_record_offset":11984,"ancestor_record_offset":8156,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":2762,"record_offset":11984,"parent_record_offset":11956,"ancestor_record_offset":8156,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":2762,"record_offset":15472,"parent_record_offset":15448,"ancestor_record_offset":8156,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":2762,"record_offset":10048,"parent_record_offset":10004,"ancestor_record_offset":8156,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":2762,"record_offset":26328,"parent_record_offset":26292,"ancestor_record_offset":8156,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":2762,"record_offset":26292,"parent_record_offset":26268,"ancestor_record_offset":8156,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":16452,"parent_record_offset":16416,"ancestor_record_offset":8156,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":8156,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":12516,"parent_record_offset":11416,"ancestor_record_offset":8156,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":19148,"parent_record_offset":19124,"ancestor_record_offset":8156,"inlinee":5835,"source_line":2186,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":16792,"parent_record_offset":16740,"ancestor_record_offset":8156,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2762,"record_offset":8868,"parent_record_offset":8840,"ancestor_record_offset":8156,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2762,"record_offset":10836,"parent_record_offset":10004,"ancestor_record_offset":8156,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2762,"record_offset":27360,"parent_record_offset":26944,"ancestor_record_offset":8156,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2762,"record_offset":26496,"parent_record_offset":26408,"ancestor_record_offset":8156,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=142
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":10440,"parent_record_offset":10152,"ancestor_record_offset":8156,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":2762,"record_offset":12132,"parent_record_offset":12108,"ancestor_record_offset":8156,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":11452,"parent_record_offset":11416,"ancestor_record_offset":8156,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":21508,"parent_record_offset":21476,"ancestor_record_offset":8156,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":9332,"parent_record_offset":9308,"ancestor_record_offset":8156,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":9308,"parent_record_offset":9284,"ancestor_record_offset":8156,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":2762,"record_offset":9516,"parent_record_offset":9488,"ancestor_record_offset":8156,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":26520,"parent_record_offset":26496,"ancestor_record_offset":8156,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":10332,"parent_record_offset":10308,"ancestor_record_offset":8156,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":2762,"record_offset":35716,"parent_record_offset":35692,"ancestor_record_offset":8156,"inlinee":4624,"source_line":658,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2762,"record_offset":16680,"parent_record_offset":16656,"ancestor_record_offset":8156,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":2762,"record_offset":16704,"parent_record_offset":16680,"ancestor_record_offset":8156,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":2762,"record_offset":26408,"parent_record_offset":26384,"ancestor_record_offset":8156,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2762,"record_offset":26464,"parent_record_offset":26440,"ancestor_record_offset":8156,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2762,"record_offset":36588,"parent_record_offset":36520,"ancestor_record_offset":8156,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2762,"record_offset":36492,"parent_record_offset":36464,"ancestor_record_offset":8156,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2762,"record_offset":26440,"parent_record_offset":26408,"ancestor_record_offset":8156,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":2762,"record_offset":36520,"parent_record_offset":36492,"ancestor_record_offset":8156,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2762,"record_offset":9260,"parent_record_offset":9236,"ancestor_record_offset":8156,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2762,"record_offset":9284,"parent_record_offset":9260,"ancestor_record_offset":8156,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2762,"record_offset":9588,"parent_record_offset":9560,"ancestor_record_offset":8156,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2762,"record_offset":31608,"parent_record_offset":31580,"ancestor_record_offset":8156,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":2762,"record_offset":9236,"parent_record_offset":9120,"ancestor_record_offset":8156,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::sort::stable::sort
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\sort\stable\mod.rs
//     evidence={"module":2762,"record_offset":36300,"parent_record_offset":36268,"ancestor_record_offset":8156,"inlinee":6038,"source_line":29,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2762,"record_offset":8588,"parent_record_offset":8560,"ancestor_record_offset":8156,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":2762,"record_offset":8948,"parent_record_offset":8920,"ancestor_record_offset":8156,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2762,"record_offset":9088,"parent_record_offset":9056,"ancestor_record_offset":8156,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":2762,"record_offset":9056,"parent_record_offset":9020,"ancestor_record_offset":8156,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2762,"record_offset":8996,"parent_record_offset":8972,"ancestor_record_offset":8156,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2762,"record_offset":9020,"parent_record_offset":8996,"ancestor_record_offset":8156,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2762,"record_offset":8972,"parent_record_offset":8948,"ancestor_record_offset":8156,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":2762,"record_offset":9560,"parent_record_offset":9020,"ancestor_record_offset":8156,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8560,"parent_record_offset":8528,"ancestor_record_offset":8156,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8528,"parent_record_offset":8500,"ancestor_record_offset":8156,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8720,"parent_record_offset":8688,"ancestor_record_offset":8156,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8816,"parent_record_offset":8720,"ancestor_record_offset":8156,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8688,"parent_record_offset":8664,"ancestor_record_offset":8156,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8500,"parent_record_offset":8464,"ancestor_record_offset":8156,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":2762,"record_offset":8840,"parent_record_offset":8816,"ancestor_record_offset":8156,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2762,"record_offset":9120,"parent_record_offset":9088,"ancestor_record_offset":8156,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2762,"record_offset":9404,"parent_record_offset":9120,"ancestor_record_offset":8156,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2762,"record_offset":9376,"parent_record_offset":9120,"ancestor_record_offset":8156,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":2762,"record_offset":9180,"parent_record_offset":9120,"ancestor_record_offset":8156,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":11316,"parent_record_offset":11288,"ancestor_record_offset":8156,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":2762,"record_offset":11340,"parent_record_offset":11316,"ancestor_record_offset":8156,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":2762,"record_offset":18452,"parent_record_offset":18428,"ancestor_record_offset":8156,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":2762,"record_offset":31788,"parent_record_offset":31756,"ancestor_record_offset":8156,"inlinee":13524,"source_line":24,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$12::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2762,"record_offset":31756,"parent_record_offset":31728,"ancestor_record_offset":8156,"inlinee":13522,"source_line":27,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::image_formation::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/image_formation.rs
//     evidence={"module":2762,"record_offset":31832,"parent_record_offset":31756,"ancestor_record_offset":8156,"inlinee":13526,"source_line":11,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2762,"record_offset":26616,"parent_record_offset":26568,"ancestor_record_offset":8156,"inlinee":10698,"source_line":36,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::impl$22::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2762,"record_offset":17060,"parent_record_offset":17036,"ancestor_record_offset":8156,"inlinee":76809,"source_line":123,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":2762,"record_offset":12296,"parent_record_offset":12272,"ancestor_record_offset":8156,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2762,"record_offset":11924,"parent_record_offset":11896,"ancestor_record_offset":8156,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2762,"record_offset":11956,"parent_record_offset":11924,"ancestor_record_offset":8156,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":2762,"record_offset":11896,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2762,"record_offset":11756,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2762,"record_offset":12368,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=14
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":2762,"record_offset":11840,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":2762,"record_offset":11728,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":2762,"record_offset":12248,"parent_record_offset":12224,"ancestor_record_offset":8156,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":21612,"parent_record_offset":21572,"ancestor_record_offset":8156,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":21640,"parent_record_offset":21612,"ancestor_record_offset":8156,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":11536,"parent_record_offset":11508,"ancestor_record_offset":8156,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":11508,"parent_record_offset":11480,"ancestor_record_offset":8156,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":12224,"parent_record_offset":12056,"ancestor_record_offset":8156,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":2762,"record_offset":11568,"parent_record_offset":11536,"ancestor_record_offset":8156,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":12164,"parent_record_offset":12080,"ancestor_record_offset":8156,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":12108,"parent_record_offset":12080,"ancestor_record_offset":8156,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":12460,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":12080,"parent_record_offset":12056,"ancestor_record_offset":8156,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":11620,"parent_record_offset":11596,"ancestor_record_offset":8156,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":21700,"parent_record_offset":21640,"ancestor_record_offset":8156,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":11596,"parent_record_offset":11536,"ancestor_record_offset":8156,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":8156,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":11700,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":2762,"record_offset":12056,"parent_record_offset":11644,"ancestor_record_offset":8156,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":9932,"parent_record_offset":9896,"ancestor_record_offset":8156,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":12704,"parent_record_offset":12672,"ancestor_record_offset":8156,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":13220,"parent_record_offset":12960,"ancestor_record_offset":8156,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":2762,"record_offset":10076,"parent_record_offset":10048,"ancestor_record_offset":8156,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":10152,"parent_record_offset":10004,"ancestor_record_offset":8156,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":2762,"record_offset":10412,"parent_record_offset":10152,"ancestor_record_offset":8156,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2762,"record_offset":21572,"parent_record_offset":21536,"ancestor_record_offset":8156,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2762,"record_offset":11480,"parent_record_offset":11416,"ancestor_record_offset":8156,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":2762,"record_offset":21536,"parent_record_offset":21476,"ancestor_record_offset":8156,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":15580,"parent_record_offset":15556,"ancestor_record_offset":8156,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":15628,"parent_record_offset":15604,"ancestor_record_offset":8156,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":15604,"parent_record_offset":15580,"ancestor_record_offset":8156,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16416,"parent_record_offset":16336,"ancestor_record_offset":8156,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":8156,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16516,"parent_record_offset":16488,"ancestor_record_offset":8156,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16388,"parent_record_offset":16336,"ancestor_record_offset":8156,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16740,"parent_record_offset":16336,"ancestor_record_offset":8156,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16336,"parent_record_offset":16044,"ancestor_record_offset":8156,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16080,"parent_record_offset":16044,"ancestor_record_offset":8156,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16820,"parent_record_offset":16792,"ancestor_record_offset":8156,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":16852,"parent_record_offset":16740,"ancestor_record_offset":8156,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":2762,"record_offset":18428,"parent_record_offset":18404,"ancestor_record_offset":8156,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":2762,"record_offset":10104,"parent_record_offset":10076,"ancestor_record_offset":8156,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2762,"record_offset":10004,"parent_record_offset":9968,"ancestor_record_offset":8156,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":2762,"record_offset":9968,"parent_record_offset":9932,"ancestor_record_offset":8156,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":2762,"record_offset":18404,"parent_record_offset":18372,"ancestor_record_offset":8156,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":11024,"parent_record_offset":11000,"ancestor_record_offset":8156,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":8156,"inlinee":20045,"source_line":305,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":8156,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":21476,"parent_record_offset":21444,"ancestor_record_offset":8156,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":11416,"parent_record_offset":11376,"ancestor_record_offset":8156,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2762,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":8156,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2762,"record_offset":22656,"parent_record_offset":22632,"ancestor_record_offset":8156,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2762,"record_offset":22680,"parent_record_offset":22656,"ancestor_record_offset":8156,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":11064,"parent_record_offset":11024,"ancestor_record_offset":8156,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":19900,"parent_record_offset":19876,"ancestor_record_offset":8156,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":19928,"parent_record_offset":19876,"ancestor_record_offset":8156,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":19876,"parent_record_offset":19852,"ancestor_record_offset":8156,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":21296,"parent_record_offset":21268,"ancestor_record_offset":8156,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13880,"parent_record_offset":13856,"ancestor_record_offset":8156,"inlinee":99745,"source_line":205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13532,"parent_record_offset":13500,"ancestor_record_offset":8156,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":2762,"record_offset":12548,"parent_record_offset":11376,"ancestor_record_offset":8156,"inlinee":62426,"source_line":519,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] uuid::impl$22::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"uuid","version":"1.15.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/uuid-1.15.1/src/lib.rs
//     evidence={"module":2762,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":8156,"inlinee":10735,"source_line":437,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$4::thumbnail_scroll<ref$<str$> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1181,"record_offset":205632,"rva":16385696,"va":5385094816}
// 编译实例 RVAs: 0xfa06a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=3
//     target_aliases=7 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10418771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e8ad0561ff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385095102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10366493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e8e3d161ff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385094952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10365255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e8b9d661ff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385094882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10364920","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e808d861ff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385094931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b5d090
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4470508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e814c9bbff","size":5,"soft_fail":false,"target_va":5380624528,"va":5385095031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d590
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e835ceffff","size":5,"soft_fail":false,"target_va":5385082256,"va":5385094998,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26446054","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e8e6889301","size":5,"soft_fail":false,"target_va":5411541776,"va":5385095717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9545104","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385094816,"raw_hex":"e8705a6eff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385094971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":210072,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":205824,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":208936,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":207796,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":20023,"source_line":1436,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::with_node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":206624,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":19779,"source_line":1532,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":205796,"parent_record_offset":205632,"ancestor_record_offset":205632,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":206056,"parent_record_offset":206020,"ancestor_record_offset":205632,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":206020,"parent_record_offset":205992,"ancestor_record_offset":205632,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":207516,"parent_record_offset":207492,"ancestor_record_offset":205632,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":207024,"parent_record_offset":207000,"ancestor_record_offset":205632,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":207108,"parent_record_offset":207084,"ancestor_record_offset":205632,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":207048,"parent_record_offset":207024,"ancestor_record_offset":205632,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":207584,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":207256,"parent_record_offset":207228,"ancestor_record_offset":205632,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":207228,"parent_record_offset":207200,"ancestor_record_offset":205632,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":207760,"parent_record_offset":206660,"ancestor_record_offset":205632,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":205992,"parent_record_offset":205952,"ancestor_record_offset":205632,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":207376,"parent_record_offset":207352,"ancestor_record_offset":205632,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":206696,"parent_record_offset":206660,"ancestor_record_offset":205632,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":206088,"parent_record_offset":206056,"ancestor_record_offset":205632,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":206116,"parent_record_offset":206056,"ancestor_record_offset":205632,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":206148,"parent_record_offset":206020,"ancestor_record_offset":205632,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":207540,"parent_record_offset":207516,"ancestor_record_offset":205632,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":207168,"parent_record_offset":207140,"ancestor_record_offset":205632,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":207200,"parent_record_offset":207168,"ancestor_record_offset":205632,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":207140,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":207000,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":207612,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":207084,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":206972,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":207492,"parent_record_offset":207468,"ancestor_record_offset":205632,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":206780,"parent_record_offset":206752,"ancestor_record_offset":205632,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":206752,"parent_record_offset":206724,"ancestor_record_offset":205632,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":207468,"parent_record_offset":207300,"ancestor_record_offset":205632,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":206812,"parent_record_offset":206780,"ancestor_record_offset":205632,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":207408,"parent_record_offset":207324,"ancestor_record_offset":205632,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":207352,"parent_record_offset":207324,"ancestor_record_offset":205632,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":207704,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":207324,"parent_record_offset":207300,"ancestor_record_offset":205632,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":206864,"parent_record_offset":206840,"ancestor_record_offset":205632,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":206840,"parent_record_offset":206780,"ancestor_record_offset":205632,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":206888,"parent_record_offset":206864,"ancestor_record_offset":205632,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":206944,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":207300,"parent_record_offset":206888,"ancestor_record_offset":205632,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":206724,"parent_record_offset":206660,"ancestor_record_offset":205632,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":205896,"parent_record_offset":205824,"ancestor_record_offset":205632,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":206660,"parent_record_offset":206624,"ancestor_record_offset":205632,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":205868,"parent_record_offset":205824,"ancestor_record_offset":205632,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":205920,"parent_record_offset":205896,"ancestor_record_offset":205632,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::impl$4::thumbnail_scroll::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1181,"record_offset":205952,"parent_record_offset":205824,"ancestor_record_offset":205632,"inlinee":61859,"source_line":47,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$4::thumbnail_scroll::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 47
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF1A3
// 首条位置: {"module":1181,"inlinee":61859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":205992,"parent_record_offset":205952,"ancestor_record_offset":205952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":206056,"parent_record_offset":206020,"ancestor_record_offset":205952,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":206020,"parent_record_offset":205992,"ancestor_record_offset":205952,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":206088,"parent_record_offset":206056,"ancestor_record_offset":205952,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":206116,"parent_record_offset":206056,"ancestor_record_offset":205952,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":206148,"parent_record_offset":206020,"ancestor_record_offset":205952,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$8::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF19D
// 首条位置: {"module":1181,"inlinee":61853}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":162952,"parent_record_offset":162924,"ancestor_record_offset":162924,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163100,"parent_record_offset":163076,"ancestor_record_offset":162924,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163124,"parent_record_offset":163100,"ancestor_record_offset":162924,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163076,"parent_record_offset":163052,"ancestor_record_offset":162924,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":162992,"parent_record_offset":162952,"ancestor_record_offset":162924,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163020,"parent_record_offset":162992,"ancestor_record_offset":162924,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163052,"parent_record_offset":163020,"ancestor_record_offset":162924,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163228,"parent_record_offset":162952,"ancestor_record_offset":162924,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1181,"record_offset":163252,"parent_record_offset":163228,"ancestor_record_offset":162924,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1181,"record_offset":163196,"parent_record_offset":163172,"ancestor_record_offset":162924,"inlinee":12273,"source_line":214,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1181,"record_offset":163172,"parent_record_offset":162952,"ancestor_record_offset":162924,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$0::create_node_content
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF19B
// 首条位置: {"module":1181,"inlinee":61851}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":163292,"parent_record_offset":162876,"ancestor_record_offset":162876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::impl$8::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1181,"record_offset":162924,"parent_record_offset":162876,"ancestor_record_offset":162876,"inlinee":61853,"source_line":66,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163100,"parent_record_offset":163076,"ancestor_record_offset":162876,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163124,"parent_record_offset":163100,"ancestor_record_offset":162876,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":163076,"parent_record_offset":163052,"ancestor_record_offset":162876,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":162992,"parent_record_offset":162952,"ancestor_record_offset":162876,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163020,"parent_record_offset":162992,"ancestor_record_offset":162876,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163052,"parent_record_offset":163020,"ancestor_record_offset":162876,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1181,"record_offset":163228,"parent_record_offset":162952,"ancestor_record_offset":162876,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":163556,"parent_record_offset":163520,"ancestor_record_offset":162876,"inlinee":4776,"source_line":1888,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$37::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":163340,"parent_record_offset":163316,"ancestor_record_offset":162876,"inlinee":4774,"source_line":2550,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$56::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1181,"record_offset":162952,"parent_record_offset":162924,"ancestor_record_offset":162876,"inlinee":7957,"source_line":3466,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":1181,"record_offset":163252,"parent_record_offset":163228,"ancestor_record_offset":162876,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":163316,"parent_record_offset":163292,"ancestor_record_offset":162876,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":163588,"parent_record_offset":163556,"ancestor_record_offset":162876,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":163376,"parent_record_offset":163340,"ancestor_record_offset":162876,"inlinee":5814,"source_line":2986,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":163400,"parent_record_offset":163376,"ancestor_record_offset":162876,"inlinee":4778,"source_line":3777,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1181,"record_offset":163432,"parent_record_offset":163340,"ancestor_record_offset":162876,"inlinee":4779,"source_line":4075,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gui::impl$28::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gui.rs
//     evidence={"module":1181,"record_offset":163196,"parent_record_offset":163172,"ancestor_record_offset":162876,"inlinee":12273,"source_line":214,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$7::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":1181,"record_offset":163172,"parent_record_offset":162952,"ancestor_record_offset":162876,"inlinee":10633,"source_line":422,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::hovered_item
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A8
// 首条位置: {"module":2762,"inlinee":99752}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13936,"parent_record_offset":13912,"ancestor_record_offset":13912,"inlinee":99745,"source_line":205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 133
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AA
// 首条位置: {"module":2762,"inlinee":99754}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":14928,"parent_record_offset":14904,"ancestor_record_offset":14904,"inlinee":99745,"source_line":205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::clicked_item
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 137
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A9
// 首条位置: {"module":2762,"inlinee":99753}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":14020,"parent_record_offset":13996,"ancestor_record_offset":13996,"inlinee":99745,"source_line":205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::abandon_focus
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 141
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A7
// 首条位置: {"module":2762,"inlinee":99751}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13880,"parent_record_offset":13856,"ancestor_record_offset":13856,"inlinee":99745,"source_line":205,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::initial_focused_item
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A3
// 首条位置: {"module":2762,"inlinee":99747}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13532,"parent_record_offset":13500,"ancestor_record_offset":13500,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":13500,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::tile_height
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AC
// 首条位置: {"module":2845,"inlinee":99756}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2845,"record_offset":6084,"parent_record_offset":6056,"ancestor_record_offset":6056,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2845,"record_offset":6108,"parent_record_offset":6084,"ancestor_record_offset":6056,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::gap
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 159
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AD
// 首条位置: {"module":2845,"inlinee":99757}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2845,"record_offset":6172,"parent_record_offset":6144,"ancestor_record_offset":6144,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2845,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":6144,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::run_count
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AE
// 首条位置: {"module":2845,"inlinee":99758}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2845,"record_offset":6436,"parent_record_offset":6408,"ancestor_record_offset":6408,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2845,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":6408,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::horizontal
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AF
// 首条位置: {"module":2845,"inlinee":99759}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2845,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":6320,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2845,"record_offset":6372,"parent_record_offset":6348,"ancestor_record_offset":6320,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02845.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::lock
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 185
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A6
// 首条位置: {"module":2762,"inlinee":99750}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13796,"parent_record_offset":13768,"ancestor_record_offset":13768,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":13820,"parent_record_offset":13796,"ancestor_record_offset":13768,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A4
// 首条位置: {"module":2762,"inlinee":99748}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13620,"parent_record_offset":13592,"ancestor_record_offset":13592,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":13644,"parent_record_offset":13620,"ancestor_record_offset":13592,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ThumbnailScrollView::nav_scroll_speed
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185A5
// 首条位置: {"module":2762,"inlinee":99749}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::menu::load::ThumbnailScrollView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":2762,"record_offset":13708,"parent_record_offset":13680,"ancestor_record_offset":13680,"inlinee":99746,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":13680,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$3::on_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":5124,"rva":31585408,"va":5400294528}
// 编译实例 RVAs: 0x1e1f480
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$3::prepare_layout
// 记录形式: compiled_procedure; instances=1
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":5240,"rva":31585456,"va":5400294576}
// 编译实例 RVAs: 0x1e1f4b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::impl$20::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":5368,"parent_record_offset":5240,"ancestor_record_offset":5240,"inlinee":19710,"source_line":1583,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$3::process_input_action
// 记录形式: compiled_procedure; instances=1
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":7740,"rva":31587568,"va":5400296688}
// 编译实例 RVAs: 0x1e1fcf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::input::action_set::ActionSetAction::as_analog
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21853465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400296688,"raw_hex":"e8e78ab2fe","size":5,"soft_fail":false,"target_va":5378443264,"va":5400296724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::input::action_set::ActionSetAction::just_pressed
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21853621","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400296688,"raw_hex":"e84b8ab2fe","size":5,"soft_fail":false,"target_va":5378443184,"va":5400296800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":2762,"record_offset":8040,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":8008,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":7888,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":5059,"source_line":1048,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":7916,"parent_record_offset":7740,"ancestor_record_offset":7740,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":8088,"parent_record_offset":8064,"ancestor_record_offset":7740,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":2762,"record_offset":8112,"parent_record_offset":8088,"ancestor_record_offset":7740,"inlinee":20620,"source_line":1826,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":2762,"record_offset":8064,"parent_record_offset":8040,"ancestor_record_offset":7740,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":2762,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":7740,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":2762,"record_offset":7972,"parent_record_offset":7940,"ancestor_record_offset":7740,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::impl$9::default
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 472
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF3D9
// 首条位置: {"module":1243,"inlinee":62425}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 519
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF3DA
// 首条位置: {"module":1243,"inlinee":62426}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":12548,"inlinee":20045,"source_line":305,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":2762,"record_offset":12600,"parent_record_offset":12576,"ancestor_record_offset":12548,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF276
// 首条位置: {"module":1205,"inlinee":62070}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":98172,"parent_record_offset":98144,"ancestor_record_offset":98144,"inlinee":20043,"source_line":295,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95272,"parent_record_offset":95044,"ancestor_record_offset":95044,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95360,"parent_record_offset":95044,"ancestor_record_offset":95044,"inlinee":19823,"source_line":271,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95068,"parent_record_offset":95044,"ancestor_record_offset":95044,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$29::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95448,"parent_record_offset":95044,"ancestor_record_offset":95044,"inlinee":61826,"source_line":198,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":96324,"parent_record_offset":96300,"ancestor_record_offset":95044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":95832,"parent_record_offset":95808,"ancestor_record_offset":95044,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":95916,"parent_record_offset":95892,"ancestor_record_offset":95044,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":95856,"parent_record_offset":95832,"ancestor_record_offset":95044,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":96392,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":96064,"parent_record_offset":96036,"ancestor_record_offset":95044,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":96036,"parent_record_offset":96008,"ancestor_record_offset":95044,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":98260,"parent_record_offset":98236,"ancestor_record_offset":98144,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":96184,"parent_record_offset":96160,"ancestor_record_offset":95044,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":95504,"parent_record_offset":95472,"ancestor_record_offset":95044,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":96348,"parent_record_offset":96324,"ancestor_record_offset":95044,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":95976,"parent_record_offset":95948,"ancestor_record_offset":95044,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":96008,"parent_record_offset":95976,"ancestor_record_offset":95044,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":95948,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":95808,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":96420,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":95892,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":95780,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":96300,"parent_record_offset":96276,"ancestor_record_offset":95044,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":95588,"parent_record_offset":95560,"ancestor_record_offset":95044,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":95560,"parent_record_offset":95532,"ancestor_record_offset":95044,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":96276,"parent_record_offset":96108,"ancestor_record_offset":95044,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":95620,"parent_record_offset":95588,"ancestor_record_offset":95044,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":96216,"parent_record_offset":96132,"ancestor_record_offset":95044,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":96160,"parent_record_offset":96132,"ancestor_record_offset":95044,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":96512,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":96132,"parent_record_offset":96108,"ancestor_record_offset":95044,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":95672,"parent_record_offset":95648,"ancestor_record_offset":95044,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":95648,"parent_record_offset":95588,"ancestor_record_offset":95044,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":95696,"parent_record_offset":95672,"ancestor_record_offset":95044,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":95752,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":96108,"parent_record_offset":95696,"ancestor_record_offset":95044,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":95532,"parent_record_offset":95472,"ancestor_record_offset":95044,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95180,"parent_record_offset":95120,"ancestor_record_offset":95044,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95120,"parent_record_offset":95068,"ancestor_record_offset":95044,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95324,"parent_record_offset":95300,"ancestor_record_offset":95044,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95472,"parent_record_offset":95448,"ancestor_record_offset":95044,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":98208,"parent_record_offset":98172,"ancestor_record_offset":98144,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95092,"parent_record_offset":95068,"ancestor_record_offset":95044,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95152,"parent_record_offset":95120,"ancestor_record_offset":95044,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":95204,"parent_record_offset":95180,"ancestor_record_offset":95044,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95300,"parent_record_offset":95272,"ancestor_record_offset":95044,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":95236,"parent_record_offset":95120,"ancestor_record_offset":95044,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 638
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF268
// 首条位置: {"module":1205,"inlinee":62056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::ignore_input
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":49940,"parent_record_offset":49912,"ancestor_record_offset":49912,"inlinee":20043,"source_line":295,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":47040,"parent_record_offset":46812,"ancestor_record_offset":46812,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":47128,"parent_record_offset":46812,"ancestor_record_offset":46812,"inlinee":19823,"source_line":271,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":46836,"parent_record_offset":46812,"ancestor_record_offset":46812,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$29::deref_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":47216,"parent_record_offset":46812,"ancestor_record_offset":46812,"inlinee":61826,"source_line":198,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":48092,"parent_record_offset":48068,"ancestor_record_offset":46812,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":47600,"parent_record_offset":47576,"ancestor_record_offset":46812,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":47684,"parent_record_offset":47660,"ancestor_record_offset":46812,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":47624,"parent_record_offset":47600,"ancestor_record_offset":46812,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":48160,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":47832,"parent_record_offset":47804,"ancestor_record_offset":46812,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":47804,"parent_record_offset":47776,"ancestor_record_offset":46812,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":50028,"parent_record_offset":50004,"ancestor_record_offset":49912,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":47952,"parent_record_offset":47928,"ancestor_record_offset":46812,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":47272,"parent_record_offset":47240,"ancestor_record_offset":46812,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":48116,"parent_record_offset":48092,"ancestor_record_offset":46812,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":47744,"parent_record_offset":47716,"ancestor_record_offset":46812,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":47776,"parent_record_offset":47744,"ancestor_record_offset":46812,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":47716,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":47576,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":48188,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":47660,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":47548,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":48068,"parent_record_offset":48044,"ancestor_record_offset":46812,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":47356,"parent_record_offset":47328,"ancestor_record_offset":46812,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":47328,"parent_record_offset":47300,"ancestor_record_offset":46812,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":48044,"parent_record_offset":47876,"ancestor_record_offset":46812,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":47388,"parent_record_offset":47356,"ancestor_record_offset":46812,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47984,"parent_record_offset":47900,"ancestor_record_offset":46812,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47928,"parent_record_offset":47900,"ancestor_record_offset":46812,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":48280,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47900,"parent_record_offset":47876,"ancestor_record_offset":46812,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47440,"parent_record_offset":47416,"ancestor_record_offset":46812,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47416,"parent_record_offset":47356,"ancestor_record_offset":46812,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47464,"parent_record_offset":47440,"ancestor_record_offset":46812,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47520,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":47876,"parent_record_offset":47464,"ancestor_record_offset":46812,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":47300,"parent_record_offset":47240,"ancestor_record_offset":46812,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":46948,"parent_record_offset":46888,"ancestor_record_offset":46812,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":46888,"parent_record_offset":46836,"ancestor_record_offset":46812,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":47092,"parent_record_offset":47068,"ancestor_record_offset":46812,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":47240,"parent_record_offset":47216,"ancestor_record_offset":46812,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":49976,"parent_record_offset":49940,"ancestor_record_offset":49912,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":46860,"parent_record_offset":46836,"ancestor_record_offset":46812,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":46920,"parent_record_offset":46888,"ancestor_record_offset":46812,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":46972,"parent_record_offset":46948,"ancestor_record_offset":46812,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":47068,"parent_record_offset":47040,"ancestor_record_offset":46812,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":47004,"parent_record_offset":46888,"ancestor_record_offset":46812,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 728
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x185AB
// 首条位置: {"module":2762,"inlinee":99755}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2762,"record_offset":26648,"parent_record_offset":26568,"ancestor_record_offset":26568,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":2762,"record_offset":26732,"parent_record_offset":26568,"ancestor_record_offset":26568,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$15::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":2762,"record_offset":26616,"parent_record_offset":26568,"ancestor_record_offset":26568,"inlinee":10698,"source_line":36,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27284,"parent_record_offset":27260,"ancestor_record_offset":26568,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27308,"parent_record_offset":27284,"ancestor_record_offset":26568,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":27260,"parent_record_offset":27068,"ancestor_record_offset":26568,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":2762,"record_offset":26696,"parent_record_offset":26672,"ancestor_record_offset":26568,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27008,"parent_record_offset":26980,"ancestor_record_offset":26568,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27068,"parent_record_offset":27036,"ancestor_record_offset":26568,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27036,"parent_record_offset":27008,"ancestor_record_offset":26568,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":27112,"parent_record_offset":27068,"ancestor_record_offset":26568,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26944,"parent_record_offset":26916,"ancestor_record_offset":26568,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26916,"parent_record_offset":26888,"ancestor_record_offset":26568,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26860,"parent_record_offset":26816,"ancestor_record_offset":26568,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26888,"parent_record_offset":26860,"ancestor_record_offset":26568,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":2762,"record_offset":26816,"parent_record_offset":26788,"ancestor_record_offset":26568,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":2762,"record_offset":26788,"parent_record_offset":26760,"ancestor_record_offset":26568,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":2762,"record_offset":26760,"parent_record_offset":26732,"ancestor_record_offset":26568,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":26980,"parent_record_offset":26944,"ancestor_record_offset":26568,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2762,"record_offset":27140,"parent_record_offset":27112,"ancestor_record_offset":26568,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":2762,"record_offset":27168,"parent_record_offset":27140,"ancestor_record_offset":26568,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2762,"record_offset":27384,"parent_record_offset":27360,"ancestor_record_offset":26568,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":2762,"record_offset":27216,"parent_record_offset":27192,"ancestor_record_offset":26568,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":2762,"record_offset":27192,"parent_record_offset":27168,"ancestor_record_offset":26568,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":2762,"record_offset":26672,"parent_record_offset":26648,"ancestor_record_offset":26568,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":2762,"record_offset":27360,"parent_record_offset":26944,"ancestor_record_offset":26568,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$5
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 815
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF26C
// 首条位置: {"module":1205,"inlinee":62060}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":65912,"parent_record_offset":65888,"ancestor_record_offset":65888,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":66016,"parent_record_offset":65964,"ancestor_record_offset":65888,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":65964,"parent_record_offset":65912,"ancestor_record_offset":65888,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":66476,"parent_record_offset":66452,"ancestor_record_offset":66308,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":65936,"parent_record_offset":65912,"ancestor_record_offset":65888,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":65988,"parent_record_offset":65964,"ancestor_record_offset":65888,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":66040,"parent_record_offset":66016,"ancestor_record_offset":65888,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":66452,"parent_record_offset":66332,"ancestor_record_offset":66308,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":66420,"parent_record_offset":66360,"ancestor_record_offset":66308,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$6
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 826
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF36B
// 首条位置: {"module":1225,"inlinee":62315}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ordering::reverse
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1225,"record_offset":14540,"parent_record_offset":14520,"ancestor_record_offset":14520,"inlinee":10270,"source_line":547,"archive":"_pdb/B/modules/01225.jsonl.gz"}
//   [pdb_inline_direct] std::time::impl$32::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":3963,"record_offset":752,"parent_record_offset":732,"ancestor_record_offset":732,"inlinee":75102,"source_line":242,"archive":"_pdb/B/modules/03963.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::SystemTime::intervals
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":3963,"record_offset":800,"parent_record_offset":776,"ancestor_record_offset":732,"inlinee":28246,"source_line":84,"archive":"_pdb/B/modules/03963.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$5::cmp
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":3963,"record_offset":776,"parent_record_offset":752,"ancestor_record_offset":732,"inlinee":16241,"source_line":124,"archive":"_pdb/B/modules/03963.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$7
// 记录形式: compiled_procedure; instances=1
// 源码行: 838
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":153480,"rva":16529472,"va":5385238592}
// 编译实例 RVAs: 0xfc3840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-2596359","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f961d8ff","size":5,"soft_fail":false,"target_va":5382644528,"va":5385240882,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$std..time..Instant$u20$as$u20$core..ops..arith..Add$LT$core..time..Duration$GT$$GT$::add
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2495512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8e8ebd9ff","size":5,"soft_fail":false,"target_va":5382748704,"va":5385244211,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14014b4d0
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15182710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e88a5418ff","size":5,"soft_fail":false,"target_va":5370066128,"va":5385248833,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=6
//     target_aliases=7 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10562376","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8b8d45eff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385238707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10510810","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8269e5fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385239269,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=3
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10509571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8fda25fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385239198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10509236","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e84ca45fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385239247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e7a00
//     owner=unresolved_namespace; count=1
//     target_aliases=92 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10348511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8211862ff","size":5,"soft_fail":false,"target_va":5374900736,"va":5385249242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10159447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8a9fa64ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5385240930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=40
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10158797","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e833fd64ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385240296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=5
//     target_aliases=69 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9724672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8009d6bff","size":5,"soft_fail":false,"target_va":5375516032,"va":5385240699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9666003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e82d826cff","size":5,"soft_fail":false,"target_va":5375573952,"va":5385239950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14093fad0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6836973","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e813ad97ff","size":5,"soft_fail":false,"target_va":5378407120,"va":5385244088,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ae8320
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5095416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e80840b2ff","size":5,"soft_fail":false,"target_va":5380145952,"va":5385241363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b468f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4707894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8ca29b8ff","size":5,"soft_fail":false,"target_va":5380532464,"va":5385240353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d58e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2538437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e83b44d9ff","size":5,"soft_fail":false,"target_va":5382704752,"va":5385243184,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=14
//     target_aliases=5 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2412512","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e82030dbff","size":5,"soft_fail":false,"target_va":5382827728,"va":5385240235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-162980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e85c83fdff","size":5,"soft_fail":false,"target_va":5385083184,"va":5385246159,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9dba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-156157","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8039efdff","size":5,"soft_fail":false,"target_va":5385083808,"va":5385239960,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-158625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e85f94fdff","size":5,"soft_fail":false,"target_va":5385087344,"va":5385245964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fa9980
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-110627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8dd4ffeff","size":5,"soft_fail":false,"target_va":5385132416,"va":5385243038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140faa000
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-115999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8e13afeff","size":5,"soft_fail":false,"target_va":5385134080,"va":5385250074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-103634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e82e6bfeff","size":5,"soft_fail":false,"target_va":5385142608,"va":5385246237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-102698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8d66efeff","size":5,"soft_fail":false,"target_va":5385143488,"va":5385246181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fba940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44034","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8fe53ffff","size":5,"soft_fail":false,"target_va":5385201984,"va":5385246013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbc5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e88a70ffff","size":5,"soft_fail":false,"target_va":5385209264,"va":5385245985,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f6ac0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t7545840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f0237300","size":5,"soft_fail":false,"target_va":5392788160,"va":5385242315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e58d40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15291115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8eb52e900","size":5,"soft_fail":false,"target_va":5400530240,"va":5385239120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e58f20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15289211","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e87b4be900","size":5,"soft_fail":false,"target_va":5400530720,"va":5385241504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141e59860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t15290347","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8eb4fe900","size":5,"soft_fail":false,"target_va":5400533088,"va":5385242736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26601382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8a6e79501","size":5,"soft_fail":false,"target_va":5411847152,"va":5385245765,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t26604082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e832f29501","size":5,"soft_fail":false,"target_va":5411847168,"va":5385243081,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2400915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e86d5ddbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5385241118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-10041375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8e1c766ff","size":5,"soft_fail":false,"target_va":5375203792,"va":5385245162,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26294338","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e842389101","size":5,"soft_fail":false,"target_va":5411540963,"va":5385246620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=7
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-2585346","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8fe8cd8ff","size":5,"soft_fail":false,"target_va":5382655024,"va":5385240365,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::insert_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t111644","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e81cb40100","size":5,"soft_fail":false,"target_va":5385355664,"va":5385244015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::iter::traits::iterator::Iterator::eq_by
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-72680","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e818e4feff","size":5,"soft_fail":false,"target_va":5385172320,"va":5385244995,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26295417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8793c9101","size":5,"soft_fail":false,"target_va":5411541808,"va":5385246386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t26295443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8933c9101","size":5,"soft_fail":false,"target_va":5411541776,"va":5385246328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<country_core::resources::session::SessionMeta,anyhow::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-63990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e80a06ffff","size":5,"soft_fail":false,"target_va":5385183920,"va":5385247905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::result::Result<std::fs::File,std::io::error::Error> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-65214","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e84201ffff","size":5,"soft_fail":false,"target_va":5385184496,"va":5385249705,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<country_core::resources::session::NextSessionKind> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-62934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e82a0affff","size":5,"soft_fail":false,"target_va":5385184240,"va":5385247169,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26296319","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8ff3f9101","size":5,"soft_fail":false,"target_va":5411542880,"va":5385246556,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::glade_settings::GladeSettings::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7373834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f67b8fff","size":5,"soft_fail":false,"target_va":5377866688,"va":5385240517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::backup_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4710437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8db1fb8ff","size":5,"soft_fail":false,"target_va":5380530560,"va":5385240992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4710053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e85b21b8ff","size":5,"soft_fail":false,"target_va":5380530176,"va":5385240224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::assets_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4709451","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8b523b8ff","size":5,"soft_fail":false,"target_va":5380531056,"va":5385240502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::load_game::parse_saved_game_data
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t4902103","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8d7cc4a00","size":5,"soft_fail":false,"target_va":5390142720,"va":5385240612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t7007344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e870ec6a00","size":5,"soft_fail":false,"target_va":5392249152,"va":5385241803,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::OpenOptions::_open
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2532807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8395ad9ff","size":5,"soft_fail":false,"target_va":5382708672,"va":5385241474,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2496191","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e841e9d9ff","size":5,"soft_fail":false,"target_va":5382744080,"va":5385240266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::components
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2499475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e86ddcd9ff","size":5,"soft_fail":false,"target_va":5382745088,"va":5385244558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::file_name
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-2500367","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f1d8d9ff","size":5,"soft_fail":false,"target_va":5382741504,"va":5385241866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::parent
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2502099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e82dd2d9ff","size":5,"soft_fail":false,"target_va":5382741200,"va":5385243294,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::pal::windows::fs::rename
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2470697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8d74cdaff","size":5,"soft_fail":false,"target_va":5382772848,"va":5385243540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-2496583","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8b9e7d9ff","size":5,"soft_fail":false,"target_va":5382747616,"va":5385244194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9704199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f9ec6bff","size":5,"soft_fail":false,"target_va":5375540928,"va":5385245122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9689421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8b3266cff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385239288,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::menu::load::load_session_meta
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t15072497","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8f1fce500","size":5,"soft_fail":false,"target_va":5400312832,"va":5385240330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::file::date_time_string_for_filename
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3332914","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e8ce24cdff","size":5,"soft_fail":false,"target_va":5381908112,"va":5385241021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::local_time::system_time_to_local_time
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3349645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385238592,"raw_hex":"e873e3ccff","size":5,"soft_fail":false,"target_va":5381889424,"va":5385239064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5385238592,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5385242196,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":161400,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1205,"record_offset":169816,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] bevy_state::state::resources::NextState::set
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_state","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_state-0.16.0/src/state/resources.rs
//     evidence={"module":1205,"record_offset":169876,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":12393,"source_line":134,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":176012,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::gt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":172652,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":11236,"source_line":1407,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":160280,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1205,"record_offset":167444,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":171608,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":166916,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=30
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":159104,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::is_ok
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":168612,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":155116,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":159416,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":14043,"source_line":1140,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1205,"record_offset":162520,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1205,"record_offset":162604,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1205,"record_offset":155028,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1205,"record_offset":154916,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":1205,"record_offset":167016,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":8789,"source_line":1366,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::impl$33::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":1205,"record_offset":169788,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":62090,"source_line":303,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":160312,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":160196,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::ffi::os_str::OsStr::to_str
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":166952,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5178,"source_line":853,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::fs::rename
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":167968,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":12457,"source_line":2462,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":159072,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":160488,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":8124,"source_line":1738,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":158932,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$60::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":171636,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":10524,"source_line":2057,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] time::offset_date_time::OffsetDateTime::format
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"time","version":"0.3.36","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.36/src/offset_date_time.rs
//     evidence={"module":1205,"record_offset":155088,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":12241,"source_line":1271,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_relative
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":172856,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":12678,"source_line":473,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":158848,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155192,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::hover
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":156464,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":20053,"source_line":374,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155336,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::mei
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":172800,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":19768,"source_line":370,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":173144,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::set_default_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":153760,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":20066,"source_line":1419,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":176456,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":157584,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":19837,"source_line":374,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":176252,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":157672,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::menu::load::display_save_file_load_error
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":176188,"parent_record_offset":153480,"ancestor_record_offset":153480,"inlinee":62092,"source_line":1080,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":161096,"parent_record_offset":161072,"ancestor_record_offset":153480,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":161120,"parent_record_offset":161096,"ancestor_record_offset":153480,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":159352,"parent_record_offset":159328,"ancestor_record_offset":153480,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":161072,"parent_record_offset":160896,"ancestor_record_offset":153480,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=41
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":159328,"parent_record_offset":159272,"ancestor_record_offset":153480,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1205,"record_offset":162964,"parent_record_offset":162932,"ancestor_record_offset":153480,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":161448,"parent_record_offset":161424,"ancestor_record_offset":153480,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":173016,"parent_record_offset":172984,"ancestor_record_offset":153480,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":160836,"parent_record_offset":160808,"ancestor_record_offset":153480,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":159300,"parent_record_offset":159272,"ancestor_record_offset":153480,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":159272,"parent_record_offset":159248,"ancestor_record_offset":153480,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":173064,"parent_record_offset":173040,"ancestor_record_offset":153480,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":173040,"parent_record_offset":173016,"ancestor_record_offset":153480,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":160896,"parent_record_offset":160864,"ancestor_record_offset":153480,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":160864,"parent_record_offset":160836,"ancestor_record_offset":153480,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":159248,"parent_record_offset":159224,"ancestor_record_offset":153480,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":160932,"parent_record_offset":160896,"ancestor_record_offset":153480,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1205,"record_offset":160768,"parent_record_offset":160732,"ancestor_record_offset":153480,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1205,"record_offset":160732,"parent_record_offset":160660,"ancestor_record_offset":153480,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":172984,"parent_record_offset":172948,"ancestor_record_offset":153480,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":160704,"parent_record_offset":160660,"ancestor_record_offset":153480,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":172948,"parent_record_offset":172880,"ancestor_record_offset":153480,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":160808,"parent_record_offset":160768,"ancestor_record_offset":153480,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$11::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":160624,"parent_record_offset":160592,"ancestor_record_offset":153480,"inlinee":4623,"source_line":3286,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1205,"record_offset":160660,"parent_record_offset":160624,"ancestor_record_offset":153480,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1205,"record_offset":169844,"parent_record_offset":169816,"ancestor_record_offset":153480,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1205,"record_offset":163136,"parent_record_offset":163104,"ancestor_record_offset":153480,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1205,"record_offset":160956,"parent_record_offset":160932,"ancestor_record_offset":153480,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1205,"record_offset":160980,"parent_record_offset":160956,"ancestor_record_offset":153480,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":163776,"parent_record_offset":163628,"ancestor_record_offset":153480,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":176036,"parent_record_offset":176012,"ancestor_record_offset":153480,"inlinee":5087,"source_line":1371,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$66::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":172756,"parent_record_offset":172732,"ancestor_record_offset":153480,"inlinee":5097,"source_line":1820,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":154612,"parent_record_offset":154588,"ancestor_record_offset":153480,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1205,"record_offset":164732,"parent_record_offset":164648,"ancestor_record_offset":153480,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":154120,"parent_record_offset":154096,"ancestor_record_offset":153480,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":154204,"parent_record_offset":154180,"ancestor_record_offset":153480,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":154144,"parent_record_offset":154120,"ancestor_record_offset":153480,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":154680,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":161028,"parent_record_offset":161004,"ancestor_record_offset":153480,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::rev::Rev::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\rev.rs
//     evidence={"module":1205,"record_offset":172148,"parent_record_offset":171984,"ancestor_record_offset":153480,"inlinee":10401,"source_line":20,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1205,"record_offset":163160,"parent_record_offset":163136,"ancestor_record_offset":153480,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1205,"record_offset":161004,"parent_record_offset":160980,"ancestor_record_offset":153480,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":154352,"parent_record_offset":154324,"ancestor_record_offset":153480,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":154324,"parent_record_offset":154296,"ancestor_record_offset":153480,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1205,"record_offset":172392,"parent_record_offset":172356,"ancestor_record_offset":153480,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":172356,"parent_record_offset":172324,"ancestor_record_offset":153480,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":161424,"parent_record_offset":161400,"ancestor_record_offset":153480,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":154884,"parent_record_offset":153760,"ancestor_record_offset":153480,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::impl$5::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":172052,"parent_record_offset":171984,"ancestor_record_offset":153480,"inlinee":4547,"source_line":2037,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":160368,"parent_record_offset":160336,"ancestor_record_offset":153480,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":160396,"parent_record_offset":160336,"ancestor_record_offset":153480,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1205,"record_offset":161172,"parent_record_offset":160768,"ancestor_record_offset":153480,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=152
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":155160,"parent_record_offset":155116,"ancestor_record_offset":153480,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":154988,"parent_record_offset":154964,"ancestor_record_offset":153480,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":154472,"parent_record_offset":154448,"ancestor_record_offset":153480,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163524,"parent_record_offset":163496,"ancestor_record_offset":153480,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163572,"parent_record_offset":163548,"ancestor_record_offset":153480,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163548,"parent_record_offset":163524,"ancestor_record_offset":153480,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":153796,"parent_record_offset":153760,"ancestor_record_offset":153480,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":156496,"parent_record_offset":156464,"ancestor_record_offset":153480,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":155056,"parent_record_offset":155028,"ancestor_record_offset":153480,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":173104,"parent_record_offset":172948,"ancestor_record_offset":153480,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":162708,"parent_record_offset":162656,"ancestor_record_offset":153480,"inlinee":39985,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1205,"record_offset":167112,"parent_record_offset":167088,"ancestor_record_offset":153480,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":1205,"record_offset":167136,"parent_record_offset":167112,"ancestor_record_offset":153480,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1205,"record_offset":154940,"parent_record_offset":154916,"ancestor_record_offset":153480,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1205,"record_offset":167064,"parent_record_offset":167040,"ancestor_record_offset":153480,"inlinee":5361,"source_line":2581,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1205,"record_offset":154964,"parent_record_offset":154940,"ancestor_record_offset":153480,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1205,"record_offset":164964,"parent_record_offset":162604,"ancestor_record_offset":153480,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":1205,"record_offset":167040,"parent_record_offset":167016,"ancestor_record_offset":153480,"inlinee":8748,"source_line":981,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":160220,"parent_record_offset":160196,"ancestor_record_offset":153480,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":160244,"parent_record_offset":160220,"ancestor_record_offset":153480,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::time::impl$21::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":1205,"record_offset":172732,"parent_record_offset":172708,"ancestor_record_offset":153480,"inlinee":5095,"source_line":79,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1205,"record_offset":164152,"parent_record_offset":163628,"ancestor_record_offset":153480,"inlinee":31271,"source_line":92,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=6
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":154636,"parent_record_offset":154612,"ancestor_record_offset":153480,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":154264,"parent_record_offset":154236,"ancestor_record_offset":153480,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":154296,"parent_record_offset":154264,"ancestor_record_offset":153480,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":154236,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":154096,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":154708,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=12
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":154180,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":154068,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":154588,"parent_record_offset":154564,"ancestor_record_offset":153480,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":156548,"parent_record_offset":156524,"ancestor_record_offset":153480,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":156572,"parent_record_offset":156548,"ancestor_record_offset":153480,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":153880,"parent_record_offset":153852,"ancestor_record_offset":153480,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":153852,"parent_record_offset":153824,"ancestor_record_offset":153480,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":154564,"parent_record_offset":154396,"ancestor_record_offset":153480,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":153912,"parent_record_offset":153880,"ancestor_record_offset":153480,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154504,"parent_record_offset":154420,"ancestor_record_offset":153480,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154448,"parent_record_offset":154420,"ancestor_record_offset":153480,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154800,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154420,"parent_record_offset":154396,"ancestor_record_offset":153480,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":153964,"parent_record_offset":153940,"ancestor_record_offset":153480,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":156632,"parent_record_offset":156572,"ancestor_record_offset":153480,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":153940,"parent_record_offset":153880,"ancestor_record_offset":153480,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":153988,"parent_record_offset":153964,"ancestor_record_offset":153480,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154040,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":154396,"parent_record_offset":153988,"ancestor_record_offset":153480,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":160424,"parent_record_offset":160336,"ancestor_record_offset":153480,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":163808,"parent_record_offset":163628,"ancestor_record_offset":153480,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":160336,"parent_record_offset":160312,"ancestor_record_offset":153480,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":160448,"parent_record_offset":160424,"ancestor_record_offset":153480,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163692,"parent_record_offset":163628,"ancestor_record_offset":153480,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":156524,"parent_record_offset":156464,"ancestor_record_offset":153480,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":153824,"parent_record_offset":153760,"ancestor_record_offset":153480,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":157764,"parent_record_offset":157704,"ancestor_record_offset":153480,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":160524,"parent_record_offset":160488,"ancestor_record_offset":153480,"inlinee":8126,"source_line":642,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":158980,"parent_record_offset":158956,"ancestor_record_offset":153480,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":159028,"parent_record_offset":159004,"ancestor_record_offset":153480,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":159004,"parent_record_offset":158980,"ancestor_record_offset":153480,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164648,"parent_record_offset":162656,"ancestor_record_offset":153480,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164672,"parent_record_offset":164648,"ancestor_record_offset":153480,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164928,"parent_record_offset":164648,"ancestor_record_offset":153480,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1205,"record_offset":163496,"parent_record_offset":162864,"ancestor_record_offset":153480,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1205,"record_offset":162864,"parent_record_offset":162832,"ancestor_record_offset":153480,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Components::prefix_verbatim
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":172324,"parent_record_offset":171924,"ancestor_record_offset":153480,"inlinee":10391,"source_line":646,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":158956,"parent_record_offset":158932,"ancestor_record_offset":153480,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Prefix::is_verbatim
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":172420,"parent_record_offset":172392,"ancestor_record_offset":153480,"inlinee":10393,"source_line":222,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$100::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":172296,"parent_record_offset":171924,"ancestor_record_offset":153480,"inlinee":10389,"source_line":371,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$116::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":171984,"parent_record_offset":171924,"ancestor_record_offset":153480,"inlinee":10395,"source_line":589,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$23::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":171924,"parent_record_offset":171636,"ancestor_record_offset":153480,"inlinee":10387,"source_line":987,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":163984,"parent_record_offset":163628,"ancestor_record_offset":153480,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":164756,"parent_record_offset":164732,"ancestor_record_offset":153480,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$89::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":172088,"parent_record_offset":172052,"ancestor_record_offset":153480,"inlinee":10397,"source_line":135,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$98::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":172120,"parent_record_offset":171984,"ancestor_record_offset":153480,"inlinee":10399,"source_line":371,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1205,"record_offset":160560,"parent_record_offset":160524,"ancestor_record_offset":153480,"inlinee":8128,"source_line":13,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1205,"record_offset":164700,"parent_record_offset":164672,"ancestor_record_offset":153480,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::time::impl$20::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\time.rs
//     evidence={"module":1205,"record_offset":172708,"parent_record_offset":172684,"ancestor_record_offset":153480,"inlinee":5093,"source_line":14,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys_common::wtf8::impl$35::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys_common\wtf8.rs
//     evidence={"module":1205,"record_offset":160592,"parent_record_offset":160560,"ancestor_record_offset":153480,"inlinee":8130,"source_line":132,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::time::impl$23::partial_cmp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\time.rs
//     evidence={"module":1205,"record_offset":172684,"parent_record_offset":172652,"ancestor_record_offset":153480,"inlinee":5091,"source_line":149,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":1205,"record_offset":176220,"parent_record_offset":176188,"ancestor_record_offset":153480,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":172880,"parent_record_offset":172856,"ancestor_record_offset":153480,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::hover
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":175924,"parent_record_offset":174832,"ancestor_record_offset":153480,"inlinee":19917,"source_line":336,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":176364,"parent_record_offset":176304,"ancestor_record_offset":153480,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":176304,"parent_record_offset":176252,"ancestor_record_offset":153480,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":157636,"parent_record_offset":157612,"ancestor_record_offset":153480,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":157704,"parent_record_offset":157672,"ancestor_record_offset":153480,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":154856,"parent_record_offset":153760,"ancestor_record_offset":153480,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":175948,"parent_record_offset":175924,"ancestor_record_offset":153480,"inlinee":14896,"source_line":603,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":158872,"parent_record_offset":158848,"ancestor_record_offset":153480,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":158896,"parent_record_offset":158872,"ancestor_record_offset":153480,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155268,"parent_record_offset":155240,"ancestor_record_offset":153480,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155240,"parent_record_offset":155216,"ancestor_record_offset":153480,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155300,"parent_record_offset":155216,"ancestor_record_offset":153480,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":155216,"parent_record_offset":155192,"ancestor_record_offset":153480,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":176480,"parent_record_offset":176456,"ancestor_record_offset":153480,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":157612,"parent_record_offset":157584,"ancestor_record_offset":153480,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":176420,"parent_record_offset":176304,"ancestor_record_offset":153480,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$7::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":162656,"parent_record_offset":162604,"ancestor_record_offset":153480,"inlinee":62086,"source_line":882,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$7::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":163628,"parent_record_offset":162708,"ancestor_record_offset":153480,"inlinee":62088,"source_line":889,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$7::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 882
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF286
// 首条位置: {"module":1205,"inlinee":62086}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::is_ok_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1205,"record_offset":162708,"parent_record_offset":162656,"ancestor_record_offset":162656,"inlinee":39985,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::fs::File::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164648,"parent_record_offset":162656,"ancestor_record_offset":162656,"inlinee":11604,"source_line":382,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":163012,"parent_record_offset":162988,"ancestor_record_offset":162656,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":162988,"parent_record_offset":162964,"ancestor_record_offset":162656,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1205,"record_offset":162964,"parent_record_offset":162932,"ancestor_record_offset":162656,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":1205,"record_offset":163136,"parent_record_offset":163104,"ancestor_record_offset":162656,"inlinee":5033,"source_line":219,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":163776,"parent_record_offset":163628,"ancestor_record_offset":162656,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1205,"record_offset":164732,"parent_record_offset":164648,"ancestor_record_offset":162656,"inlinee":5126,"source_line":703,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1205,"record_offset":163160,"parent_record_offset":163136,"ancestor_record_offset":162656,"inlinee":5034,"source_line":393,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":163864,"parent_record_offset":163832,"ancestor_record_offset":162656,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":163892,"parent_record_offset":163832,"ancestor_record_offset":162656,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":162768,"parent_record_offset":162708,"ancestor_record_offset":162656,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_byte_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163524,"parent_record_offset":163496,"ancestor_record_offset":162656,"inlinee":5966,"source_line":1340,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163572,"parent_record_offset":163548,"ancestor_record_offset":162656,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":163548,"parent_record_offset":163524,"ancestor_record_offset":162656,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":163716,"parent_record_offset":163692,"ancestor_record_offset":162656,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":163740,"parent_record_offset":163716,"ancestor_record_offset":162656,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::history::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1205,"record_offset":164152,"parent_record_offset":163628,"ancestor_record_offset":162656,"inlinee":31271,"source_line":92,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163920,"parent_record_offset":163832,"ancestor_record_offset":162656,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":163808,"parent_record_offset":163628,"ancestor_record_offset":162656,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":163832,"parent_record_offset":163808,"ancestor_record_offset":162656,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163944,"parent_record_offset":163920,"ancestor_record_offset":162656,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163692,"parent_record_offset":163628,"ancestor_record_offset":162656,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164032,"parent_record_offset":164008,"ancestor_record_offset":162656,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164080,"parent_record_offset":164056,"ancestor_record_offset":162656,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164056,"parent_record_offset":164032,"ancestor_record_offset":162656,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164672,"parent_record_offset":164648,"ancestor_record_offset":162656,"inlinee":11594,"source_line":1342,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::OpenOptions::open
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\fs.rs
//     evidence={"module":1205,"record_offset":164928,"parent_record_offset":164648,"ancestor_record_offset":162656,"inlinee":11599,"source_line":1559,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::decode_repr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1205,"record_offset":163496,"parent_record_offset":162864,"ancestor_record_offset":162656,"inlinee":5965,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::io::error::repr_bitpacked::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\io\error\repr_bitpacked.rs
//     evidence={"module":1205,"record_offset":162864,"parent_record_offset":162832,"ancestor_record_offset":162656,"inlinee":5963,"source_line":239,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":164008,"parent_record_offset":163984,"ancestor_record_offset":162656,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$45::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":163984,"parent_record_offset":163628,"ancestor_record_offset":162656,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$83::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":164756,"parent_record_offset":164732,"ancestor_record_offset":162656,"inlinee":5128,"source_line":3362,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::pal::windows::fs::OpenOptions::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\pal\windows\fs.rs
//     evidence={"module":1205,"record_offset":164700,"parent_record_offset":164672,"ancestor_record_offset":162656,"inlinee":11605,"source_line":206,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::menu::load::ui::closure$7::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/menu/load.rs
//     evidence={"module":1205,"record_offset":163628,"parent_record_offset":162708,"ancestor_record_offset":162656,"inlinee":62088,"source_line":889,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$7::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 889
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF288
// 首条位置: {"module":1205,"inlinee":62088}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":163776,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":164124,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::history::impl$23::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/history/mod.rs
//     evidence={"module":1205,"record_offset":164152,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":31271,"source_line":92,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":163808,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163692,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":163984,"parent_record_offset":163628,"ancestor_record_offset":163628,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":163864,"parent_record_offset":163832,"ancestor_record_offset":163628,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":1205,"record_offset":163892,"parent_record_offset":163832,"ancestor_record_offset":163628,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":163716,"parent_record_offset":163692,"ancestor_record_offset":163628,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1205,"record_offset":163740,"parent_record_offset":163716,"ancestor_record_offset":163628,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163920,"parent_record_offset":163832,"ancestor_record_offset":163628,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1205,"record_offset":163832,"parent_record_offset":163808,"ancestor_record_offset":163628,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1205,"record_offset":163944,"parent_record_offset":163920,"ancestor_record_offset":163628,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164032,"parent_record_offset":164008,"ancestor_record_offset":163628,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164080,"parent_record_offset":164056,"ancestor_record_offset":163628,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":1205,"record_offset":164056,"parent_record_offset":164032,"ancestor_record_offset":163628,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":1205,"record_offset":164008,"parent_record_offset":163984,"ancestor_record_offset":163628,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$8
// 记录形式: compiled_procedure; instances=1
// 源码行: 1025
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":180124,"rva":16541344,"va":5385250464}
// 编译实例 RVAs: 0xfc66a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0d70
//     owner=unresolved_namespace; count=2
//     target_aliases=7 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10574309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e81ba65eff","size":5,"soft_fail":false,"target_va":5374676336,"va":5385250640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10522129","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e8ef715fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385250588,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10520896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e8c0765fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385250523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10520557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e813785fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385250568,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14068bfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9677006","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e832576cff","size":5,"soft_fail":false,"target_va":5375573952,"va":5385250953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9b5c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-176807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e8594dfdff","size":5,"soft_fail":false,"target_va":5385074112,"va":5385250914,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9dba0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-167160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e80873fdff","size":5,"soft_fail":false,"target_va":5385083808,"va":5385250963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26290513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e851299101","size":5,"soft_fail":false,"target_va":5411541808,"va":5385251290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26290469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e825299101","size":5,"soft_fail":false,"target_va":5411541776,"va":5385251302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] lang_lib::lookup
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"lang-lib","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4363256","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e8086cbdff","size":5,"soft_fail":false,"target_va":5380887680,"va":5385250931,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9700737","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385250464,"raw_hex":"e87ffa6bff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385250604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":182892,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180264,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::NodeContext::layout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180404,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":19811,"source_line":1400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":181540,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":19837,"source_line":374,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::hotkey
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":181628,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":19831,"source_line":333,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":181716,"parent_record_offset":180124,"ancestor_record_offset":180124,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":181264,"parent_record_offset":181240,"ancestor_record_offset":180124,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":180772,"parent_record_offset":180748,"ancestor_record_offset":180124,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":180856,"parent_record_offset":180832,"ancestor_record_offset":180124,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":180796,"parent_record_offset":180772,"ancestor_record_offset":180124,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":181332,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":181004,"parent_record_offset":180976,"ancestor_record_offset":180124,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":180976,"parent_record_offset":180948,"ancestor_record_offset":180124,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":181508,"parent_record_offset":180404,"ancestor_record_offset":180124,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":181124,"parent_record_offset":181100,"ancestor_record_offset":180124,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":180444,"parent_record_offset":180404,"ancestor_record_offset":180124,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":181772,"parent_record_offset":181744,"ancestor_record_offset":180124,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":181288,"parent_record_offset":181264,"ancestor_record_offset":180124,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":180916,"parent_record_offset":180888,"ancestor_record_offset":180124,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":180948,"parent_record_offset":180916,"ancestor_record_offset":180124,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":180888,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":180748,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":181360,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":180832,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":180720,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":181240,"parent_record_offset":181216,"ancestor_record_offset":180124,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":181872,"parent_record_offset":181832,"ancestor_record_offset":180124,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":181900,"parent_record_offset":181872,"ancestor_record_offset":180124,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":180528,"parent_record_offset":180500,"ancestor_record_offset":180124,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":180500,"parent_record_offset":180472,"ancestor_record_offset":180124,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":181216,"parent_record_offset":181048,"ancestor_record_offset":180124,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":180560,"parent_record_offset":180528,"ancestor_record_offset":180124,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181156,"parent_record_offset":181072,"ancestor_record_offset":180124,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181100,"parent_record_offset":181072,"ancestor_record_offset":180124,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181452,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181072,"parent_record_offset":181048,"ancestor_record_offset":180124,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":180612,"parent_record_offset":180588,"ancestor_record_offset":180124,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181960,"parent_record_offset":181900,"ancestor_record_offset":180124,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":180588,"parent_record_offset":180528,"ancestor_record_offset":180124,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":180636,"parent_record_offset":180612,"ancestor_record_offset":180124,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":180692,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":181048,"parent_record_offset":180636,"ancestor_record_offset":180124,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":181832,"parent_record_offset":181800,"ancestor_record_offset":180124,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":180472,"parent_record_offset":180404,"ancestor_record_offset":180124,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":181800,"parent_record_offset":181744,"ancestor_record_offset":180124,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":181592,"parent_record_offset":181568,"ancestor_record_offset":180124,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":181744,"parent_record_offset":181716,"ancestor_record_offset":180124,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":182916,"parent_record_offset":182892,"ancestor_record_offset":180124,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":182940,"parent_record_offset":182916,"ancestor_record_offset":180124,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180336,"parent_record_offset":180312,"ancestor_record_offset":180124,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180312,"parent_record_offset":180288,"ancestor_record_offset":180124,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180368,"parent_record_offset":180288,"ancestor_record_offset":180124,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$9::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":180288,"parent_record_offset":180264,"ancestor_record_offset":180124,"inlinee":7072,"source_line":154,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":181568,"parent_record_offset":181540,"ancestor_record_offset":180124,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$8::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1033
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF188
// 首条位置: {"module":1181,"inlinee":61832}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1181,"record_offset":133728,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":134276,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::used
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133644,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":20032,"source_line":331,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":134016,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":134104,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::weight
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":134192,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":19823,"source_line":271,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::MenuButtonView::color
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":132404,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":19837,"source_line":374,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":133812,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$31::deref
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":132492,"parent_record_offset":132340,"ancestor_record_offset":132340,"inlinee":19767,"source_line":192,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":134452,"parent_record_offset":134428,"ancestor_record_offset":132340,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1181,"record_offset":134428,"parent_record_offset":134372,"ancestor_record_offset":132340,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1181,"record_offset":133776,"parent_record_offset":133752,"ancestor_record_offset":132340,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":134400,"parent_record_offset":134372,"ancestor_record_offset":132340,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":134372,"parent_record_offset":134348,"ancestor_record_offset":132340,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1181,"record_offset":134348,"parent_record_offset":134324,"ancestor_record_offset":132340,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1181,"record_offset":133388,"parent_record_offset":133364,"ancestor_record_offset":132340,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":132896,"parent_record_offset":132872,"ancestor_record_offset":132340,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1181,"record_offset":132980,"parent_record_offset":132956,"ancestor_record_offset":132340,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":132920,"parent_record_offset":132896,"ancestor_record_offset":132340,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1181,"record_offset":133456,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":133128,"parent_record_offset":133100,"ancestor_record_offset":132340,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1181,"record_offset":133100,"parent_record_offset":133072,"ancestor_record_offset":132340,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":135812,"parent_record_offset":135788,"ancestor_record_offset":132340,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1181,"record_offset":133752,"parent_record_offset":133728,"ancestor_record_offset":132340,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1181,"record_offset":134300,"parent_record_offset":134276,"ancestor_record_offset":132340,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1181,"record_offset":133248,"parent_record_offset":133224,"ancestor_record_offset":132340,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1181,"record_offset":132540,"parent_record_offset":132516,"ancestor_record_offset":132340,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1181,"record_offset":133412,"parent_record_offset":133388,"ancestor_record_offset":132340,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":133040,"parent_record_offset":133012,"ancestor_record_offset":132340,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":133072,"parent_record_offset":133040,"ancestor_record_offset":132340,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1181,"record_offset":133012,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":132872,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":133484,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1181,"record_offset":132956,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1181,"record_offset":132844,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1181,"record_offset":133364,"parent_record_offset":133340,"ancestor_record_offset":132340,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":132624,"parent_record_offset":132596,"ancestor_record_offset":132340,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":132652,"parent_record_offset":132624,"ancestor_record_offset":132340,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":133340,"parent_record_offset":133172,"ancestor_record_offset":132340,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1181,"record_offset":132684,"parent_record_offset":132652,"ancestor_record_offset":132340,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":133280,"parent_record_offset":133196,"ancestor_record_offset":132340,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":133224,"parent_record_offset":133196,"ancestor_record_offset":132340,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":133576,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":133196,"parent_record_offset":133172,"ancestor_record_offset":132340,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":132736,"parent_record_offset":132712,"ancestor_record_offset":132340,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":132712,"parent_record_offset":132652,"ancestor_record_offset":132340,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":132760,"parent_record_offset":132736,"ancestor_record_offset":132340,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":132816,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1181,"record_offset":133172,"parent_record_offset":132760,"ancestor_record_offset":132340,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":132596,"parent_record_offset":132568,"ancestor_record_offset":132340,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1181,"record_offset":132568,"parent_record_offset":132516,"ancestor_record_offset":132340,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133924,"parent_record_offset":133864,"ancestor_record_offset":132340,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133864,"parent_record_offset":133812,"ancestor_record_offset":132340,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":132456,"parent_record_offset":132432,"ancestor_record_offset":132340,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":132516,"parent_record_offset":132492,"ancestor_record_offset":132340,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":133668,"parent_record_offset":133644,"ancestor_record_offset":132340,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1181,"record_offset":133692,"parent_record_offset":133668,"ancestor_record_offset":132340,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133836,"parent_record_offset":133812,"ancestor_record_offset":132340,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133896,"parent_record_offset":133864,"ancestor_record_offset":132340,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1181,"record_offset":133948,"parent_record_offset":133924,"ancestor_record_offset":132340,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":134044,"parent_record_offset":134016,"ancestor_record_offset":132340,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::MenuButtonView::content_mut
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":132432,"parent_record_offset":132404,"ancestor_record_offset":132340,"inlinee":19773,"source_line":210,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1181,"record_offset":133980,"parent_record_offset":133864,"ancestor_record_offset":132340,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01181.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::ui::closure$9
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 1067
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF263
// 首条位置: {"module":1205,"inlinee":62051}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1205,"record_offset":36444,"parent_record_offset":36132,"ancestor_record_offset":36132,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::hotkey
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":31008,"parent_record_offset":30976,"ancestor_record_offset":30976,"inlinee":19880,"source_line":1032,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::PillButtonView::on_click
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":31096,"parent_record_offset":30976,"ancestor_record_offset":30976,"inlinee":62052,"source_line":1002,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$53::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":32384,"parent_record_offset":30976,"ancestor_record_offset":30976,"inlinee":62054,"source_line":192,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":1205,"record_offset":36472,"parent_record_offset":36444,"ancestor_record_offset":36132,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1205,"record_offset":32012,"parent_record_offset":31988,"ancestor_record_offset":30976,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":31520,"parent_record_offset":31496,"ancestor_record_offset":30976,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1205,"record_offset":31604,"parent_record_offset":31580,"ancestor_record_offset":30976,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":31544,"parent_record_offset":31520,"ancestor_record_offset":30976,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1205,"record_offset":32080,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":31752,"parent_record_offset":31724,"ancestor_record_offset":30976,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1205,"record_offset":31724,"parent_record_offset":31696,"ancestor_record_offset":30976,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":36404,"parent_record_offset":36380,"ancestor_record_offset":36132,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":36264,"parent_record_offset":36240,"ancestor_record_offset":36132,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1205,"record_offset":31872,"parent_record_offset":31848,"ancestor_record_offset":30976,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":31192,"parent_record_offset":31160,"ancestor_record_offset":30976,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1205,"record_offset":32432,"parent_record_offset":32408,"ancestor_record_offset":30976,"inlinee":4777,"source_line":425,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] country_core::systems::ui::impl$2::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/ui/mod.rs
//     evidence={"module":1205,"record_offset":32036,"parent_record_offset":32012,"ancestor_record_offset":30976,"inlinee":9662,"source_line":30,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":31664,"parent_record_offset":31636,"ancestor_record_offset":30976,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":31696,"parent_record_offset":31664,"ancestor_record_offset":30976,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1205,"record_offset":31636,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":31496,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":32108,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1205,"record_offset":31580,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1205,"record_offset":31468,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1205,"record_offset":31988,"parent_record_offset":31964,"ancestor_record_offset":30976,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":32516,"parent_record_offset":32488,"ancestor_record_offset":30976,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":32544,"parent_record_offset":32516,"ancestor_record_offset":30976,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":31276,"parent_record_offset":31248,"ancestor_record_offset":30976,"inlinee":8411,"source_line":1457,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":31248,"parent_record_offset":31220,"ancestor_record_offset":30976,"inlinee":8410,"source_line":1445,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":31964,"parent_record_offset":31796,"ancestor_record_offset":30976,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1205,"record_offset":31308,"parent_record_offset":31276,"ancestor_record_offset":30976,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31904,"parent_record_offset":31820,"ancestor_record_offset":30976,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31848,"parent_record_offset":31820,"ancestor_record_offset":30976,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":32200,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31820,"parent_record_offset":31796,"ancestor_record_offset":30976,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31360,"parent_record_offset":31336,"ancestor_record_offset":30976,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":32604,"parent_record_offset":32544,"ancestor_record_offset":30976,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31336,"parent_record_offset":31276,"ancestor_record_offset":30976,"inlinee":8412,"source_line":1189,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31384,"parent_record_offset":31360,"ancestor_record_offset":30976,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31440,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1205,"record_offset":31796,"parent_record_offset":31384,"ancestor_record_offset":30976,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":32488,"parent_record_offset":32460,"ancestor_record_offset":30976,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":31220,"parent_record_offset":31160,"ancestor_record_offset":30976,"inlinee":8409,"source_line":1153,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1205,"record_offset":32460,"parent_record_offset":32408,"ancestor_record_offset":30976,"inlinee":11287,"source_line":1426,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::Node::set_sounds
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":32260,"parent_record_offset":31128,"ancestor_record_offset":30976,"inlinee":19772,"source_line":277,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":31060,"parent_record_offset":31036,"ancestor_record_offset":30976,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":32408,"parent_record_offset":32384,"ancestor_record_offset":30976,"inlinee":19719,"source_line":1521,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::node_mut
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":31160,"parent_record_offset":31128,"ancestor_record_offset":30976,"inlinee":19720,"source_line":1525,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::sounds
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":31128,"parent_record_offset":31096,"ancestor_record_offset":30976,"inlinee":19770,"source_line":170,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::intersects
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":36300,"parent_record_offset":36172,"ancestor_record_offset":36132,"inlinee":19915,"source_line":597,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeInteraction::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":36324,"parent_record_offset":36300,"ancestor_record_offset":36132,"inlinee":19916,"source_line":585,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::PillButtonView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":31036,"parent_record_offset":31008,"ancestor_record_offset":30976,"inlinee":19814,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 1080
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF28C
// 首条位置: {"module":1205,"inlinee":62092}
// 编译实例 RVAs: 0x1e23b90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15225432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312720,"raw_hex":"e8a8ad17ff","size":5,"soft_fail":false,"target_va":5385087344,"va":5400312771,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fba940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15110839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312720,"raw_hex":"e8496d19ff","size":5,"soft_fail":false,"target_va":5385201984,"va":5400312818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fbc5b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15103533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312720,"raw_hex":"e8d38919ff","size":5,"soft_fail":false,"target_va":5385209264,"va":5400312792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":1205,"record_offset":176220,"parent_record_offset":176188,"ancestor_record_offset":176188,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 1083
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF270
// 首条位置: {"module":1205,"inlinee":62064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":76988,"parent_record_offset":76952,"ancestor_record_offset":76952,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":77272,"parent_record_offset":76952,"ancestor_record_offset":76952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":77072,"parent_record_offset":76952,"ancestor_record_offset":76952,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":77448,"parent_record_offset":77424,"ancestor_record_offset":76952,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":77424,"parent_record_offset":77368,"ancestor_record_offset":76952,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":77036,"parent_record_offset":77012,"ancestor_record_offset":76952,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":77396,"parent_record_offset":77368,"ancestor_record_offset":76952,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":77368,"parent_record_offset":77344,"ancestor_record_offset":76952,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":77344,"parent_record_offset":77320,"ancestor_record_offset":76952,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":77012,"parent_record_offset":76988,"ancestor_record_offset":76952,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":77296,"parent_record_offset":77272,"ancestor_record_offset":76952,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":77180,"parent_record_offset":77124,"ancestor_record_offset":76952,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":77124,"parent_record_offset":77072,"ancestor_record_offset":76952,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":77096,"parent_record_offset":77072,"ancestor_record_offset":76952,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":77152,"parent_record_offset":77124,"ancestor_record_offset":76952,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":77204,"parent_record_offset":77180,"ancestor_record_offset":76952,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":77236,"parent_record_offset":77124,"ancestor_record_offset":76952,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::display_save_file_load_error::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 1087
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1205,"record_offset":182980,"rva":16542192,"va":5385251312}
// 编译实例 RVAs: 0xfc69f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bd910
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10523038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e8626e5fff","size":5,"soft_fail":false,"target_va":5374728464,"va":5385251497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10521808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e830735fff","size":5,"soft_fail":false,"target_va":5374729632,"va":5385251435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdf20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10521465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e887745fff","size":5,"soft_fail":false,"target_va":5374730016,"va":5385251476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10170221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e893d064ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5385251720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9d930
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-168344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e8686efdff","size":5,"soft_fail":false,"target_va":5385083184,"va":5385251523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-109093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e8db55feff","size":5,"soft_fail":false,"target_va":5385142608,"va":5385251696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fac4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-108055","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e8e959feff","size":5,"soft_fail":false,"target_va":5385143488,"va":5385251538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-2411203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e83d35dbff","size":5,"soft_fail":false,"target_va":5382840208,"va":5385251406,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_align
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4414432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e820a4bcff","size":5,"soft_fail":false,"target_va":5380837184,"va":5385251611,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::set_max_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-text","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-4414602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e876a3bcff","size":5,"soft_fail":false,"target_va":5380837056,"va":5385251653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::NodeContext::child_with_global_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9701645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5385251312,"raw_hex":"e8f3f66bff","size":5,"soft_fail":false,"target_va":5375549872,"va":5385251512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":183148,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":183780,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::align
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183520,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":19821,"source_line":261,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::color
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183432,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":19824,"source_line":276,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::max_width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183608,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":19820,"source_line":254,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::LabelView::metrics
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183696,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":10727,"source_line":249,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::widgets::impl$26::label
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183228,"parent_record_offset":182980,"ancestor_record_offset":182980,"inlinee":20089,"source_line":42,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":183956,"parent_record_offset":183932,"ancestor_record_offset":182980,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1205,"record_offset":183932,"parent_record_offset":183876,"ancestor_record_offset":182980,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1205,"record_offset":183196,"parent_record_offset":183172,"ancestor_record_offset":182980,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":183904,"parent_record_offset":183876,"ancestor_record_offset":182980,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":183876,"parent_record_offset":183852,"ancestor_record_offset":182980,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1205,"record_offset":183852,"parent_record_offset":183828,"ancestor_record_offset":182980,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1205,"record_offset":183172,"parent_record_offset":183148,"ancestor_record_offset":182980,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1205,"record_offset":183804,"parent_record_offset":183780,"ancestor_record_offset":182980,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183340,"parent_record_offset":183280,"ancestor_record_offset":182980,"inlinee":7070,"source_line":1318,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::child_with_content
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183280,"parent_record_offset":183228,"ancestor_record_offset":182980,"inlinee":12662,"source_line":1302,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeContext::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183484,"parent_record_offset":183460,"ancestor_record_offset":182980,"inlinee":12681,"source_line":1446,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183252,"parent_record_offset":183228,"ancestor_record_offset":182980,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$4::to_typed_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183312,"parent_record_offset":183280,"ancestor_record_offset":182980,"inlinee":7074,"source_line":115,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$8::to_global_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":183364,"parent_record_offset":183340,"ancestor_record_offset":182980,"inlinee":7076,"source_line":148,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::LabelView::content_mut
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183460,"parent_record_offset":183432,"ancestor_record_offset":182980,"inlinee":19769,"source_line":210,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::widgets::impl$26::label::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/widgets.rs
//     evidence={"module":1205,"record_offset":183396,"parent_record_offset":183280,"ancestor_record_offset":182980,"inlinee":20020,"source_line":44,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::load_session_meta
// 记录形式: compiled_procedure; instances=1
// 源码行: 1096
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":42508,"rva":31603712,"va":5400312832}
// 编译实例 RVAs: 0x1e23c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25231449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312832,"raw_hex":"e8a7ff7efe","size":5,"soft_fail":false,"target_va":5375081504,"va":5400312948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d76ed0
//     owner=unresolved_namespace; count=2
//     target_aliases=5 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17485147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312832,"raw_hex":"e8a532f5fe","size":5,"soft_fail":false,"target_va":5382827728,"va":5400312870,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionId::save_folder_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19782691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312832,"raw_hex":"e8dd23d2fe","size":5,"soft_fail":false,"target_va":5380530176,"va":5400312862,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::session::SessionMeta::try_load_from_path
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-19781601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312832,"raw_hex":"e81f28d2fe","size":5,"soft_fail":false,"target_va":5380531328,"va":5400312924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::path::Path::_join
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-17568826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400312832,"raw_hex":"e8c6ebf3fe","size":5,"soft_fail":false,"target_va":5382744080,"va":5400312901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":42952,"parent_record_offset":42508,"ancestor_record_offset":42508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":42780,"parent_record_offset":42508,"ancestor_record_offset":42508,"inlinee":5199,"source_line":2688,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$45::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":42640,"parent_record_offset":42508,"ancestor_record_offset":42508,"inlinee":5130,"source_line":1905,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":43172,"parent_record_offset":43148,"ancestor_record_offset":42508,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":2762,"record_offset":43148,"parent_record_offset":43096,"ancestor_record_offset":42508,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":43120,"parent_record_offset":43096,"ancestor_record_offset":42508,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":43096,"parent_record_offset":43072,"ancestor_record_offset":42508,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":43072,"parent_record_offset":43052,"ancestor_record_offset":42508,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":42972,"parent_record_offset":42952,"ancestor_record_offset":42508,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$59::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":42688,"parent_record_offset":42664,"ancestor_record_offset":42508,"inlinee":5134,"source_line":1639,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$5::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":42736,"parent_record_offset":42712,"ancestor_record_offset":42508,"inlinee":5138,"source_line":599,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$7::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\ffi\os_str.rs
//     evidence={"module":2762,"record_offset":42712,"parent_record_offset":42688,"ancestor_record_offset":42508,"inlinee":5136,"source_line":617,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\path.rs
//     evidence={"module":2762,"record_offset":42664,"parent_record_offset":42640,"ancestor_record_offset":42508,"inlinee":5132,"source_line":2182,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons
// 记录形式: compiled_procedure; instances=1
// 源码行: 1102
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2762,"record_offset":44148,"rva":31604000,"va":5400313120}
// 编译实例 RVAs: 0x1e23d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405bdda0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25583625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400313120,"raw_hex":"e8f79f79fe","size":5,"soft_fail":false,"target_va":5374729632,"va":5400313252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140f9e970
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15226101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400313120,"raw_hex":"e80bab17ff","size":5,"soft_fail":false,"target_va":5385087344,"va":5400313440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140fc26c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15079426","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400313120,"raw_hex":"e8fee719ff","size":5,"soft_fail":false,"target_va":5385234112,"va":5400313533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25109532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400313120,"raw_hex":"e8e4db80fe","size":5,"soft_fail":false,"target_va":5375203792,"va":5400313319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] toolbar::mei::core::MeiContext::reuse_or_create_node
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-24772353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400313120,"raw_hex":"e8ff0086fe","size":5,"soft_fail":false,"target_va":5375540928,"va":5400313276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2762,"record_offset":44284,"parent_record_offset":44148,"ancestor_record_offset":44148,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::config::impl$18::template
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/config.rs
//     evidence={"module":2762,"record_offset":44628,"parent_record_offset":44148,"ancestor_record_offset":44148,"inlinee":20074,"source_line":167,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::MeiContext::new_at_pos
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":44340,"parent_record_offset":44148,"ancestor_record_offset":44148,"inlinee":61863,"source_line":456,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":44500,"parent_record_offset":44468,"ancestor_record_offset":44148,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":44548,"parent_record_offset":44524,"ancestor_record_offset":44148,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":2762,"record_offset":44524,"parent_record_offset":44500,"ancestor_record_offset":44148,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":44468,"parent_record_offset":44432,"ancestor_record_offset":44148,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":2762,"record_offset":44432,"parent_record_offset":44364,"ancestor_record_offset":44148,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":2762,"record_offset":44308,"parent_record_offset":44284,"ancestor_record_offset":44148,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":2762,"record_offset":44588,"parent_record_offset":44432,"ancestor_record_offset":44148,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::MeiContext::new_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":44364,"parent_record_offset":44340,"ancestor_record_offset":44148,"inlinee":10701,"source_line":494,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::impl$0::as_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":2762,"record_offset":44404,"parent_record_offset":44364,"ancestor_record_offset":44148,"inlinee":20018,"source_line":85,"archive":"_pdb/B/modules/02762.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF290
// 首条位置: {"module":1205,"inlinee":62096}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":186828,"parent_record_offset":186796,"ancestor_record_offset":186796,"inlinee":20048,"source_line":321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":186884,"parent_record_offset":186796,"ancestor_record_offset":186796,"inlinee":20047,"source_line":316,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":186852,"parent_record_offset":186828,"ancestor_record_offset":186796,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 1150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF28F
// 首条位置: {"module":1205,"inlinee":62095}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":188196,"parent_record_offset":188164,"ancestor_record_offset":188164,"inlinee":20048,"source_line":321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_direct] toolbar::mei::core::Node::steal_focus
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":188252,"parent_record_offset":188164,"ancestor_record_offset":188164,"inlinee":20047,"source_line":316,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":188220,"parent_record_offset":188196,"ancestor_record_offset":188164,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 1169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF291
// 首条位置: {"module":1205,"inlinee":62097}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] toolbar::mei::core::Node::lose_focus_on_invalid_nav
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"toolbar","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/mei/core.rs
//     evidence={"module":1205,"record_offset":191948,"parent_record_offset":191924,"ancestor_record_offset":191924,"inlinee":20048,"source_line":321,"archive":"_pdb/B/modules/01205.jsonl.gz"}
//   [pdb_inline_transitive] toolbar::mei::core::NodeFlags::insert
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bitflags","version":"1.3.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bitflags-1.3.2/src/lib.rs
//     evidence={"module":1205,"record_offset":191972,"parent_record_offset":191948,"ancestor_record_offset":191924,"inlinee":19913,"source_line":609,"archive":"_pdb/B/modules/01205.jsonl.gz"}
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
// 完整结构及原始类型记录: load.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::LocalState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 35
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "constructor": "bevy_ecs::system::system_param::Local", "arguments": ["toolbar::menu::load::LocalState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd> >", "constructor": "tuple$", "arguments": ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd> > >", "constructor": "bevy_ecs::system::function_system::FunctionSystemState", "arguments": ["tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::function_system::FunctionSystemState<tuple$<bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>", "constructor": "bevy_ecs::system::function_system::FunctionSystem", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::InfallibleSystemWrapper", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_utils::synccell::SyncCell<toolbar::menu::load::LocalState>", "constructor": "bevy_utils::synccell::SyncCell", "arguments": ["toolbar::menu::load::LocalState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<toolbar::menu::load::LocalState> >", "constructor": "core::ptr::drop_in_place", "arguments": ["bevy_utils::synccell::SyncCell<toolbar::menu::load::LocalState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::system_type<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::system_type", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_hash<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_hash", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::as_dyn_eq<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::as_dyn_eq", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$7::dyn_clone<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$7::dyn_clone", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::SystemTypeSet", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "core::any::impl$0::type_id", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::as_any<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::as_any", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::label::impl$0::dyn_eq<bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::label::impl$0::dyn_eq", "arguments": ["bevy_ecs::schedule::set::SystemTypeSet<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::set::impl$1::fmt<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::schedule::set::impl$1::fmt", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::initialize", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::run_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::get_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::get_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::has_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::has_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_exclusive", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::set_last_run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::set_last_run", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::apply_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::apply_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::queue_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::queue_deferred", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::check_change_tick<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::check_change_tick", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::validate_param_unsafe", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::update_archetype_component_access", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::name<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::name", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::is_send<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::is_send", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::type_id<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::system::system::System::type_id", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>", "constructor": "tuple$", "arguments": ["bevy_ecs::system::function_system::IsFunctionSystem", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "tuple$", "arguments": ["bevy_ecs::schedule::config::Infallible", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>,bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState> >", "constructor": "bevy_ecs::schedule::config::IntoScheduleConfigs::run_if", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)", "alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyError> > > > >,alloc::alloc::Global>", "tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "tuple$<bevy_ecs::system::function_system::IsFunctionSystem,bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)>", "bevy_state::condition::in_state::closure_env$0<country_core::app_loading_state::AppLoadingState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>", "constructor": "bevy_ecs::system::function_system::impl$7::update_archetype_component_access", "arguments": ["void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)", "void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::schedule_system::impl$1::default_system_sets<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >", "constructor": "bevy_ecs::system::schedule_system::impl$1::default_system_sets", "arguments": ["bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::validate_param<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::system::system::System::validate_param", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::system::system::System::run_without_applying_deferred", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::system::System::run<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > >", "constructor": "bevy_ecs::system::system::System::run", "arguments": ["bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::SaveLoadError
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::load::SaveLoadError>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::load::SaveLoadError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::ThumbnailScrollContent
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::content<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContext::content", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContext::get_content_mut_impl", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "core::ptr::drop_in_place", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "core::any::impl$0::type_id", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::interactive<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContent::interactive", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::NodeContent::pre_update<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::NodeContent::pre_update", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::as_any_mut<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$21::as_any_mut", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "toolbar::mei::core::impl$21::type_name<toolbar::menu::load::ThumbnailScrollContent>", "constructor": "toolbar::mei::core::impl$21::type_name", "arguments": ["toolbar::menu::load::ThumbnailScrollContent"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] toolbar::menu::load::ThumbnailScrollView
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 17; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 7; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::resources::history::History>", "bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>", "bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>", "bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::history::History>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::abandon_focus
// 信息 ID: 56062465a01482b82b68b581
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13856; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::clicked_item
// 信息 ID: 1c953f4a28ff440467421e6d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13996; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::gap
// 信息 ID: ca7702b40ef87d203084a3fa
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::horizontal
// 信息 ID: ec2b324e2625bdbf991030ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6320; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::hovered_item
// 信息 ID: c858ba25ae1e77a8119af2ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13912; 不是独立调用边。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=14960; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6496; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::hovered_item_pos
// 信息 ID: 883ee1d25d300350ff99a1e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=14904; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::initial_focused_item
// 信息 ID: ef5d83cba4495189934fb533
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13500; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=5964; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::lock
// 信息 ID: 15f99eacc7ba7a6c55bb89b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::min_scroll_pos
// 信息 ID: 1826d685beeba40a99686b64
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13592; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::nav_scroll_speed
// 信息 ID: 86ec7f9ce08e3539fca8ec58
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=13680; 不是独立调用边。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6232; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::run_count
// 信息 ID: f4492f664e14659e00e595b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6408; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ThumbnailScrollView::tile_height
// 信息 ID: 58d9401ee0278a203a4beae2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::photomode::lut_gallery::ui; module=2845, inline_record=6056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error
// 信息 ID: b351617c7e3005c2331fb486
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e2111e -> 0x1e23b90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$7; module=1205, inline_record=176188; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error::closure$0
// 信息 ID: 5b5543b8da7cc28af78f9a86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::display_save_file_load_error", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::display_save_file_load_error::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1205, inline_record=76952; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1205, inline_record=77532; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$0,ref$<str$> >; module=1205, inline_record=78080; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::display_save_file_load_error::closure$1
// 信息 ID: defaf90e1196f178099ebfd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "toolbar::menu::load::display_save_file_load_error", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::display_save_file_load_error::closure_env$1 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$1,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::display_save_file_load_error::closure_env$1,ref$<str$> > @ 0xfbac04 -> 0xfc69f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$0::create_node_content
// 信息 ID: 948649dc137b01305e819d0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>; module=1181, inline_record=162876; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$3::on_input
// 信息 ID: bfba64a0a0dac33007dcd8a2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$3::prepare_layout
// 信息 ID: 6dc4005436e0fc8195a0e43b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$3::process_input_action
// 信息 ID: da06417cbf57902e36217e15
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$3::render
// 信息 ID: 48d2070d5fdef9e5234990a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$4::thumbnail_scroll::closure$0
// 信息 ID: 3b48968ff953a88474048822
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::impl$4::thumbnail_scroll", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::impl$4::thumbnail_scroll<ref$<str$> >; module=1181, inline_record=205952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$4::thumbnail_scroll<ref$<str$> >
// 信息 ID: 7612e23b064de118d9ffb093
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] ref$ -> ["str$"]; 字段布局 = unknown。
// [类型关系] toolbar::menu::load::impl$4::thumbnail_scroll -> ["ref$<str$>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e20299 -> 0xfa06a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::photomode::lut_gallery::ui @ 0x1ec0602 -> 0xfa06a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$8::default
// 信息 ID: 93202080712cc86e80a81025
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::get_content_mut_impl<toolbar::menu::load::ThumbnailScrollContent>; module=1181, inline_record=162924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::impl$9::default
// 信息 ID: f1925e87bee4198a550287d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eb0ec72c6f2fb8117f84eae0: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::schedule_system::impl$1::initialize<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> >; module=1243, inline_record=2612; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::load_session_meta
// 信息 ID: 704b8db5f5e8176517d38c79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui::closure$7 @ 0xfc3f0a -> 0x1e23c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e21aca -> 0x1e23c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e20b02 -> 0x1e23c00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons
// 信息 ID: a3f9c5a4d1e336a8bb7386d5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::menu::load::ui @ 0x1e209b1 -> 0x1e23d20 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0
// 信息 ID: 3abed5403c73f312d91c60e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::show_contextual_buttons::closure_env$0 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::show_contextual_buttons::closure_env$0,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::show_contextual_buttons::closure_env$0,ref$<str$> > @ 0xfc2984 -> 0xfc6be0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$0
// 信息 ID: 352bc0be3954106cf6410026
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1205, inline_record=186796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$1
// 信息 ID: 8c756e241a712429fd7c2da1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1205, inline_record=188164; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::show_contextual_buttons::closure$0::closure$2
// 信息 ID: 56cac4d333ba0f35c707f4e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::show_contextual_buttons", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::show_contextual_buttons::closure$0; module=1205, inline_record=191924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui
// 信息 ID: 8bf6a7415949a7b364a2f0a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] eb0ec72c6f2fb8117f84eae0: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > @ 0xff4421 -> 0x1e1fe80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::SavedSessionList>,bevy_ecs::system::system_param::Local<toolbar::menu::load::LocalState>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::menu_audio::PlayMenuUiSound>,bevy_ecs::event::writer::EventWriter<country_core::systems::save::save_game::DeleteSessionCmd>)> > > @ 0x1e11dcc -> 0x1e1fe80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$0
// 信息 ID: 2ddb85a9e30868b253fcc3cc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=12548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$2
// 信息 ID: d54de708ab7e228a6819691f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$2 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=1205, inline_record=95044; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=1205, inline_record=96608; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$2,ref$<str$> >; module=1205, inline_record=98144; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$3
// 信息 ID: 6b5dc0b74f069977d793a6f8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$3 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$3,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$3,ref$<str$> >; module=1205, inline_record=46812; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$3,ref$<str$> >; module=1205, inline_record=48376; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$3,ref$<str$> >; module=1205, inline_record=49912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$4
// 信息 ID: 7d0c15dd693867eb3926272a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui; module=2762, inline_record=26568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$5
// 信息 ID: 3d1dce35702d58912856b166
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 5, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$5 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1205, inline_record=65888; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1205, inline_record=66112; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$5,ref$<str$> >; module=1205, inline_record=66308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$6
// 信息 ID: be70483009e84704ec7b29b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 6, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$6 <- core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$6 <- core::slice::sort::stable::driftsort_main<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6>,alloc::vec::Vec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::alloc::Global> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$6 <- core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$6 <- core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$6 <- core::slice::sort::stable::quicksort::quicksort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 14 条；闭包环境类型引用 5 条。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=1225, inline_record=14520; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::smallsort::insertion_sort_shift_left<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=1225, inline_record=14684; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3956, inline_record=1140; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3956, inline_record=1232; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3956, inline_record=1316; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3956, inline_record=2536; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::stable::drift::sort<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3956, inline_record=2760; 不是独立调用边。
// [PDB 内联宿主] core::slice::sort::shared::pivot::median3_rec<country_core::systems::save::save_backup_list::BackupSaveFile,alloc::slice::impl$0::sort_by::closure_env$0<country_core::systems::save::save_backup_list::BackupSaveFile,toolbar::menu::load::ui::closure_env$6> >; module=3963, inline_record=732; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$7
// 信息 ID: a9e75e005c32582d62bafccb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$7 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$7,ref$<str$> > @ 0xfbd6e4 -> 0xfc3840 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$7::closure$0
// 信息 ID: 63d09cf96775f069e45b2421
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$7; module=1205, inline_record=162656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$7::closure$0::closure$0
// 信息 ID: 804297dfca15909e13be5fcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 7, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] toolbar::menu::load::ui::closure$7; module=1205, inline_record=163628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$8
// 信息 ID: 7dbecf7bcacfbf27ea9964d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$8 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$8,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$8,ref$<str$> > @ 0xfc26a4 -> 0xfc66a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$8::closure$0
// 信息 ID: ebd83d0083f51ec2b23a923b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 8, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure$8::closure_env$0 <- toolbar::mei::core::NodeContext::with<toolbar::menu::load::ui::closure$8::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::core::NodeContext::with<toolbar::menu::load::ui::closure$8::closure_env$0>; module=1181, inline_record=132340; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::menu::load::ui::closure$9
// 信息 ID: 4e87a646655695e542d173f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 9, "lexical_owner": "toolbar::menu::load::ui", "captures": "unknown"}
// [闭包环境类型引用] toolbar::menu::load::ui::closure_env$9 <- toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> >; module=1205, inline_record=30976; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> >; module=1205, inline_record=33568; 不是独立调用边。
// [PDB 内联宿主] toolbar::mei::config::TemplateView::with_slot<toolbar::menu::load::ui::closure_env$9,ref$<str$> >; module=1205, inline_record=36132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
