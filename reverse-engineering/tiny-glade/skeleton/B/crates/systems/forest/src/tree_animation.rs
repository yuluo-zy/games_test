// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/forest/src/tree_animation.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: aa7801127872eae7250902542d909ef497bd00ac575cce0145deaaf7697290e1
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: tree_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3707
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use forest; // namespace candidate
//   package identities: [{"name":"forest","source":"local_or_path","version":"0.1.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3707,"record_offset":2032,"rva":36148480,"va":5404857600}
// 编译实例 RVAs: 0x2279500
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: tree_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27551557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8bb985bfe","size":5,"soft_fail":false,"target_va":5377307216,"va":5404858768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6681473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e881f36500","size":5,"soft_fail":false,"target_va":5411541584,"va":5404860106,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1205","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e84bfbffff","size":5,"soft_fail":false,"target_va":5404856544,"va":5404857744,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6685541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e865036600","size":5,"soft_fail":false,"target_va":5411543360,"va":5404857814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::boxes
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13617827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e85d3530ff","size":5,"soft_fail":false,"target_va":5391240768,"va":5404858590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::get_triggers
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-13617735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8b93530ff","size":5,"soft_fail":false,"target_va":5391240848,"va":5404858578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23472775","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e879d599fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5404858306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23467586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8bee999fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5404858221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23473473","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8bfd299fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5404859948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23473016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e888d499fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5404857715,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6948559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8cf066a00","size":5,"soft_fail":false,"target_va":5411808608,"va":5404860044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e86cf0ffff","size":5,"soft_fail":false,"target_va":5404856400,"va":5404860383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6677005","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e80de26500","size":5,"soft_fail":false,"target_va":5411537104,"va":5404860094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_direction
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22948678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8bad4a1fe","size":5,"soft_fail":false,"target_va":5381909888,"va":5404858561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22948730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e886d4a1fe","size":5,"soft_fail":false,"target_va":5381909824,"va":5404858549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::intersect_segment
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-17011885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e8536bfcfe","size":5,"soft_fail":false,"target_va":5387847024,"va":5404858904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::pack_2x16f_uint
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22974448","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404857600,"raw_hex":"e81070a1fe","size":5,"soft_fail":false,"target_va":5381885424,"va":5404859867,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":4664,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":8456,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":6112,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":5704,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3707,"record_offset":4784,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3707,"record_offset":8576,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3707,"record_offset":4900,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":12536,"source_line":2758,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":8744,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] forest::tree_animation::should_remove_tree_canopy_from_view
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"forest","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3707,"record_offset":2680,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":121209,"source_line":152,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":3696,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":2220,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":2720,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3707,"record_offset":8660,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":14226,"source_line":314,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3707,"record_offset":4816,"parent_record_offset":2032,"ancestor_record_offset":2032,"inlinee":13649,"source_line":233,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":5944,"parent_record_offset":5920,"ancestor_record_offset":2032,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":5992,"parent_record_offset":5968,"ancestor_record_offset":2032,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":5968,"parent_record_offset":5944,"ancestor_record_offset":2032,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":5920,"parent_record_offset":5896,"ancestor_record_offset":2032,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":5896,"parent_record_offset":5864,"ancestor_record_offset":2032,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":6480,"parent_record_offset":6456,"ancestor_record_offset":2032,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":6504,"parent_record_offset":6480,"ancestor_record_offset":2032,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":2032,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":7372,"parent_record_offset":7348,"ancestor_record_offset":2032,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":7736,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":7184,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":7932,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":7100,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":4692,"parent_record_offset":4664,"ancestor_record_offset":2032,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":2032,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7644,"parent_record_offset":7620,"ancestor_record_offset":2032,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":6676,"parent_record_offset":6652,"ancestor_record_offset":2032,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":6568,"parent_record_offset":6540,"ancestor_record_offset":2032,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7620,"parent_record_offset":7596,"ancestor_record_offset":2032,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7668,"parent_record_offset":7644,"ancestor_record_offset":2032,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7296,"parent_record_offset":7268,"ancestor_record_offset":2032,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7348,"parent_record_offset":7320,"ancestor_record_offset":2032,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":6652,"parent_record_offset":6540,"ancestor_record_offset":2032,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":2032,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$63::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":7596,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":13690,"source_line":2083,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":5812,"parent_record_offset":5728,"ancestor_record_offset":2032,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":2032,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":2032,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3707,"record_offset":7268,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11680,"source_line":182,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3707,"record_offset":6540,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11676,"source_line":195,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":6776,"parent_record_offset":6752,"ancestor_record_offset":2032,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":6752,"parent_record_offset":6728,"ancestor_record_offset":2032,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":6616,"parent_record_offset":6592,"ancestor_record_offset":2032,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":6592,"parent_record_offset":6568,"ancestor_record_offset":2032,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":6728,"parent_record_offset":6704,"ancestor_record_offset":2032,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":6456,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":7692,"parent_record_offset":7668,"ancestor_record_offset":2032,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":6372,"parent_record_offset":6136,"ancestor_record_offset":2032,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":5780,"parent_record_offset":5756,"ancestor_record_offset":2032,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":2032,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":5756,"parent_record_offset":5728,"ancestor_record_offset":2032,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3707,"record_offset":8100,"parent_record_offset":8068,"ancestor_record_offset":2032,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3707,"record_offset":8412,"parent_record_offset":8012,"ancestor_record_offset":2032,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":4016,"parent_record_offset":3988,"ancestor_record_offset":2032,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":2032,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":4044,"parent_record_offset":4016,"ancestor_record_offset":2032,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":3964,"parent_record_offset":3936,"ancestor_record_offset":2032,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":2032,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":4160,"parent_record_offset":3988,"ancestor_record_offset":2032,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":2032,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":3707,"record_offset":5316,"parent_record_offset":4984,"ancestor_record_offset":2032,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3707,"record_offset":5028,"parent_record_offset":4984,"ancestor_record_offset":2032,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3707,"record_offset":5380,"parent_record_offset":4924,"ancestor_record_offset":2032,"inlinee":12540,"source_line":2765,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3707,"record_offset":4924,"parent_record_offset":4900,"ancestor_record_offset":2032,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3707,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2032,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3707,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":2032,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3707,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":2032,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3707,"record_offset":5476,"parent_record_offset":5448,"ancestor_record_offset":2032,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3707,"record_offset":3832,"parent_record_offset":3792,"ancestor_record_offset":2032,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3707,"record_offset":5656,"parent_record_offset":5404,"ancestor_record_offset":2032,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3707,"record_offset":5348,"parent_record_offset":4956,"ancestor_record_offset":2032,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3707,"record_offset":2628,"parent_record_offset":2600,"ancestor_record_offset":2032,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3707,"record_offset":4620,"parent_record_offset":3792,"ancestor_record_offset":2032,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":4224,"parent_record_offset":3936,"ancestor_record_offset":2032,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3707,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":2032,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":5216,"parent_record_offset":5184,"ancestor_record_offset":2032,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":3132,"parent_record_offset":3108,"ancestor_record_offset":2032,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2032,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":3316,"parent_record_offset":3288,"ancestor_record_offset":2032,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2032,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3707,"record_offset":6396,"parent_record_offset":6372,"ancestor_record_offset":2032,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":6420,"parent_record_offset":6396,"ancestor_record_offset":2032,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3707,"record_offset":6036,"parent_record_offset":5864,"ancestor_record_offset":2032,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2032,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":5184,"parent_record_offset":5152,"ancestor_record_offset":2032,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2032,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":3388,"parent_record_offset":3360,"ancestor_record_offset":2032,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":5068,"parent_record_offset":5028,"ancestor_record_offset":2032,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":3036,"parent_record_offset":2920,"ancestor_record_offset":2032,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":5152,"parent_record_offset":5068,"ancestor_record_offset":2032,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3707,"record_offset":2348,"parent_record_offset":2320,"ancestor_record_offset":2032,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3707,"record_offset":2748,"parent_record_offset":2720,"ancestor_record_offset":2032,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3707,"record_offset":2888,"parent_record_offset":2856,"ancestor_record_offset":2032,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3707,"record_offset":2856,"parent_record_offset":2820,"ancestor_record_offset":2032,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":2796,"parent_record_offset":2772,"ancestor_record_offset":2032,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":2820,"parent_record_offset":2796,"ancestor_record_offset":2032,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":2772,"parent_record_offset":2748,"ancestor_record_offset":2032,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":3360,"parent_record_offset":2820,"ancestor_record_offset":2032,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2320,"parent_record_offset":2284,"ancestor_record_offset":2032,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2284,"parent_record_offset":2256,"ancestor_record_offset":2032,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2480,"parent_record_offset":2448,"ancestor_record_offset":2032,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2576,"parent_record_offset":2480,"ancestor_record_offset":2032,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":2032,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2256,"parent_record_offset":2220,"ancestor_record_offset":2032,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":2600,"parent_record_offset":2576,"ancestor_record_offset":2032,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":2920,"parent_record_offset":2888,"ancestor_record_offset":2032,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":3204,"parent_record_offset":2920,"ancestor_record_offset":2032,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":3176,"parent_record_offset":2920,"ancestor_record_offset":2032,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":2980,"parent_record_offset":2920,"ancestor_record_offset":2032,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] forest::tree_animation::handplaced_tree_animation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"forest","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3707,"record_offset":5404,"parent_record_offset":5380,"ancestor_record_offset":2032,"inlinee":121211,"source_line":74,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5628,"parent_record_offset":5404,"ancestor_record_offset":2032,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5536,"parent_record_offset":5404,"ancestor_record_offset":2032,"inlinee":30675,"source_line":1420,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$60::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5448,"parent_record_offset":5404,"ancestor_record_offset":2032,"inlinee":30677,"source_line":1560,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":3728,"parent_record_offset":3696,"ancestor_record_offset":2032,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":8808,"parent_record_offset":8776,"ancestor_record_offset":2032,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":9308,"parent_record_offset":9048,"ancestor_record_offset":2032,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3707,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":2032,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":3936,"parent_record_offset":3792,"ancestor_record_offset":2032,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":4196,"parent_record_offset":3936,"ancestor_record_offset":2032,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3707,"record_offset":5260,"parent_record_offset":4984,"ancestor_record_offset":2032,"inlinee":30671,"source_line":52,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3707,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":2032,"inlinee":16035,"source_line":48,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$12::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3707,"record_offset":4984,"parent_record_offset":4956,"ancestor_record_offset":2032,"inlinee":30670,"source_line":1008,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3707,"record_offset":4956,"parent_record_offset":4924,"ancestor_record_offset":2032,"inlinee":30668,"source_line":1061,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3707,"record_offset":3888,"parent_record_offset":3860,"ancestor_record_offset":2032,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3707,"record_offset":3792,"parent_record_offset":3760,"ancestor_record_offset":2032,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3707,"record_offset":3760,"parent_record_offset":3728,"ancestor_record_offset":2032,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D97B
// 首条位置: {"module":3707,"inlinee":121211}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3707,"record_offset":5656,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5628,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$44::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5536,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":30675,"source_line":1420,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$60::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3707,"record_offset":5448,"parent_record_offset":5404,"ancestor_record_offset":5404,"inlinee":30677,"source_line":1560,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3707,"record_offset":5568,"parent_record_offset":5536,"ancestor_record_offset":5404,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3707,"record_offset":5476,"parent_record_offset":5448,"ancestor_record_offset":5404,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::should_remove_tree_canopy_from_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1D979
// 首条位置: {"module":3707,"inlinee":121209}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::forest_tree_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3707,"record_offset":9956,"rva":36151504,"va":5404860624}
// 编译实例 RVAs: 0x227a0d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30507247","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8117f2efe","size":5,"soft_fail":false,"target_va":5374354992,"va":5404862234,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6679067","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e81bea6500","size":5,"soft_fail":false,"target_va":5411541584,"va":5404862512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6679767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8d7ec6500","size":5,"soft_fail":false,"target_va":5411542296,"va":5404862524,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8a4efffff","size":5,"soft_fail":false,"target_va":5404856544,"va":5404860727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6682558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8bef76500","size":5,"soft_fail":false,"target_va":5411543360,"va":5404860797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23475704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e808ca99fe","size":5,"soft_fail":false,"target_va":5381385536,"va":5404861235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23470518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e84ade99fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5404861153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23475918","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e832c999fe","size":5,"soft_fail":false,"target_va":5381386480,"va":5404862393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-23475994","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8e6c899fe","size":5,"soft_fail":false,"target_va":5381384704,"va":5404860693,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6946149","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e865fd6900","size":5,"soft_fail":false,"target_va":5411808608,"va":5404862454,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e84de7ffff","size":5,"soft_fail":false,"target_va":5404856400,"va":5404862718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6674599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e8a7d86500","size":5,"soft_fail":false,"target_va":5411537104,"va":5404862500,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-22955976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5404860624,"raw_hex":"e838b8a1fe","size":5,"soft_fail":false,"target_va":5381906160,"va":5404862131,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":16124,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":5305,"source_line":3369,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":16068,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":15544,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::writer::EventWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/writer.rs
//     evidence={"module":3707,"record_offset":15744,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":7657,"source_line":74,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":12936,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":12528,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3707,"record_offset":15656,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":16220,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":11560,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":10128,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":10584,"parent_record_offset":9956,"ancestor_record_offset":9956,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":12768,"parent_record_offset":12744,"ancestor_record_offset":9956,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":12816,"parent_record_offset":12792,"ancestor_record_offset":9956,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3707,"record_offset":12792,"parent_record_offset":12768,"ancestor_record_offset":9956,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":15904,"parent_record_offset":15864,"ancestor_record_offset":9956,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":12744,"parent_record_offset":12720,"ancestor_record_offset":9956,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":12720,"parent_record_offset":12688,"ancestor_record_offset":9956,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":13316,"parent_record_offset":13292,"ancestor_record_offset":9956,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":13340,"parent_record_offset":13316,"ancestor_record_offset":9956,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":15864,"parent_record_offset":15828,"ancestor_record_offset":9956,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3707,"record_offset":12688,"parent_record_offset":12660,"ancestor_record_offset":9956,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":9956,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":14768,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":14020,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":15048,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":26669,"source_line":317,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":15076,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3707,"record_offset":13936,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":15800,"parent_record_offset":15768,"ancestor_record_offset":9956,"inlinee":4459,"source_line":397,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":15768,"parent_record_offset":15744,"ancestor_record_offset":9956,"inlinee":4457,"source_line":434,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3707,"record_offset":15568,"parent_record_offset":15544,"ancestor_record_offset":9956,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::collections::Events::send_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/collections.rs
//     evidence={"module":3707,"record_offset":15828,"parent_record_offset":15768,"ancestor_record_offset":9956,"inlinee":7659,"source_line":128,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14676,"parent_record_offset":14652,"ancestor_record_offset":9956,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":9956,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":13404,"parent_record_offset":13376,"ancestor_record_offset":9956,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":9956,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14700,"parent_record_offset":14676,"ancestor_record_offset":9956,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14132,"parent_record_offset":14104,"ancestor_record_offset":9956,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14184,"parent_record_offset":14156,"ancestor_record_offset":9956,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":13488,"parent_record_offset":13376,"ancestor_record_offset":9956,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":13540,"parent_record_offset":13512,"ancestor_record_offset":9956,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$65::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3707,"record_offset":14628,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":17024,"source_line":2083,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":12636,"parent_record_offset":12552,"ancestor_record_offset":9956,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":12660,"parent_record_offset":12636,"ancestor_record_offset":9956,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3707,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":9956,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3707,"record_offset":14104,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":17025,"source_line":182,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3707,"record_offset":13376,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":17022,"source_line":195,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":13612,"parent_record_offset":13588,"ancestor_record_offset":9956,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":13588,"parent_record_offset":13564,"ancestor_record_offset":9956,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3707,"record_offset":13452,"parent_record_offset":13428,"ancestor_record_offset":9956,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":14432,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":9956,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":13564,"parent_record_offset":13540,"ancestor_record_offset":9956,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":13292,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":14724,"parent_record_offset":14700,"ancestor_record_offset":9956,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3707,"record_offset":13208,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":9956,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":9956,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3707,"record_offset":12580,"parent_record_offset":12552,"ancestor_record_offset":9956,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3707,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":9956,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3707,"record_offset":15500,"parent_record_offset":15128,"ancestor_record_offset":9956,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":11880,"parent_record_offset":11852,"ancestor_record_offset":9956,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":11932,"parent_record_offset":11908,"ancestor_record_offset":9956,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":11908,"parent_record_offset":11880,"ancestor_record_offset":9956,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":11828,"parent_record_offset":11800,"ancestor_record_offset":9956,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":11852,"parent_record_offset":11828,"ancestor_record_offset":9956,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":12024,"parent_record_offset":11852,"ancestor_record_offset":9956,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3707,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":9956,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3707,"record_offset":11956,"parent_record_offset":11932,"ancestor_record_offset":9956,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3707,"record_offset":10284,"parent_record_offset":10252,"ancestor_record_offset":9956,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3707,"record_offset":11696,"parent_record_offset":11656,"ancestor_record_offset":9956,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3707,"record_offset":13720,"parent_record_offset":13696,"ancestor_record_offset":9956,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3707,"record_offset":10532,"parent_record_offset":10504,"ancestor_record_offset":9956,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3707,"record_offset":12484,"parent_record_offset":11656,"ancestor_record_offset":9956,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":12088,"parent_record_offset":11800,"ancestor_record_offset":9956,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3707,"record_offset":12884,"parent_record_offset":12860,"ancestor_record_offset":9956,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":9956,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":9956,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":10972,"parent_record_offset":10948,"ancestor_record_offset":9956,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3707,"record_offset":11180,"parent_record_offset":11152,"ancestor_record_offset":9956,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3707,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":9956,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3707,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":9956,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3707,"record_offset":14964,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":15012,"parent_record_offset":14988,"ancestor_record_offset":9956,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":16152,"parent_record_offset":16124,"ancestor_record_offset":9956,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":13256,"parent_record_offset":13232,"ancestor_record_offset":9956,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":14988,"parent_record_offset":14964,"ancestor_record_offset":9956,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3707,"record_offset":16180,"parent_record_offset":16152,"ancestor_record_offset":9956,"inlinee":5488,"source_line":278,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3707,"record_offset":12860,"parent_record_offset":12688,"ancestor_record_offset":9956,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":9956,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":13144,"parent_record_offset":13120,"ancestor_record_offset":9956,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":10948,"parent_record_offset":10924,"ancestor_record_offset":9956,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":9956,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":13056,"parent_record_offset":12960,"ancestor_record_offset":9956,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":10900,"parent_record_offset":10784,"ancestor_record_offset":9956,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3707,"record_offset":13120,"parent_record_offset":13056,"ancestor_record_offset":9956,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3707,"record_offset":10252,"parent_record_offset":10224,"ancestor_record_offset":9956,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3707,"record_offset":10612,"parent_record_offset":10584,"ancestor_record_offset":9956,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3707,"record_offset":10752,"parent_record_offset":10720,"ancestor_record_offset":9956,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3707,"record_offset":10720,"parent_record_offset":10684,"ancestor_record_offset":9956,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":10660,"parent_record_offset":10636,"ancestor_record_offset":9956,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":9956,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":10636,"parent_record_offset":10612,"ancestor_record_offset":9956,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3707,"record_offset":11224,"parent_record_offset":10684,"ancestor_record_offset":9956,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":9956,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":9956,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10384,"parent_record_offset":10352,"ancestor_record_offset":9956,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10480,"parent_record_offset":10384,"ancestor_record_offset":9956,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10352,"parent_record_offset":10328,"ancestor_record_offset":9956,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10164,"parent_record_offset":10128,"ancestor_record_offset":9956,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3707,"record_offset":10504,"parent_record_offset":10480,"ancestor_record_offset":9956,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":10784,"parent_record_offset":10752,"ancestor_record_offset":9956,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":11068,"parent_record_offset":10784,"ancestor_record_offset":9956,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":11040,"parent_record_offset":10784,"ancestor_record_offset":9956,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3707,"record_offset":10844,"parent_record_offset":10784,"ancestor_record_offset":9956,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":11592,"parent_record_offset":11560,"ancestor_record_offset":9956,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":16284,"parent_record_offset":16252,"ancestor_record_offset":9956,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":16784,"parent_record_offset":16524,"ancestor_record_offset":9956,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3707,"record_offset":11724,"parent_record_offset":11696,"ancestor_record_offset":9956,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":11800,"parent_record_offset":11656,"ancestor_record_offset":9956,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3707,"record_offset":12060,"parent_record_offset":11800,"ancestor_record_offset":9956,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3707,"record_offset":11752,"parent_record_offset":11724,"ancestor_record_offset":9956,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3707,"record_offset":11656,"parent_record_offset":11624,"ancestor_record_offset":9956,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03707.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3707,"record_offset":11624,"parent_record_offset":11592,"ancestor_record_offset":9956,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03707.jsonl.gz"}
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
// 完整结构及原始类型记录: tree_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: tree_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: tree_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>

// ------------------------------------------------------------------
// 函数: forest::tree_animation::forest_tree_animation
// 信息 ID: 654377bcf07c6d2b38ce9444
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4e9f1c2a8ffbfe65ae9470c3: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0x1bf3d01 -> 0x227a0d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::writer::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > > @ 0x1bf76b8 -> 0x227a0d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation
// 信息 ID: 60cda5ebd06c75e7fc24b60b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 154a1c8c75146f343d5796ba: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> > @ 0x1bf395f -> 0x2279500 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> > > @ 0x1bf7b7f -> 0x2279500 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation::closure$0
// 信息 ID: 4bc4d535d7bf2281acb4d9ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "forest::tree_animation::handplaced_tree_animation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3707, inline_record=5404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::should_remove_tree_canopy_from_view
// 信息 ID: 730394583dc3e2a15cc218be
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3707, inline_record=2680; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
