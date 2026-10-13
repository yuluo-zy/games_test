// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/systems/critters/src/shared.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 90f34e6e2319082e6aecdb9fa122c0691df1f7eec23f50c21963621f188c67bb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shared.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 4164
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_ptr; // namespace candidate
//   package identities: [{"name":"bevy_ptr","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
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
// 首条位置: {"module":4164,"record_offset":19160,"rva":43215280,"va":5411924400}
// 编译实例 RVAs: 0x29369b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=1
//     target_aliases=77 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36096007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8f937d9fd","size":5,"soft_fail":false,"target_va":5375829056,"va":5411925058,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c714c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30170803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e84da133fe","size":5,"soft_fail":false,"target_va":5381756096,"va":5411926894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=9
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30059559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8d95335fe","size":5,"soft_fail":false,"target_va":5381867296,"va":5411926850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deb2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28622880","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8e03f4bfe","size":5,"soft_fail":false,"target_va":5383303904,"va":5411926779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28602383","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8f18f4bfe","size":5,"soft_fail":false,"target_va":5383325024,"va":5411927402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142927b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-61620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e84c0fffff","size":5,"soft_fail":false,"target_va":5411863376,"va":5411924991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4423365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8c57e4300","size":5,"soft_fail":false,"target_va":5416351056,"va":5411927686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4693243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8fb9c4700","size":5,"soft_fail":false,"target_va":5416621408,"va":5411928160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4693450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8ca9d4700","size":5,"soft_fail":false,"target_va":5416621600,"va":5411928145,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8fa140000","size":5,"soft_fail":false,"target_va":5411929936,"va":5411924561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4693108","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8749c4700","size":5,"soft_fail":false,"target_va":5416621376,"va":5411928263,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30696019","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8ad9d2bfe","size":5,"soft_fail":false,"target_va":5381231824,"va":5411927838,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31024273","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e86f9b26fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5411927804,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32763511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e889110cfe","size":5,"soft_fail":false,"target_va":5379164016,"va":5411927522,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::paths::draw_path::PathRastersResources::as_float_grid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30514248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8b8632efe","size":5,"soft_fail":false,"target_va":5381410880,"va":5411925123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31934330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e886b818fe","size":5,"soft_fail":false,"target_va":5379993424,"va":5411927749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27961146","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8c65855fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5411924933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27952639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8017a55fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5411924858,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27963117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8135155fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5411928024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27961717","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e88b5655fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5411924528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5109089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e861f54d00","size":5,"soft_fail":false,"target_va":5417037264,"va":5411928170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-61013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8ab11ffff","size":5,"soft_fail":false,"target_va":5411867888,"va":5411928896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4691143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e8c7944700","size":5,"soft_fail":false,"target_va":5416619360,"va":5411928212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2037586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e852171f00","size":5,"soft_fail":false,"target_va":5413964384,"va":5411926793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2036492","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411924400,"raw_hex":"e80c131f00","size":5,"soft_fail":false,"target_va":5413964416,"va":5411927919,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5411924400,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5411925013,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":28832,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::message::message_writer::MessageWriter::write
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/message_writer.rs
//     evidence={"module":4164,"record_offset":30464,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":11206,"source_line":69,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":22196,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":21756,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4164,"record_offset":31024,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":29948,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":21664,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4164,"record_offset":31136,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":30932,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4164,"record_offset":30820,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4164,"record_offset":21604,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":4164,"record_offset":21700,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":30004,"source_line":310,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":4164,"record_offset":29456,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":136421,"source_line":87,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":4164,"record_offset":21728,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/paths/draw_path.rs
//     evidence={"module":4164,"record_offset":29484,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":26172,"source_line":155,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":28976,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29280,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8712,"source_line":1309,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29192,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":28888,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":30240,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":29564,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":29980,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":20796,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":19392,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":19784,"parent_record_offset":19160,"ancestor_record_offset":19160,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":22028,"parent_record_offset":22004,"ancestor_record_offset":19160,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":22076,"parent_record_offset":22052,"ancestor_record_offset":19160,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":22052,"parent_record_offset":22028,"ancestor_record_offset":19160,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":30652,"parent_record_offset":30612,"ancestor_record_offset":19160,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":22004,"parent_record_offset":21980,"ancestor_record_offset":19160,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":21980,"parent_record_offset":21948,"ancestor_record_offset":19160,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":22532,"parent_record_offset":22508,"ancestor_record_offset":19160,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":22556,"parent_record_offset":22532,"ancestor_record_offset":19160,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":30584,"parent_record_offset":30548,"ancestor_record_offset":19160,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":30612,"parent_record_offset":30584,"ancestor_record_offset":19160,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":21948,"parent_record_offset":21920,"ancestor_record_offset":19160,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":26536,"parent_record_offset":26508,"ancestor_record_offset":19160,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":27860,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":24952,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":25036,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":28112,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":24868,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":30520,"parent_record_offset":30488,"ancestor_record_offset":19160,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":30488,"parent_record_offset":30464,"ancestor_record_offset":19160,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":28856,"parent_record_offset":28832,"ancestor_record_offset":19160,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::message::messages::Messages::write_with_caller
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/message/messages.rs
//     evidence={"module":4164,"record_offset":30548,"parent_record_offset":30488,"ancestor_record_offset":19160,"inlinee":8681,"source_line":129,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":28188,"parent_record_offset":28164,"ancestor_record_offset":19160,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$101::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":28140,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":81359,"source_line":2520,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":23836,"parent_record_offset":23804,"ancestor_record_offset":19160,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":25164,"parent_record_offset":25128,"ancestor_record_offset":19160,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":22628,"parent_record_offset":22592,"ancestor_record_offset":19160,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":28164,"parent_record_offset":28140,"ancestor_record_offset":19160,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":28212,"parent_record_offset":28188,"ancestor_record_offset":19160,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":25724,"parent_record_offset":25128,"ancestor_record_offset":19160,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":23156,"parent_record_offset":22592,"ancestor_record_offset":19160,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":28520,"parent_record_offset":28492,"ancestor_record_offset":19160,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":26440,"parent_record_offset":25128,"ancestor_record_offset":19160,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":26508,"parent_record_offset":26476,"ancestor_record_offset":19160,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":23804,"parent_record_offset":22592,"ancestor_record_offset":19160,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":23868,"parent_record_offset":23836,"ancestor_record_offset":19160,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":21896,"parent_record_offset":21780,"ancestor_record_offset":19160,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":21920,"parent_record_offset":21896,"ancestor_record_offset":19160,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":22220,"parent_record_offset":22196,"ancestor_record_offset":19160,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":25128,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":81407,"source_line":182,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":22592,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":81361,"source_line":195,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4164,"record_offset":23108,"parent_record_offset":23084,"ancestor_record_offset":19160,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":23952,"parent_record_offset":23924,"ancestor_record_offset":19160,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":22744,"parent_record_offset":22716,"ancestor_record_offset":19160,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":23924,"parent_record_offset":23896,"ancestor_record_offset":19160,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":22716,"parent_record_offset":22688,"ancestor_record_offset":19160,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":22932,"parent_record_offset":22908,"ancestor_record_offset":19160,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":27608,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10328,"source_line":208,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":22688,"parent_record_offset":22656,"ancestor_record_offset":19160,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":22656,"parent_record_offset":22628,"ancestor_record_offset":19160,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":23896,"parent_record_offset":23868,"ancestor_record_offset":19160,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":22508,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":28236,"parent_record_offset":28212,"ancestor_record_offset":19160,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":22424,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4164,"record_offset":23084,"parent_record_offset":23060,"ancestor_record_offset":19160,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":23060,"parent_record_offset":23036,"ancestor_record_offset":19160,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":21836,"parent_record_offset":21812,"ancestor_record_offset":19160,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":21860,"parent_record_offset":21836,"ancestor_record_offset":19160,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":21780,"parent_record_offset":21756,"ancestor_record_offset":19160,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":21812,"parent_record_offset":21780,"ancestor_record_offset":19160,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4164,"record_offset":28428,"parent_record_offset":28404,"ancestor_record_offset":19160,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4164,"record_offset":28784,"parent_record_offset":28332,"ancestor_record_offset":19160,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21124,"parent_record_offset":21096,"ancestor_record_offset":19160,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21152,"parent_record_offset":21124,"ancestor_record_offset":19160,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21072,"parent_record_offset":21044,"ancestor_record_offset":19160,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21096,"parent_record_offset":21072,"ancestor_record_offset":19160,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21212,"parent_record_offset":21096,"ancestor_record_offset":19160,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":21324,"parent_record_offset":21300,"ancestor_record_offset":19160,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4164,"record_offset":29528,"parent_record_offset":29484,"ancestor_record_offset":19160,"inlinee":9255,"source_line":1503,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4164,"record_offset":29140,"parent_record_offset":29116,"ancestor_record_offset":19160,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4164,"record_offset":21176,"parent_record_offset":21152,"ancestor_record_offset":19160,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":29220,"parent_record_offset":29192,"ancestor_record_offset":19160,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":29308,"parent_record_offset":29280,"ancestor_record_offset":19160,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":28916,"parent_record_offset":28888,"ancestor_record_offset":19160,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4164,"record_offset":20944,"parent_record_offset":20904,"ancestor_record_offset":19160,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":22864,"parent_record_offset":22836,"ancestor_record_offset":19160,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":24072,"parent_record_offset":24048,"ancestor_record_offset":19160,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":22908,"parent_record_offset":22716,"ancestor_record_offset":19160,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4164,"record_offset":19732,"parent_record_offset":19704,"ancestor_record_offset":19160,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4164,"record_offset":21560,"parent_record_offset":20904,"ancestor_record_offset":19160,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4164,"record_offset":21276,"parent_record_offset":21044,"ancestor_record_offset":19160,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4164,"record_offset":22144,"parent_record_offset":22120,"ancestor_record_offset":19160,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":22392,"parent_record_offset":22332,"ancestor_record_offset":19160,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":20204,"parent_record_offset":20180,"ancestor_record_offset":19160,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":20180,"parent_record_offset":20156,"ancestor_record_offset":19160,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":20388,"parent_record_offset":20360,"ancestor_record_offset":19160,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4164,"record_offset":30772,"parent_record_offset":30612,"ancestor_record_offset":19160,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4164,"record_offset":20488,"parent_record_offset":20460,"ancestor_record_offset":19160,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":20460,"parent_record_offset":20432,"ancestor_record_offset":19160,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":22448,"parent_record_offset":22424,"ancestor_record_offset":19160,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":27804,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":30844,"parent_record_offset":30820,"ancestor_record_offset":19160,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":22472,"parent_record_offset":22448,"ancestor_record_offset":19160,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":22988,"parent_record_offset":22956,"ancestor_record_offset":19160,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4164,"record_offset":22120,"parent_record_offset":21948,"ancestor_record_offset":19160,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":19160,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":20156,"parent_record_offset":20132,"ancestor_record_offset":19160,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":20516,"parent_record_offset":20488,"ancestor_record_offset":19160,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":22332,"parent_record_offset":22220,"ancestor_record_offset":19160,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":20108,"parent_record_offset":19992,"ancestor_record_offset":19160,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4164,"record_offset":19520,"parent_record_offset":19496,"ancestor_record_offset":19160,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4164,"record_offset":19820,"parent_record_offset":19784,"ancestor_record_offset":19160,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4164,"record_offset":19960,"parent_record_offset":19928,"ancestor_record_offset":19160,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4164,"record_offset":19928,"parent_record_offset":19892,"ancestor_record_offset":19160,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":19160,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":19892,"parent_record_offset":19868,"ancestor_record_offset":19160,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":19844,"parent_record_offset":19820,"ancestor_record_offset":19160,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":20432,"parent_record_offset":19892,"ancestor_record_offset":19160,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19496,"parent_record_offset":19464,"ancestor_record_offset":19160,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19464,"parent_record_offset":19436,"ancestor_record_offset":19160,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19624,"parent_record_offset":19592,"ancestor_record_offset":19160,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19680,"parent_record_offset":19624,"ancestor_record_offset":19160,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19592,"parent_record_offset":19560,"ancestor_record_offset":19160,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19436,"parent_record_offset":19392,"ancestor_record_offset":19160,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":19704,"parent_record_offset":19680,"ancestor_record_offset":19160,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":19992,"parent_record_offset":19960,"ancestor_record_offset":19160,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":20276,"parent_record_offset":19992,"ancestor_record_offset":19160,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":20248,"parent_record_offset":19992,"ancestor_record_offset":19160,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":20052,"parent_record_offset":19992,"ancestor_record_offset":19160,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4164,"record_offset":21628,"parent_record_offset":21604,"ancestor_record_offset":19160,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":4164,"record_offset":31048,"parent_record_offset":31024,"ancestor_record_offset":19160,"inlinee":136421,"source_line":87,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29084,"parent_record_offset":29052,"ancestor_record_offset":19160,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29052,"parent_record_offset":29028,"ancestor_record_offset":19160,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29028,"parent_record_offset":29000,"ancestor_record_offset":19160,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":29000,"parent_record_offset":28976,"ancestor_record_offset":19160,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":29592,"parent_record_offset":29564,"ancestor_record_offset":19160,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":30040,"parent_record_offset":30004,"ancestor_record_offset":19160,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":30004,"parent_record_offset":29980,"ancestor_record_offset":19160,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":30264,"parent_record_offset":30240,"ancestor_record_offset":19160,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":28260,"parent_record_offset":28236,"ancestor_record_offset":19160,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":22956,"parent_record_offset":22932,"ancestor_record_offset":19160,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":20832,"parent_record_offset":20796,"ancestor_record_offset":19160,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":31200,"parent_record_offset":31168,"ancestor_record_offset":19160,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":31652,"parent_record_offset":31448,"ancestor_record_offset":19160,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4164,"record_offset":20972,"parent_record_offset":20944,"ancestor_record_offset":19160,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":21044,"parent_record_offset":20904,"ancestor_record_offset":19160,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":21248,"parent_record_offset":21044,"ancestor_record_offset":19160,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4164,"record_offset":29116,"parent_record_offset":29084,"ancestor_record_offset":19160,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4164,"record_offset":21000,"parent_record_offset":20972,"ancestor_record_offset":19160,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4164,"record_offset":20904,"parent_record_offset":20868,"ancestor_record_offset":19160,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4164,"record_offset":20868,"parent_record_offset":20832,"ancestor_record_offset":19160,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/04164.jsonl.gz"}
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
// 首条位置: {"module":4164,"record_offset":32132,"rva":43219888,"va":5411929008}
// 编译实例 RVAs: 0x2937bb0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5128446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411929008,"raw_hex":"e8fe404e00","size":5,"soft_fail":false,"target_va":5417057936,"va":5411929485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":36840,"parent_record_offset":32132,"ancestor_record_offset":32132,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":32716,"parent_record_offset":32132,"ancestor_record_offset":32132,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":32284,"parent_record_offset":32132,"ancestor_record_offset":32132,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":36488,"parent_record_offset":32132,"ancestor_record_offset":32132,"inlinee":9796,"source_line":898,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4164,"record_offset":36456,"parent_record_offset":32132,"ancestor_record_offset":32132,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":32548,"parent_record_offset":32524,"ancestor_record_offset":32132,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":32596,"parent_record_offset":32572,"ancestor_record_offset":32132,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":32572,"parent_record_offset":32548,"ancestor_record_offset":32132,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":32524,"parent_record_offset":32500,"ancestor_record_offset":32132,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":32500,"parent_record_offset":32468,"ancestor_record_offset":32132,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":33020,"parent_record_offset":32996,"ancestor_record_offset":32132,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":33044,"parent_record_offset":33020,"ancestor_record_offset":32132,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":32468,"parent_record_offset":32444,"ancestor_record_offset":32132,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":34260,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":32996,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":33080,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":34512,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":32912,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":36868,"parent_record_offset":36840,"ancestor_record_offset":32132,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34588,"parent_record_offset":34564,"ancestor_record_offset":32132,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":32132,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":33228,"parent_record_offset":33200,"ancestor_record_offset":32132,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34564,"parent_record_offset":34540,"ancestor_record_offset":32132,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34612,"parent_record_offset":34588,"ancestor_record_offset":32132,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":33760,"parent_record_offset":33172,"ancestor_record_offset":32132,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":33792,"parent_record_offset":33760,"ancestor_record_offset":32132,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34740,"parent_record_offset":34540,"ancestor_record_offset":32132,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34788,"parent_record_offset":34764,"ancestor_record_offset":32132,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$89::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":34540,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10339,"source_line":2520,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":32420,"parent_record_offset":32308,"ancestor_record_offset":32132,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":32444,"parent_record_offset":32420,"ancestor_record_offset":32132,"inlinee":9124,"source_line":2408,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":32748,"parent_record_offset":32716,"ancestor_record_offset":32132,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":33172,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10332,"source_line":182,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":34884,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10321,"source_line":195,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4164,"record_offset":33708,"parent_record_offset":33684,"ancestor_record_offset":32132,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":33344,"parent_record_offset":33316,"ancestor_record_offset":32132,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":33316,"parent_record_offset":33288,"ancestor_record_offset":32132,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":33532,"parent_record_offset":33508,"ancestor_record_offset":32132,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":33288,"parent_record_offset":33256,"ancestor_record_offset":32132,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":33256,"parent_record_offset":33228,"ancestor_record_offset":32132,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":36364,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":34636,"parent_record_offset":34612,"ancestor_record_offset":32132,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":36280,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4164,"record_offset":34192,"parent_record_offset":33792,"ancestor_record_offset":32132,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4164,"record_offset":33684,"parent_record_offset":33660,"ancestor_record_offset":32132,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":33660,"parent_record_offset":33636,"ancestor_record_offset":32132,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4164,"record_offset":34216,"parent_record_offset":34192,"ancestor_record_offset":32132,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":32360,"parent_record_offset":32336,"ancestor_record_offset":32132,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":32384,"parent_record_offset":32360,"ancestor_record_offset":32132,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":32308,"parent_record_offset":32284,"ancestor_record_offset":32132,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":32336,"parent_record_offset":32308,"ancestor_record_offset":32132,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4164,"record_offset":34812,"parent_record_offset":34788,"ancestor_record_offset":32132,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":36768,"parent_record_offset":36724,"ancestor_record_offset":32132,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":36576,"parent_record_offset":36524,"ancestor_record_offset":32132,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4164,"record_offset":33436,"parent_record_offset":33372,"ancestor_record_offset":32132,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":33920,"parent_record_offset":33896,"ancestor_record_offset":32132,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":33464,"parent_record_offset":33436,"ancestor_record_offset":32132,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":33508,"parent_record_offset":33316,"ancestor_record_offset":32132,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4164,"record_offset":32664,"parent_record_offset":32640,"ancestor_record_offset":32132,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":32876,"parent_record_offset":32848,"ancestor_record_offset":32132,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":32936,"parent_record_offset":32912,"ancestor_record_offset":32132,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":34456,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":32960,"parent_record_offset":32936,"ancestor_record_offset":32132,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":33588,"parent_record_offset":33556,"ancestor_record_offset":32132,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4164,"record_offset":32640,"parent_record_offset":32468,"ancestor_record_offset":32132,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":32848,"parent_record_offset":32748,"ancestor_record_offset":32132,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":36524,"parent_record_offset":36488,"ancestor_record_offset":32132,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":36724,"parent_record_offset":36488,"ancestor_record_offset":32132,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":34668,"parent_record_offset":34636,"ancestor_record_offset":32132,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":33556,"parent_record_offset":33532,"ancestor_record_offset":32132,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
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
// 首条位置: {"module":4164,"record_offset":272,"rva":43209168,"va":5411918288}
// 编译实例 RVAs: 0x29351d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca440
//     owner=unresolved_namespace; count=2
//     target_aliases=77 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-36089895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8d94fd9fd","size":5,"soft_fail":false,"target_va":5375829056,"va":5411918946,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c714c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30165666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e85eb533fe","size":5,"soft_fail":false,"target_va":5381756096,"va":5411921757,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c8c720
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30054413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8f36735fe","size":5,"soft_fail":false,"target_va":5381867296,"va":5411921704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140deb2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28617742","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8f2534bfe","size":5,"soft_fail":false,"target_va":5383303904,"va":5411921641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0600
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28597087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8a1a44bfe","size":5,"soft_fail":false,"target_va":5383325184,"va":5411922266,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142927b50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-55508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e82c27ffff","size":5,"soft_fail":false,"target_va":5411863376,"va":5411918879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1be90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t5138794","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e86a694e00","size":5,"soft_fail":false,"target_va":5417057936,"va":5411919137,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::iter::QueryIterationCursor$LT$D$C$F$GT$::init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-51518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8c236ffff","size":5,"soft_fail":false,"target_va":5411867600,"va":5411919113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4697851","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8fbae4700","size":5,"soft_fail":false,"target_va":5416621408,"va":5411923552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4698058","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8caaf4700","size":5,"soft_fail":false,"target_va":5416621600,"va":5411923537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t11481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8d92c0000","size":5,"soft_fail":false,"target_va":5411929936,"va":5411918450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t4697716","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e874ae4700","size":5,"soft_fail":false,"target_va":5416621376,"va":5411923655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-30691422","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8a2af2bfe","size":5,"soft_fail":false,"target_va":5381231824,"va":5411923241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31019692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e854ad26fe","size":5,"soft_fail":false,"target_va":5380903536,"va":5411923223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-32757989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e81b270cfe","size":5,"soft_fail":false,"target_va":5379164016,"va":5411922000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-31929744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e870ca18fe","size":5,"soft_fail":false,"target_va":5379993424,"va":5411923163,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27312305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e84f3f5ffe","size":5,"soft_fail":false,"target_va":5384610144,"va":5411922444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27955022","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8b27055fe","size":5,"soft_fail":false,"target_va":5383963792,"va":5411918809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27946515","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8ed9155fe","size":5,"soft_fail":false,"target_va":5383972224,"va":5411918734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27958509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8136355fe","size":5,"soft_fail":false,"target_va":5383964912,"va":5411923416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-27955605","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e86b6e55fe","size":5,"soft_fail":false,"target_va":5383962816,"va":5411918416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::get_or_init_slow
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5113697","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e861074e00","size":5,"soft_fail":false,"target_va":5417037264,"va":5411923562,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::LocalKey$LT$T$GT$::with
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-56417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e89f23ffff","size":5,"soft_fail":false,"target_va":5411867888,"va":5411924300,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4695751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8c7a64700","size":5,"soft_fail":false,"target_va":5416619360,"va":5411923604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2042724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8642b1f00","size":5,"soft_fail":false,"target_va":5413964384,"va":5411921655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2041079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5411918288,"raw_hex":"e8f7241f00","size":5,"soft_fail":false,"target_va":5413964416,"va":5411923332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5411918288,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5411918901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$58::deref_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":14752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9404,"source_line":434,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":3244,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":10317,"source_line":909,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":2904,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":18903,"source_line":704,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":4164,"record_offset":18072,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4164,"record_offset":14724,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":4164,"record_offset":3108,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":15220,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":14808,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":2784,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4164,"record_offset":18164,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":17980,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":17752,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4164,"record_offset":17868,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":6163,"source_line":25,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4164,"record_offset":2724,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8075,"source_line":281,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":4164,"record_offset":2820,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":30004,"source_line":310,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":4164,"record_offset":15104,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":136421,"source_line":87,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":4164,"record_offset":2848,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":27618,"source_line":188,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4164,"record_offset":16296,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":87383,"source_line":712,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":4164,"record_offset":14928,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8716,"source_line":1394,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15528,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15280,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9827,"source_line":624,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15988,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":16662,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":3144,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$38::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15900,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11803,"source_line":1624,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$88::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":16268,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11877,"source_line":2063,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":1916,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":512,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":904,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4164,"record_offset":3080,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":11055,"source_line":354,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":4164,"record_offset":17112,"parent_record_offset":272,"ancestor_record_offset":272,"inlinee":26436,"source_line":29,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":13536,"parent_record_offset":13512,"ancestor_record_offset":272,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":13584,"parent_record_offset":13560,"ancestor_record_offset":272,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":4164,"record_offset":13560,"parent_record_offset":13536,"ancestor_record_offset":272,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":13512,"parent_record_offset":13488,"ancestor_record_offset":272,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":272,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":3596,"parent_record_offset":3572,"ancestor_record_offset":272,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":272,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":4164,"record_offset":13464,"parent_record_offset":13440,"ancestor_record_offset":272,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::contains
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":272,"inlinee":7082,"source_line":661,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":13440,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10334,"source_line":484,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":7660,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10331,"source_line":655,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":7744,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":8568,"source_line":478,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":13692,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10337,"source_line":346,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/archetype.rs
//     evidence={"module":4164,"record_offset":7576,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10330,"source_line":864,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$57::set_changed
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":4164,"record_offset":14776,"parent_record_offset":14752,"ancestor_record_offset":272,"inlinee":9399,"source_line":397,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12528,"parent_record_offset":12504,"ancestor_record_offset":272,"inlinee":9590,"source_line":2964,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$104::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12480,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":108165,"source_line":2520,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::matches_component_set
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":4840,"parent_record_offset":4804,"ancestor_record_offset":272,"inlinee":7078,"source_line":1577,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_archetype
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":7992,"parent_record_offset":7836,"ancestor_record_offset":272,"inlinee":10333,"source_line":1530,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":3784,"parent_record_offset":3656,"ancestor_record_offset":272,"inlinee":10322,"source_line":1545,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12704,"parent_record_offset":12480,"ancestor_record_offset":272,"inlinee":10341,"source_line":1598,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$37::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12752,"parent_record_offset":12728,"ancestor_record_offset":272,"inlinee":9575,"source_line":1605,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::matches_component_set
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":6888,"parent_record_offset":6856,"ancestor_record_offset":272,"inlinee":9009,"source_line":1966,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":7880,"parent_record_offset":7836,"ancestor_record_offset":272,"inlinee":11800,"source_line":1912,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$48::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":3700,"parent_record_offset":3656,"ancestor_record_offset":272,"inlinee":11801,"source_line":1927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12504,"parent_record_offset":12480,"ancestor_record_offset":272,"inlinee":11822,"source_line":1987,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$49::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":272,"inlinee":9577,"source_line":1994,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":9072,"parent_record_offset":7836,"ancestor_record_offset":272,"inlinee":18914,"source_line":2203,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_archetype::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":9132,"parent_record_offset":9104,"ancestor_record_offset":272,"inlinee":18916,"source_line":2209,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":4804,"parent_record_offset":3656,"ancestor_record_offset":272,"inlinee":18910,"source_line":2219,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$55::set_table::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":4876,"parent_record_offset":4840,"ancestor_record_offset":272,"inlinee":18912,"source_line":2224,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":12844,"parent_record_offset":12480,"ancestor_record_offset":272,"inlinee":18918,"source_line":2277,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$56::fetch::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/fetch.rs
//     evidence={"module":4164,"record_offset":13200,"parent_record_offset":13176,"ancestor_record_offset":272,"inlinee":28978,"source_line":2286,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":3040,"parent_record_offset":2928,"ancestor_record_offset":272,"inlinee":10316,"source_line":40,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/iter.rs
//     evidence={"module":4164,"record_offset":3268,"parent_record_offset":3244,"ancestor_record_offset":272,"inlinee":10318,"source_line":2511,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":7836,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":108163,"source_line":182,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/query/world_query.rs
//     evidence={"module":4164,"record_offset":3656,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":108162,"source_line":195,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/blob_array.rs
//     evidence={"module":4164,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":272,"inlinee":10327,"source_line":143,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":4960,"parent_record_offset":4932,"ancestor_record_offset":272,"inlinee":7086,"source_line":47,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":3884,"parent_record_offset":3860,"ancestor_record_offset":272,"inlinee":8615,"source_line":56,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":4932,"parent_record_offset":4904,"ancestor_record_offset":272,"inlinee":7084,"source_line":398,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":3748,"parent_record_offset":3724,"ancestor_record_offset":272,"inlinee":8626,"source_line":405,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/sparse_set.rs
//     evidence={"module":4164,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":272,"inlinee":8628,"source_line":406,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=14
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":3724,"parent_record_offset":3700,"ancestor_record_offset":272,"inlinee":8625,"source_line":489,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":3808,"parent_record_offset":3784,"ancestor_record_offset":272,"inlinee":10323,"source_line":377,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":4904,"parent_record_offset":4876,"ancestor_record_offset":272,"inlinee":18913,"source_line":510,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":3572,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10320,"source_line":646,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":13820,"parent_record_offset":13792,"ancestor_record_offset":272,"inlinee":8632,"source_line":114,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":3488,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":10319,"source_line":752,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_changed_ticks_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4164,"record_offset":7416,"parent_record_offset":7152,"ancestor_record_offset":272,"inlinee":11819,"source_line":313,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/column.rs
//     evidence={"module":4164,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":272,"inlinee":10326,"source_line":297,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/table/mod.rs
//     evidence={"module":4164,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":272,"inlinee":10325,"source_line":382,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::thin_array_ptr::ThinArrayPtr::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/storage/thin_array_ptr.rs
//     evidence={"module":4164,"record_offset":7440,"parent_record_offset":7416,"ancestor_record_offset":272,"inlinee":11820,"source_line":300,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":2980,"parent_record_offset":2956,"ancestor_record_offset":272,"inlinee":11851,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":3004,"parent_record_offset":2980,"ancestor_record_offset":272,"inlinee":11852,"source_line":522,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":2928,"parent_record_offset":2904,"ancestor_record_offset":272,"inlinee":18904,"source_line":604,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/system/query.rs
//     evidence={"module":4164,"record_offset":2956,"parent_record_offset":2928,"ancestor_record_offset":272,"inlinee":11850,"source_line":621,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"bevy_ptr","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.17.3/src/lib.rs
//     evidence={"module":4164,"record_offset":12580,"parent_record_offset":12552,"ancestor_record_offset":272,"inlinee":9579,"source_line":1073,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":4164,"record_offset":12868,"parent_record_offset":12844,"ancestor_record_offset":272,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2244,"parent_record_offset":2216,"ancestor_record_offset":272,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2272,"parent_record_offset":2244,"ancestor_record_offset":272,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2192,"parent_record_offset":2164,"ancestor_record_offset":272,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2216,"parent_record_offset":2192,"ancestor_record_offset":272,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2332,"parent_record_offset":2216,"ancestor_record_offset":272,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":4164,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":272,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::simd::Simd::splat
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\simd.rs
//     evidence={"module":4164,"record_offset":17400,"parent_record_offset":17376,"ancestor_record_offset":272,"inlinee":17835,"source_line":55,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16536,"parent_record_offset":16512,"ancestor_record_offset":272,"inlinee":7191,"source_line":35,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_and_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16996,"parent_record_offset":16296,"ancestor_record_offset":272,"inlinee":27910,"source_line":253,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16772,"parent_record_offset":16568,"ancestor_record_offset":272,"inlinee":32186,"source_line":113,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16424,"parent_record_offset":16400,"ancestor_record_offset":272,"inlinee":7190,"source_line":87,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":17376,"parent_record_offset":17352,"ancestor_record_offset":272,"inlinee":17833,"source_line":934,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16676,"parent_record_offset":16620,"ancestor_record_offset":272,"inlinee":11716,"source_line":1045,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":16744,"parent_record_offset":16568,"ancestor_record_offset":272,"inlinee":32232,"source_line":137,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":4164,"record_offset":17024,"parent_record_offset":16296,"ancestor_record_offset":272,"inlinee":11712,"source_line":319,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":4164,"record_offset":15424,"parent_record_offset":15400,"ancestor_record_offset":272,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":4164,"record_offset":2296,"parent_record_offset":2272,"ancestor_record_offset":272,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":14956,"parent_record_offset":14928,"ancestor_record_offset":272,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":3180,"parent_record_offset":3144,"ancestor_record_offset":272,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$353::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":15928,"parent_record_offset":15900,"ancestor_record_offset":272,"inlinee":8870,"source_line":791,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":4164,"record_offset":15596,"parent_record_offset":15556,"ancestor_record_offset":272,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":4164,"record_offset":2064,"parent_record_offset":2024,"ancestor_record_offset":272,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":12136,"parent_record_offset":12112,"ancestor_record_offset":272,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":272,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":272,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":4164,"record_offset":4032,"parent_record_offset":3860,"ancestor_record_offset":272,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4164,"record_offset":852,"parent_record_offset":824,"ancestor_record_offset":272,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":4164,"record_offset":2680,"parent_record_offset":2024,"ancestor_record_offset":272,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":4164,"record_offset":2396,"parent_record_offset":2164,"ancestor_record_offset":272,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":4164,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":272,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":3456,"parent_record_offset":3396,"ancestor_record_offset":272,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":1324,"parent_record_offset":1300,"ancestor_record_offset":272,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":272,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":4164,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":272,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":4164,"record_offset":1608,"parent_record_offset":1580,"ancestor_record_offset":272,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":272,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":3512,"parent_record_offset":3488,"ancestor_record_offset":272,"inlinee":4165,"source_line":572,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":13636,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":4164,"record_offset":17892,"parent_record_offset":17868,"ancestor_record_offset":272,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":3536,"parent_record_offset":3512,"ancestor_record_offset":272,"inlinee":4167,"source_line":218,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":4164,"record_offset":4112,"parent_record_offset":4080,"ancestor_record_offset":272,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":4164,"record_offset":17916,"parent_record_offset":17892,"ancestor_record_offset":272,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":1252,"parent_record_offset":1228,"ancestor_record_offset":272,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":1276,"parent_record_offset":1252,"ancestor_record_offset":272,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":1636,"parent_record_offset":1608,"ancestor_record_offset":272,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":3396,"parent_record_offset":3268,"ancestor_record_offset":272,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":4164,"record_offset":1228,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4164,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":272,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":4164,"record_offset":940,"parent_record_offset":904,"ancestor_record_offset":272,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4164,"record_offset":1080,"parent_record_offset":1048,"ancestor_record_offset":272,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":4164,"record_offset":1048,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":988,"parent_record_offset":964,"ancestor_record_offset":272,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":1012,"parent_record_offset":988,"ancestor_record_offset":272,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":964,"parent_record_offset":940,"ancestor_record_offset":272,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":4164,"record_offset":1552,"parent_record_offset":1012,"ancestor_record_offset":272,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":616,"parent_record_offset":584,"ancestor_record_offset":272,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":584,"parent_record_offset":556,"ancestor_record_offset":272,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":744,"parent_record_offset":712,"ancestor_record_offset":272,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":800,"parent_record_offset":744,"ancestor_record_offset":272,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":712,"parent_record_offset":680,"ancestor_record_offset":272,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":556,"parent_record_offset":512,"ancestor_record_offset":272,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":4164,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":272,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":1112,"parent_record_offset":1080,"ancestor_record_offset":272,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":1396,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":1368,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":4164,"record_offset":1172,"parent_record_offset":1112,"ancestor_record_offset":272,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":4164,"record_offset":2748,"parent_record_offset":2724,"ancestor_record_offset":272,"inlinee":8076,"source_line":245,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$21::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":4164,"record_offset":18096,"parent_record_offset":18072,"ancestor_record_offset":272,"inlinee":136421,"source_line":87,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/affine3a.rs
//     evidence={"module":4164,"record_offset":17140,"parent_record_offset":17112,"ancestor_record_offset":272,"inlinee":26438,"source_line":253,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/mat3a.rs
//     evidence={"module":4164,"record_offset":17180,"parent_record_offset":17140,"ancestor_record_offset":272,"inlinee":26440,"source_line":278,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::lerp_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4164,"record_offset":16332,"parent_record_offset":16296,"ancestor_record_offset":272,"inlinee":136422,"source_line":696,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::impl$15::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/quat.rs
//     evidence={"module":4164,"record_offset":16360,"parent_record_offset":16332,"ancestor_record_offset":272,"inlinee":136424,"source_line":1020,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4164,"record_offset":17264,"parent_record_offset":17180,"ancestor_record_offset":272,"inlinee":11723,"source_line":108,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$25::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec3a.rs
//     evidence={"module":4164,"record_offset":17352,"parent_record_offset":17140,"ancestor_record_offset":272,"inlinee":17832,"source_line":1499,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4164,"record_offset":16568,"parent_record_offset":16360,"ancestor_record_offset":272,"inlinee":32229,"source_line":626,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$25::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4164,"record_offset":16400,"parent_record_offset":16360,"ancestor_record_offset":272,"inlinee":7188,"source_line":1326,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::impl$35::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/sse2/vec4.rs
//     evidence={"module":4164,"record_offset":16512,"parent_record_offset":16360,"ancestor_record_offset":272,"inlinee":7193,"source_line":1404,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15368,"parent_record_offset":15336,"ancestor_record_offset":272,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15336,"parent_record_offset":15304,"ancestor_record_offset":272,"inlinee":9776,"source_line":552,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15304,"parent_record_offset":15280,"ancestor_record_offset":272,"inlinee":9828,"source_line":571,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":16176,"parent_record_offset":15988,"ancestor_record_offset":272,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":4164,"record_offset":15556,"parent_record_offset":15528,"ancestor_record_offset":272,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4164,"record_offset":16620,"parent_record_offset":16596,"ancestor_record_offset":272,"inlinee":32231,"source_line":52,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/sse2.rs
//     evidence={"module":4164,"record_offset":16596,"parent_record_offset":16568,"ancestor_record_offset":272,"inlinee":32230,"source_line":77,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxU32::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":13852,"parent_record_offset":13820,"ancestor_record_offset":272,"inlinee":8634,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":4164,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":272,"inlinee":4193,"source_line":153,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":1952,"parent_record_offset":1916,"ancestor_record_offset":272,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":18228,"parent_record_offset":18196,"ancestor_record_offset":272,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":18680,"parent_record_offset":18476,"ancestor_record_offset":272,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":4164,"record_offset":2092,"parent_record_offset":2064,"ancestor_record_offset":272,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":2164,"parent_record_offset":2024,"ancestor_record_offset":272,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":4164,"record_offset":2368,"parent_record_offset":2164,"ancestor_record_offset":272,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":4164,"record_offset":15400,"parent_record_offset":15368,"ancestor_record_offset":272,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":4164,"record_offset":2120,"parent_record_offset":2092,"ancestor_record_offset":272,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4164,"record_offset":2024,"parent_record_offset":1988,"ancestor_record_offset":272,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/04164.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":4164,"record_offset":1988,"parent_record_offset":1952,"ancestor_record_offset":272,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/04164.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

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
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "country_core::systems::paths::draw_path::PathRastersResources", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_critters::shared::critters_avoid_collisions
// 信息 ID: 71d435e99867304094137081
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 9168b35bbb2cbaf69de68a30: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x22a367b -> 0x29369b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,country_core::systems::paths::draw_path::PathRastersResources,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::message::message_writer::MessageWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x22b387a -> 0x29369b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::move_critters
// 信息 ID: e04736a75e0c669c81539052
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 529f3daa130b01f5d1602f50: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x22ad57c -> 0x29351d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> @ 0x22b3ef6 -> 0x29351d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::smooth_critter_velocities
// 信息 ID: 3f814a9319b8313c28bfac1c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4afd1c41309402f293e71f20: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x22a49a1 -> 0x2937bb0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),tuple$<>,void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x22b3a60 -> 0x2937bb0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
