// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/shared.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 7c2aaa2a48c683c2809da686d69de15ddec06f8c7808a27361b10c1b64cb5688
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shared.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3880
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
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::shared::critters_avoid_collisions
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3880,"record_offset":2044,"rva":36796832,"va":5405505952}
// 编译实例 RVAs: 0x23179a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29990805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e86b6036fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5405506832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ea800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26401793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8ff236dfe","size":5,"soft_fail":false,"target_va":5379106816,"va":5405508604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26228199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e819ca6ffe","size":5,"soft_fail":false,"target_va":5379280368,"va":5405508562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23596585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8d7f197fe","size":5,"soft_fail":false,"target_va":5381911904,"va":5405508484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cdc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23574601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8b74798fe","size":5,"soft_fail":false,"target_va":5381934528,"va":5405509124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14230e780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-38160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8f06affff","size":5,"soft_fail":false,"target_va":5405468544,"va":5405506699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6340378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e81abf6000","size":5,"soft_fail":false,"target_va":5411847152,"va":5405506769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::writer::EventWriter$LT$E$GT$::write
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3173557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8b56c3000","size":5,"soft_fail":false,"target_va":5408682976,"va":5405509414,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6031667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e833095c00","size":5,"soft_fail":false,"target_va":5411541584,"va":5405509912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6031807","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8bf095c00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405509964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5972478","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8fe215b00","size":5,"soft_fail":false,"target_va":5411482433,"va":5405509950,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1218","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e83efbffff","size":5,"soft_fail":false,"target_va":5405504896,"va":5405506109,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6037176","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8b81e5c00","size":5,"soft_fail":false,"target_va":5411543360,"va":5405506179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25092372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8ec1e81fe","size":5,"soft_fail":false,"target_va":5380417152,"va":5405509519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25716867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e87d9777fe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405509486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27572747","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8f5455bfe","size":5,"soft_fail":false,"target_va":5377936528,"va":5405509270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25375029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8cbce7cfe","size":5,"soft_fail":false,"target_va":5380131872,"va":5405506896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25636886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8eacf78fe","size":5,"soft_fail":false,"target_va":5379872560,"va":5405509441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24121111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8e9f08ffe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405506642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24115922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e82e0590fe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405506557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24123255","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e889e88ffe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405509730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24121381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8dbef8ffe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405506080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6301521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e851276000","size":5,"soft_fail":false,"target_va":5411811376,"va":5405509850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-42421","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e84b5affff","size":5,"soft_fail":false,"target_va":5405468144,"va":5405510560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6027199","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8bff75b00","size":5,"soft_fail":false,"target_va":5411537104,"va":5405509900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3174586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8ba703000","size":5,"soft_fail":false,"target_va":5408683088,"va":5405508497,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3173513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405505952,"raw_hex":"e8896c3000","size":5,"soft_fail":false,"target_va":5408683120,"va":5405509602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5405505952,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5405506789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":12084,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":5432,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":5020,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3880,"record_offset":13804,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":13136,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":4928,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":13916,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":13708,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3880,"record_offset":13596,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3880,"record_offset":4688,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3880,"record_offset":4964,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":31029,"source_line":301,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3880,"record_offset":12680,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":122488,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3880,"record_offset":4992,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":3880,"record_offset":12708,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":24398,"source_line":108,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12228,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12504,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12416,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12140,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":13396,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":12784,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":13164,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":3704,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":2272,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":2728,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":5264,"parent_record_offset":5240,"ancestor_record_offset":2044,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":5312,"parent_record_offset":5288,"ancestor_record_offset":2044,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":2044,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":2044,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":5216,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":5824,"parent_record_offset":5800,"ancestor_record_offset":2044,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":2044,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":5184,"parent_record_offset":5156,"ancestor_record_offset":2044,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":9736,"parent_record_offset":9708,"ancestor_record_offset":2044,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":11252,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":8228,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":8312,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":11532,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":8144,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":12108,"parent_record_offset":12084,"ancestor_record_offset":2044,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":11160,"parent_record_offset":11136,"ancestor_record_offset":2044,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":7056,"parent_record_offset":7024,"ancestor_record_offset":2044,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":8440,"parent_record_offset":8404,"ancestor_record_offset":2044,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":5920,"parent_record_offset":5884,"ancestor_record_offset":2044,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":2044,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":11184,"parent_record_offset":11160,"ancestor_record_offset":2044,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":8964,"parent_record_offset":8404,"ancestor_record_offset":2044,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":6412,"parent_record_offset":5884,"ancestor_record_offset":2044,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":11772,"parent_record_offset":11744,"ancestor_record_offset":2044,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":9644,"parent_record_offset":8404,"ancestor_record_offset":2044,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":9708,"parent_record_offset":9676,"ancestor_record_offset":2044,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":7024,"parent_record_offset":5884,"ancestor_record_offset":2044,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":7088,"parent_record_offset":7056,"ancestor_record_offset":2044,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$69::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":11112,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":72397,"source_line":2083,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":5132,"parent_record_offset":5044,"ancestor_record_offset":2044,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":5156,"parent_record_offset":5132,"ancestor_record_offset":2044,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":5456,"parent_record_offset":5432,"ancestor_record_offset":2044,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":8404,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":72430,"source_line":182,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":5884,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":72398,"source_line":195,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3880,"record_offset":6364,"parent_record_offset":6340,"ancestor_record_offset":2044,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":7172,"parent_record_offset":7144,"ancestor_record_offset":2044,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":6036,"parent_record_offset":6008,"ancestor_record_offset":2044,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":7144,"parent_record_offset":7116,"ancestor_record_offset":2044,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":6008,"parent_record_offset":5980,"ancestor_record_offset":2044,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":6224,"parent_record_offset":6200,"ancestor_record_offset":2044,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":10832,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":11681,"source_line":211,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":5980,"parent_record_offset":5948,"ancestor_record_offset":2044,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":2044,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":7116,"parent_record_offset":7088,"ancestor_record_offset":2044,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":5800,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":2044,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":5716,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3880,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":2044,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":2044,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":5100,"parent_record_offset":5076,"ancestor_record_offset":2044,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":5044,"parent_record_offset":5020,"ancestor_record_offset":2044,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":5076,"parent_record_offset":5044,"ancestor_record_offset":2044,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3880,"record_offset":11680,"parent_record_offset":11656,"ancestor_record_offset":2044,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3880,"record_offset":12036,"parent_record_offset":11584,"ancestor_record_offset":2044,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4040,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4092,"parent_record_offset":4068,"ancestor_record_offset":2044,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4068,"parent_record_offset":4040,"ancestor_record_offset":2044,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":3988,"parent_record_offset":3960,"ancestor_record_offset":2044,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":2044,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4184,"parent_record_offset":4012,"ancestor_record_offset":2044,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":4296,"parent_record_offset":4272,"ancestor_record_offset":2044,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3880,"record_offset":4756,"parent_record_offset":4716,"ancestor_record_offset":2044,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3880,"record_offset":12748,"parent_record_offset":12708,"ancestor_record_offset":2044,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3880,"record_offset":4116,"parent_record_offset":4092,"ancestor_record_offset":2044,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3880,"record_offset":2428,"parent_record_offset":2396,"ancestor_record_offset":2044,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":12444,"parent_record_offset":12416,"ancestor_record_offset":2044,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":12532,"parent_record_offset":12504,"ancestor_record_offset":2044,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":12168,"parent_record_offset":12140,"ancestor_record_offset":2044,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3880,"record_offset":3856,"parent_record_offset":3812,"ancestor_record_offset":2044,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":6156,"parent_record_offset":6128,"ancestor_record_offset":2044,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":7292,"parent_record_offset":7268,"ancestor_record_offset":2044,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":6200,"parent_record_offset":6008,"ancestor_record_offset":2044,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3880,"record_offset":2676,"parent_record_offset":2648,"ancestor_record_offset":2044,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3880,"record_offset":4644,"parent_record_offset":3812,"ancestor_record_offset":2044,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":4248,"parent_record_offset":3960,"ancestor_record_offset":2044,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3880,"record_offset":5380,"parent_record_offset":5356,"ancestor_record_offset":2044,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":5676,"parent_record_offset":5652,"ancestor_record_offset":2044,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2044,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2044,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":3324,"parent_record_offset":3296,"ancestor_record_offset":2044,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":2044,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3880,"record_offset":4840,"parent_record_offset":4812,"ancestor_record_offset":2044,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3880,"record_offset":4868,"parent_record_offset":4840,"ancestor_record_offset":2044,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":5740,"parent_record_offset":5716,"ancestor_record_offset":2044,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":11028,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":2044,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":11076,"parent_record_offset":11052,"ancestor_record_offset":2044,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":5764,"parent_record_offset":5740,"ancestor_record_offset":2044,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":11052,"parent_record_offset":11028,"ancestor_record_offset":2044,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3880,"record_offset":5356,"parent_record_offset":5184,"ancestor_record_offset":2044,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2044,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":5652,"parent_record_offset":5628,"ancestor_record_offset":2044,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":3092,"parent_record_offset":3068,"ancestor_record_offset":2044,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":3396,"parent_record_offset":3368,"ancestor_record_offset":2044,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":5564,"parent_record_offset":5456,"ancestor_record_offset":2044,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":3044,"parent_record_offset":2928,"ancestor_record_offset":2044,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":5628,"parent_record_offset":5564,"ancestor_record_offset":2044,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3880,"record_offset":2396,"parent_record_offset":2368,"ancestor_record_offset":2044,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3880,"record_offset":2756,"parent_record_offset":2728,"ancestor_record_offset":2044,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3880,"record_offset":2896,"parent_record_offset":2864,"ancestor_record_offset":2044,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3880,"record_offset":2864,"parent_record_offset":2828,"ancestor_record_offset":2044,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":2804,"parent_record_offset":2780,"ancestor_record_offset":2044,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2044,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":2780,"parent_record_offset":2756,"ancestor_record_offset":2044,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":3368,"parent_record_offset":2828,"ancestor_record_offset":2044,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2368,"parent_record_offset":2336,"ancestor_record_offset":2044,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":2044,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2528,"parent_record_offset":2496,"ancestor_record_offset":2044,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2624,"parent_record_offset":2528,"ancestor_record_offset":2044,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2496,"parent_record_offset":2472,"ancestor_record_offset":2044,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":4784,"parent_record_offset":4756,"ancestor_record_offset":2044,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2308,"parent_record_offset":2272,"ancestor_record_offset":2044,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2044,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":2928,"parent_record_offset":2896,"ancestor_record_offset":2044,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":3212,"parent_record_offset":2928,"ancestor_record_offset":2044,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":3184,"parent_record_offset":2928,"ancestor_record_offset":2044,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":2988,"parent_record_offset":2928,"ancestor_record_offset":2044,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3880,"record_offset":4716,"parent_record_offset":4688,"ancestor_record_offset":2044,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3880,"record_offset":13828,"parent_record_offset":13804,"ancestor_record_offset":2044,"inlinee":122488,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12336,"parent_record_offset":12304,"ancestor_record_offset":2044,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12304,"parent_record_offset":12280,"ancestor_record_offset":2044,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12280,"parent_record_offset":12252,"ancestor_record_offset":2044,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":12252,"parent_record_offset":12228,"ancestor_record_offset":2044,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":12812,"parent_record_offset":12784,"ancestor_record_offset":2044,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":13224,"parent_record_offset":13188,"ancestor_record_offset":2044,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":13188,"parent_record_offset":13164,"ancestor_record_offset":2044,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":13424,"parent_record_offset":13396,"ancestor_record_offset":2044,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3880,"record_offset":6248,"parent_record_offset":6224,"ancestor_record_offset":2044,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":3740,"parent_record_offset":3704,"ancestor_record_offset":2044,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":13980,"parent_record_offset":13948,"ancestor_record_offset":2044,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":14496,"parent_record_offset":14236,"ancestor_record_offset":2044,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3880,"record_offset":3884,"parent_record_offset":3856,"ancestor_record_offset":2044,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":3960,"parent_record_offset":3812,"ancestor_record_offset":2044,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":4220,"parent_record_offset":3960,"ancestor_record_offset":2044,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3880,"record_offset":12368,"parent_record_offset":12336,"ancestor_record_offset":2044,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3880,"record_offset":3912,"parent_record_offset":3884,"ancestor_record_offset":2044,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3880,"record_offset":3812,"parent_record_offset":3776,"ancestor_record_offset":2044,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3880,"record_offset":3776,"parent_record_offset":3740,"ancestor_record_offset":2044,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::shared::smooth_critter_velocities
// 记录形式: compiled_procedure; instances=1
// 源码行: 199
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3880,"record_offset":15088,"rva":36801552,"va":5405510672}
// 编译实例 RVAs: 0x2318c10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6337105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405510672,"raw_hex":"e851b26000","size":5,"soft_fail":false,"target_va":5411848048,"va":5405510938,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":19712,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":15644,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":15240,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":19360,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":24403,"source_line":759,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3880,"record_offset":19328,"parent_record_offset":15088,"ancestor_record_offset":15088,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":15476,"parent_record_offset":15452,"ancestor_record_offset":15088,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":15524,"parent_record_offset":15500,"ancestor_record_offset":15088,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":15500,"parent_record_offset":15476,"ancestor_record_offset":15088,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":15452,"parent_record_offset":15428,"ancestor_record_offset":15088,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":15428,"parent_record_offset":15396,"ancestor_record_offset":15088,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":17280,"parent_record_offset":17256,"ancestor_record_offset":15088,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":17304,"parent_record_offset":17280,"ancestor_record_offset":15088,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":15088,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":18424,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":19144,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":19228,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":18620,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":19060,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":19740,"parent_record_offset":19712,"ancestor_record_offset":15088,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16832,"parent_record_offset":16808,"ancestor_record_offset":15088,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":17896,"parent_record_offset":17340,"ancestor_record_offset":15088,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16288,"parent_record_offset":15768,"ancestor_record_offset":15088,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16808,"parent_record_offset":16784,"ancestor_record_offset":15088,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16856,"parent_record_offset":16832,"ancestor_record_offset":15088,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":17372,"parent_record_offset":17340,"ancestor_record_offset":15088,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":15800,"parent_record_offset":15768,"ancestor_record_offset":15088,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16956,"parent_record_offset":16784,"ancestor_record_offset":15088,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":15088,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":16784,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":15348,"parent_record_offset":15264,"ancestor_record_offset":15088,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":15088,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":15680,"parent_record_offset":15644,"ancestor_record_offset":15088,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":17340,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":15768,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3880,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":15088,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":15904,"parent_record_offset":15872,"ancestor_record_offset":15088,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":15872,"parent_record_offset":15840,"ancestor_record_offset":15088,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":16040,"parent_record_offset":16016,"ancestor_record_offset":15088,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":15840,"parent_record_offset":15800,"ancestor_record_offset":15088,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":16316,"parent_record_offset":16288,"ancestor_record_offset":15088,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":17256,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":16880,"parent_record_offset":16856,"ancestor_record_offset":15088,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":17172,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3880,"record_offset":16228,"parent_record_offset":15800,"ancestor_record_offset":15088,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3880,"record_offset":16172,"parent_record_offset":15800,"ancestor_record_offset":15088,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":16684,"parent_record_offset":16660,"ancestor_record_offset":15088,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3880,"record_offset":16252,"parent_record_offset":16228,"ancestor_record_offset":15088,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":15316,"parent_record_offset":15292,"ancestor_record_offset":15088,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":15264,"parent_record_offset":15240,"ancestor_record_offset":15088,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":15292,"parent_record_offset":15264,"ancestor_record_offset":15088,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3880,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":15088,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":19640,"parent_record_offset":19596,"ancestor_record_offset":15088,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":19448,"parent_record_offset":19396,"ancestor_record_offset":15088,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3880,"record_offset":15952,"parent_record_offset":15928,"ancestor_record_offset":15088,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":15928,"parent_record_offset":15904,"ancestor_record_offset":15088,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":15976,"parent_record_offset":15952,"ancestor_record_offset":15088,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":16016,"parent_record_offset":15872,"ancestor_record_offset":15088,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3880,"record_offset":15592,"parent_record_offset":15568,"ancestor_record_offset":15088,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":17136,"parent_record_offset":17100,"ancestor_record_offset":15088,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":16432,"parent_record_offset":16404,"ancestor_record_offset":15088,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":15088,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":16468,"parent_record_offset":16432,"ancestor_record_offset":15088,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":16096,"parent_record_offset":16064,"ancestor_record_offset":15088,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3880,"record_offset":15568,"parent_record_offset":15396,"ancestor_record_offset":15088,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":17100,"parent_record_offset":15680,"ancestor_record_offset":15088,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":19396,"parent_record_offset":19360,"ancestor_record_offset":15088,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":19596,"parent_record_offset":19360,"ancestor_record_offset":15088,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3880,"record_offset":16064,"parent_record_offset":16040,"ancestor_record_offset":15088,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::shared::move_critters
// 记录形式: compiled_procedure; instances=1
// 源码行: 214
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3880,"record_offset":19780,"rva":36802464,"va":5405511584}
// 编译实例 RVAs: 0x2318fa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=2
//     target_aliases=69 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29996444","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8644a36fe","size":5,"soft_fail":false,"target_va":5375516032,"va":5405512471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409ea800
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26408076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8740b6dfe","size":5,"soft_fail":false,"target_va":5379106816,"va":5405514887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a14df0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-26234468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e89cb16ffe","size":5,"soft_fail":false,"target_va":5379280368,"va":5405514831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c90030
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23633364","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e82c6297fe","size":5,"soft_fail":false,"target_va":5381881904,"va":5405515263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97560
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23602869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e84bd997fe","size":5,"soft_fail":false,"target_va":5381911904,"va":5405514768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14230e780
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-43793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8ef54ffff","size":5,"soft_fail":false,"target_va":5405468544,"va":5405512332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923bf0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6334739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e813a96000","size":5,"soft_fail":false,"target_va":5411847152,"va":5405512408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923f70
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6335383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e897ab6000","size":5,"soft_fail":false,"target_va":5411848048,"va":5405512660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-42705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e82f59ffff","size":5,"soft_fail":false,"target_va":5405469936,"va":5405512636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6024970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e80aef5b00","size":5,"soft_fail":false,"target_va":5411541584,"va":5405516609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6025107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e893ef5b00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405516664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5965778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8d2075b00","size":5,"soft_fail":false,"target_va":5411482433,"va":5405516650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e83de5ffff","size":5,"soft_fail":false,"target_va":5405504896,"va":5405511742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6031543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8b7085c00","size":5,"soft_fail":false,"target_va":5411543360,"va":5405511812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25097919","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8410981fe","size":5,"soft_fail":false,"target_va":5380417152,"va":5405515066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25722430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8c28177fe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405515049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27579609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8272b5bfe","size":5,"soft_fail":false,"target_va":5377936528,"va":5405516132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25642435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e83dba78fe","size":5,"soft_fail":false,"target_va":5379872560,"va":5405514990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-22868101","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e87b0fa3fe","size":5,"soft_fail":false,"target_va":5382647344,"va":5405515440,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24126731","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8f5da8ffe","size":5,"soft_fail":false,"target_va":5381385536,"va":5405512262,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24121542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e83aef8ffe","size":5,"soft_fail":false,"target_va":5381390640,"va":5405512177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24129952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e860ce8ffe","size":5,"soft_fail":false,"target_va":5381386480,"va":5405516427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-24127013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8dbd98ffe","size":5,"soft_fail":false,"target_va":5381384704,"va":5405511712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6294824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8280d6000","size":5,"soft_fail":false,"target_va":5411811376,"va":5405516547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-49121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e81f40ffff","size":5,"soft_fail":false,"target_va":5405468144,"va":5405517260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6020502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e896dd5b00","size":5,"soft_fail":false,"target_va":5411537104,"va":5405516597,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3168302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e82e583000","size":5,"soft_fail":false,"target_va":5408683088,"va":5405514781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3167966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405511584,"raw_hex":"e8de563000","size":5,"soft_fail":false,"target_va":5408683120,"va":5405515149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5405511584,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5405512428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":32560,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":9177,"source_line":434,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":23100,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":22792,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":17020,"source_line":701,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3880,"record_offset":33296,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":3880,"record_offset":32532,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":3880,"record_offset":22968,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":32616,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":22672,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":35728,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":33144,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":35124,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3880,"record_offset":33032,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3880,"record_offset":22432,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4445,"source_line":242,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3880,"record_offset":22708,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":31029,"source_line":301,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3880,"record_offset":32916,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":122488,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3880,"record_offset":22736,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":25412,"source_line":158,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3880,"record_offset":33724,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":77645,"source_line":635,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":3880,"record_offset":32740,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":35440,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":35204,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":13657,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33532,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":12697,"source_line":538,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":23000,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33444,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":13659,"source_line":1368,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33696,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":13727,"source_line":1809,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":21448,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4416,"source_line":589,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":20016,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4368,"source_line":626,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":20472,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":4385,"source_line":659,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3880,"record_offset":22940,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":11915,"source_line":339,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3880,"record_offset":34572,"parent_record_offset":19780,"ancestor_record_offset":19780,"inlinee":24721,"source_line":29,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":31520,"parent_record_offset":31496,"ancestor_record_offset":19780,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":31568,"parent_record_offset":31544,"ancestor_record_offset":19780,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3880,"record_offset":31544,"parent_record_offset":31520,"ancestor_record_offset":19780,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":31496,"parent_record_offset":31472,"ancestor_record_offset":19780,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":31472,"parent_record_offset":31448,"ancestor_record_offset":19780,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":23496,"parent_record_offset":23472,"ancestor_record_offset":19780,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":23520,"parent_record_offset":23496,"ancestor_record_offset":19780,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3880,"record_offset":31448,"parent_record_offset":31424,"ancestor_record_offset":19780,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":28264,"parent_record_offset":28236,"ancestor_record_offset":19780,"inlinee":5673,"source_line":621,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":31424,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":26824,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":26908,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":31620,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3880,"record_offset":26740,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/change_detection.rs
//     evidence={"module":3880,"record_offset":32584,"parent_record_offset":32560,"ancestor_record_offset":19780,"inlinee":9179,"source_line":397,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30616,"parent_record_offset":30592,"ancestor_record_offset":19780,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::matches_component_set
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":24664,"parent_record_offset":24628,"ancestor_record_offset":19780,"inlinee":5720,"source_line":1210,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":27156,"parent_record_offset":27000,"ancestor_record_offset":19780,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":23680,"parent_record_offset":23556,"ancestor_record_offset":19780,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30800,"parent_record_offset":30568,"ancestor_record_offset":19780,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30848,"parent_record_offset":30824,"ancestor_record_offset":19780,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":25968,"parent_record_offset":25936,"ancestor_record_offset":19780,"inlinee":10339,"source_line":1577,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":27044,"parent_record_offset":27000,"ancestor_record_offset":19780,"inlinee":13655,"source_line":1524,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":23596,"parent_record_offset":23556,"ancestor_record_offset":19780,"inlinee":13656,"source_line":1539,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30592,"parent_record_offset":30568,"ancestor_record_offset":19780,"inlinee":13686,"source_line":1596,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30640,"parent_record_offset":30616,"ancestor_record_offset":19780,"inlinee":9533,"source_line":1602,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":28164,"parent_record_offset":27000,"ancestor_record_offset":19780,"inlinee":17008,"source_line":1799,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_archetype::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":28236,"parent_record_offset":28200,"ancestor_record_offset":19780,"inlinee":17010,"source_line":1805,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":24628,"parent_record_offset":23556,"ancestor_record_offset":19780,"inlinee":17014,"source_line":1815,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$41::set_table::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":24700,"parent_record_offset":24664,"ancestor_record_offset":19780,"inlinee":17016,"source_line":1816,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":31080,"parent_record_offset":30568,"ancestor_record_offset":19780,"inlinee":26664,"source_line":1867,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$42::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":31184,"parent_record_offset":31160,"ancestor_record_offset":19780,"inlinee":26666,"source_line":1875,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$71::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3880,"record_offset":30568,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":97708,"source_line":2083,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":22900,"parent_record_offset":22816,"ancestor_record_offset":19780,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3880,"record_offset":23124,"parent_record_offset":23100,"ancestor_record_offset":19780,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":27000,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":97706,"source_line":182,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3880,"record_offset":23556,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":97709,"source_line":195,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3880,"record_offset":24092,"parent_record_offset":24068,"ancestor_record_offset":19780,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":24784,"parent_record_offset":24756,"ancestor_record_offset":19780,"inlinee":5675,"source_line":49,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":23780,"parent_record_offset":23756,"ancestor_record_offset":19780,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":24756,"parent_record_offset":24728,"ancestor_record_offset":19780,"inlinee":5674,"source_line":388,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":23644,"parent_record_offset":23620,"ancestor_record_offset":19780,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3880,"record_offset":23952,"parent_record_offset":23928,"ancestor_record_offset":19780,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":23620,"parent_record_offset":23596,"ancestor_record_offset":19780,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":23704,"parent_record_offset":23680,"ancestor_record_offset":19780,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":24728,"parent_record_offset":24700,"ancestor_record_offset":19780,"inlinee":17017,"source_line":496,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":23472,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":30668,"parent_record_offset":30640,"ancestor_record_offset":19780,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":23388,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3880,"record_offset":26520,"parent_record_offset":26232,"ancestor_record_offset":19780,"inlinee":13683,"source_line":319,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3880,"record_offset":24068,"parent_record_offset":24044,"ancestor_record_offset":19780,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3880,"record_offset":24044,"parent_record_offset":24020,"ancestor_record_offset":19780,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/thin_array_ptr.rs
//     evidence={"module":3880,"record_offset":26544,"parent_record_offset":26520,"ancestor_record_offset":19780,"inlinee":13684,"source_line":300,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":22868,"parent_record_offset":22844,"ancestor_record_offset":19780,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":22816,"parent_record_offset":22792,"ancestor_record_offset":19780,"inlinee":17021,"source_line":601,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3880,"record_offset":22844,"parent_record_offset":22816,"ancestor_record_offset":19780,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3880,"record_offset":30732,"parent_record_offset":30640,"ancestor_record_offset":19780,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3880,"record_offset":31104,"parent_record_offset":31080,"ancestor_record_offset":19780,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21784,"parent_record_offset":21756,"ancestor_record_offset":19780,"inlinee":4432,"source_line":1817,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21836,"parent_record_offset":21812,"ancestor_record_offset":19780,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21812,"parent_record_offset":21784,"ancestor_record_offset":19780,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21732,"parent_record_offset":21704,"ancestor_record_offset":19780,"inlinee":4429,"source_line":1089,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21756,"parent_record_offset":21732,"ancestor_record_offset":19780,"inlinee":4430,"source_line":1121,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":21928,"parent_record_offset":21756,"ancestor_record_offset":19780,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$44::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":3880,"record_offset":22040,"parent_record_offset":22016,"ancestor_record_offset":19780,"inlinee":4443,"source_line":1808,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3880,"record_offset":22500,"parent_record_offset":22460,"ancestor_record_offset":19780,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":33960,"parent_record_offset":33936,"ancestor_record_offset":19780,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_and_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34456,"parent_record_offset":33724,"ancestor_record_offset":19780,"inlinee":37284,"source_line":244,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34232,"parent_record_offset":33992,"ancestor_record_offset":19780,"inlinee":28547,"source_line":105,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":33848,"parent_record_offset":33824,"ancestor_record_offset":19780,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34832,"parent_record_offset":34808,"ancestor_record_offset":19780,"inlinee":24651,"source_line":910,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34104,"parent_record_offset":34048,"ancestor_record_offset":19780,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34204,"parent_record_offset":33992,"ancestor_record_offset":19780,"inlinee":44716,"source_line":129,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3880,"record_offset":34484,"parent_record_offset":33724,"ancestor_record_offset":19780,"inlinee":13674,"source_line":301,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":3880,"record_offset":21860,"parent_record_offset":21836,"ancestor_record_offset":19780,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\mod.rs
//     evidence={"module":3880,"record_offset":20172,"parent_record_offset":20140,"ancestor_record_offset":19780,"inlinee":4377,"source_line":1044,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":32768,"parent_record_offset":32740,"ancestor_record_offset":19780,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":23036,"parent_record_offset":23000,"ancestor_record_offset":19780,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":33472,"parent_record_offset":33444,"ancestor_record_offset":19780,"inlinee":13661,"source_line":761,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":3880,"record_offset":35508,"parent_record_offset":35468,"ancestor_record_offset":19780,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3880,"record_offset":21600,"parent_record_offset":21556,"ancestor_record_offset":19780,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":30196,"parent_record_offset":30172,"ancestor_record_offset":19780,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":23888,"parent_record_offset":23864,"ancestor_record_offset":19780,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":24904,"parent_record_offset":24880,"ancestor_record_offset":19780,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3880,"record_offset":23928,"parent_record_offset":23756,"ancestor_record_offset":19780,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3880,"record_offset":20420,"parent_record_offset":20392,"ancestor_record_offset":19780,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3880,"record_offset":22388,"parent_record_offset":21556,"ancestor_record_offset":19780,"inlinee":4795,"source_line":296,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":21992,"parent_record_offset":21704,"ancestor_record_offset":19780,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3880,"record_offset":30700,"parent_record_offset":30668,"ancestor_record_offset":19780,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":23348,"parent_record_offset":23324,"ancestor_record_offset":19780,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":20884,"parent_record_offset":20860,"ancestor_record_offset":19780,"inlinee":4410,"source_line":533,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":20860,"parent_record_offset":20836,"ancestor_record_offset":19780,"inlinee":4409,"source_line":686,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3880,"record_offset":21068,"parent_record_offset":21040,"ancestor_record_offset":19780,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":3880,"record_offset":21884,"parent_record_offset":21860,"ancestor_record_offset":19780,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3880,"record_offset":22584,"parent_record_offset":22556,"ancestor_record_offset":19780,"inlinee":4452,"source_line":14,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\cmp.rs
//     evidence={"module":3880,"record_offset":22612,"parent_record_offset":22584,"ancestor_record_offset":19780,"inlinee":4454,"source_line":82,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":23412,"parent_record_offset":23388,"ancestor_record_offset":19780,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3880,"record_offset":33056,"parent_record_offset":33032,"ancestor_record_offset":19780,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":26412,"parent_record_offset":26388,"ancestor_record_offset":19780,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":23436,"parent_record_offset":23412,"ancestor_record_offset":19780,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3880,"record_offset":26388,"parent_record_offset":26356,"ancestor_record_offset":19780,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3880,"record_offset":33080,"parent_record_offset":33056,"ancestor_record_offset":19780,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":20812,"parent_record_offset":20788,"ancestor_record_offset":19780,"inlinee":4407,"source_line":80,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":23324,"parent_record_offset":23300,"ancestor_record_offset":19780,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":20836,"parent_record_offset":20812,"ancestor_record_offset":19780,"inlinee":4408,"source_line":118,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":21140,"parent_record_offset":21112,"ancestor_record_offset":19780,"inlinee":4402,"source_line":279,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":23236,"parent_record_offset":23124,"ancestor_record_offset":19780,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$183::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":20788,"parent_record_offset":20672,"ancestor_record_offset":19780,"inlinee":4405,"source_line":392,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3880,"record_offset":23300,"parent_record_offset":23236,"ancestor_record_offset":19780,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3880,"record_offset":20140,"parent_record_offset":20112,"ancestor_record_offset":19780,"inlinee":4375,"source_line":366,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\mod.rs
//     evidence={"module":3880,"record_offset":20500,"parent_record_offset":20472,"ancestor_record_offset":19780,"inlinee":4386,"source_line":1488,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3880,"record_offset":20640,"parent_record_offset":20608,"ancestor_record_offset":19780,"inlinee":4396,"source_line":128,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\iter.rs
//     evidence={"module":3880,"record_offset":20608,"parent_record_offset":20572,"ancestor_record_offset":19780,"inlinee":4394,"source_line":218,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":20548,"parent_record_offset":20524,"ancestor_record_offset":19780,"inlinee":4390,"source_line":315,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":20572,"parent_record_offset":20548,"ancestor_record_offset":19780,"inlinee":4392,"source_line":709,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":20524,"parent_record_offset":20500,"ancestor_record_offset":19780,"inlinee":4388,"source_line":798,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\pattern.rs
//     evidence={"module":3880,"record_offset":21112,"parent_record_offset":20572,"ancestor_record_offset":19780,"inlinee":4400,"source_line":659,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20112,"parent_record_offset":20080,"ancestor_record_offset":19780,"inlinee":4373,"source_line":433,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20080,"parent_record_offset":20052,"ancestor_record_offset":19780,"inlinee":4372,"source_line":463,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20272,"parent_record_offset":20240,"ancestor_record_offset":19780,"inlinee":4380,"source_line":501,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20368,"parent_record_offset":20272,"ancestor_record_offset":19780,"inlinee":4381,"source_line":521,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20240,"parent_record_offset":20216,"ancestor_record_offset":19780,"inlinee":4379,"source_line":533,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":22528,"parent_record_offset":22500,"ancestor_record_offset":19780,"inlinee":4450,"source_line":28,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20052,"parent_record_offset":20016,"ancestor_record_offset":19780,"inlinee":4370,"source_line":59,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\traits.rs
//     evidence={"module":3880,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":19780,"inlinee":4383,"source_line":189,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":20672,"parent_record_offset":20640,"ancestor_record_offset":19780,"inlinee":4398,"source_line":79,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":20956,"parent_record_offset":20672,"ancestor_record_offset":19780,"inlinee":4412,"source_line":17,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":20928,"parent_record_offset":20672,"ancestor_record_offset":19780,"inlinee":4411,"source_line":11,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\str\validations.rs
//     evidence={"module":3880,"record_offset":20732,"parent_record_offset":20672,"ancestor_record_offset":19780,"inlinee":4403,"source_line":24,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3880,"record_offset":22460,"parent_record_offset":22432,"ancestor_record_offset":19780,"inlinee":4446,"source_line":206,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$19::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3880,"record_offset":33320,"parent_record_offset":33296,"ancestor_record_offset":19780,"inlinee":122488,"source_line":87,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/affine3a.rs
//     evidence={"module":3880,"record_offset":34600,"parent_record_offset":34572,"ancestor_record_offset":19780,"inlinee":24722,"source_line":233,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/mat3a.rs
//     evidence={"module":3880,"record_offset":34640,"parent_record_offset":34600,"ancestor_record_offset":19780,"inlinee":24723,"source_line":270,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::lerp_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3880,"record_offset":33760,"parent_record_offset":33724,"ancestor_record_offset":19780,"inlinee":122489,"source_line":619,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$5::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":3880,"record_offset":33788,"parent_record_offset":33760,"ancestor_record_offset":19780,"inlinee":122491,"source_line":867,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3880,"record_offset":34720,"parent_record_offset":34640,"ancestor_record_offset":19780,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":3880,"record_offset":34808,"parent_record_offset":34600,"ancestor_record_offset":19780,"inlinee":24650,"source_line":1245,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3880,"record_offset":33992,"parent_record_offset":33788,"ancestor_record_offset":19780,"inlinee":50548,"source_line":532,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3880,"record_offset":33824,"parent_record_offset":33788,"ancestor_record_offset":19780,"inlinee":25141,"source_line":1167,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":3880,"record_offset":33936,"parent_record_offset":33788,"ancestor_record_offset":19780,"inlinee":28544,"source_line":1245,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33620,"parent_record_offset":33588,"ancestor_record_offset":19780,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33588,"parent_record_offset":33564,"ancestor_record_offset":19780,"inlinee":4894,"source_line":451,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":33564,"parent_record_offset":33532,"ancestor_record_offset":19780,"inlinee":4893,"source_line":470,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":3880,"record_offset":35468,"parent_record_offset":35440,"ancestor_record_offset":19780,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3880,"record_offset":34048,"parent_record_offset":34020,"ancestor_record_offset":19780,"inlinee":50550,"source_line":52,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":3880,"record_offset":34020,"parent_record_offset":33992,"ancestor_record_offset":19780,"inlinee":50549,"source_line":77,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3880,"record_offset":23976,"parent_record_offset":23952,"ancestor_record_offset":19780,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":21484,"parent_record_offset":21448,"ancestor_record_offset":19780,"inlinee":4418,"source_line":425,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":35792,"parent_record_offset":35760,"ancestor_record_offset":19780,"inlinee":4966,"source_line":601,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":36308,"parent_record_offset":36048,"ancestor_record_offset":19780,"inlinee":4968,"source_line":602,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":3880,"record_offset":21628,"parent_record_offset":21600,"ancestor_record_offset":19780,"inlinee":4423,"source_line":90,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":21704,"parent_record_offset":21556,"ancestor_record_offset":19780,"inlinee":4427,"source_line":429,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3880,"record_offset":21964,"parent_record_offset":21704,"ancestor_record_offset":19780,"inlinee":4441,"source_line":591,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":3880,"record_offset":33652,"parent_record_offset":33620,"ancestor_record_offset":19780,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":3880,"record_offset":21656,"parent_record_offset":21628,"ancestor_record_offset":19780,"inlinee":4425,"source_line":51,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3880,"record_offset":21556,"parent_record_offset":21520,"ancestor_record_offset":19780,"inlinee":4421,"source_line":305,"archive":"_pdb/B/modules/03880.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\thread\local.rs
//     evidence={"module":3880,"record_offset":21520,"parent_record_offset":21484,"ancestor_record_offset":19780,"inlinee":4420,"source_line":270,"archive":"_pdb/B/modules/03880.jsonl.gz"}
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
// 完整结构及原始类型记录: shared.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shared.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shared.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "country_core::systems::paths::draw_path::PathRastersResources", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>

// ------------------------------------------------------------------
// 函数: system_critters::shared::critters_avoid_collisions
// 信息 ID: b05fc3abe9c82861e7f3ea88
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 73c79c6894276771ead5156c: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > > @ 0x1da50c8 -> 0x23179a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::writer::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x1daa7fa -> 0x23179a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::move_critters
// 信息 ID: 2a8d722cb0f3080a2ba5b7d0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 529f3daa130b01f5d1602f50: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > > @ 0x1da33ca -> 0x2318fa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x1daafc2 -> 0x2318fa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::smooth_critter_velocities
// 信息 ID: fe2285c142c7ed305fe1b02f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4afd1c41309402f293e71f20: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > > @ 0x1da41e5 -> 0x2318c10 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1daab2c -> 0x2318c10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
