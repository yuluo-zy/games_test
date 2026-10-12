// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/shared.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ef7025be5b8f3705eb1281014a15d349d4f4d998aef4b3c7498d1cb7dd373888
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shared.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3609
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::shared::critters_avoid_collisions
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3609,"record_offset":3060,"rva":28527600,"va":5397236720}
// 编译实例 RVAs: 0x1b34bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24087834","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8e67290fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5397237669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c28f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21441527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e809d4b8fe","size":5,"soft_fail":false,"target_va":5375797488,"va":5397239010,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20296267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8b54dcafe","size":5,"soft_fail":false,"target_va":5376943968,"va":5397240230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2daf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-29749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8cb8bffff","size":5,"soft_fail":false,"target_va":5397207792,"va":5397237536,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6355727","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e80ffb6000","size":5,"soft_fail":false,"target_va":5403593344,"va":5397237612,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::event::EventWriter$LT$E$GT$::send
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17703292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e884def1fe","size":5,"soft_fail":false,"target_va":5379537200,"va":5397240487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5737932","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8cc8d5700","size":5,"soft_fail":false,"target_va":5402978688,"va":5397240751,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5738195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8d38e5700","size":5,"soft_fail":false,"target_va":5402978880,"va":5397240680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6205160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8e8ae5e00","size":5,"soft_fail":false,"target_va":5403445888,"va":5397240723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-5119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e801ecffff","size":5,"soft_fail":false,"target_va":5397235456,"va":5397240570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5742816","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8e0a05700","size":5,"soft_fail":false,"target_va":5402980128,"va":5397237307,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e82efbffff","size":5,"soft_fail":false,"target_va":5397235648,"va":5397236877,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5743704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e858a45700","size":5,"soft_fail":false,"target_va":5402980656,"va":5397236947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15307049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8d76e16ff","size":5,"soft_fail":false,"target_va":5381932960,"va":5397240004,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21804992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e84048b3fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5397239963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22730820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8bc27a5fe","size":5,"soft_fail":false,"target_va":5374509552,"va":5397240367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22424694","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e88ad3a9fe","size":5,"soft_fail":false,"target_va":5374815216,"va":5397239905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20880572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e84463c1fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5397237479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20876683","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e87572c1fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5397237238,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20880709","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8bb62c1fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5397236848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h07fb3e82d412bcf2 (.llvm.169583130843556459)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-30335","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e88189ffff","size":5,"soft_fail":false,"target_va":5397206928,"va":5397237258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Segment2d::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19268318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e822fdd9fe","size":5,"soft_fail":false,"target_va":5377970240,"va":5397238553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2901269","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e815452c00","size":5,"soft_fail":false,"target_va":5400139840,"va":5397238566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2899785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e8493f2c00","size":5,"soft_fail":false,"target_va":5400139872,"va":5397240082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-20298927","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397236720,"raw_hex":"e85143cafe","size":5,"soft_fail":false,"target_va":5376939952,"va":5397238874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397236720,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5397237632,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":8032,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":6480,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3609,"record_offset":6280,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3609,"record_offset":12064,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":11820,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":6160,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3609,"record_offset":11852,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":11964,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3609,"record_offset":5920,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3609,"record_offset":6196,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21852,"source_line":300,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3609,"record_offset":9572,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":108535,"source_line":82,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3609,"record_offset":6224,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8176,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":106794,"source_line":500,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8364,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8088,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8452,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":12408,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":10544,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":12176,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":4872,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":3284,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":3860,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3609,"record_offset":8628,"parent_record_offset":3060,"ancestor_record_offset":3060,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":8056,"parent_record_offset":8032,"ancestor_record_offset":3060,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":7788,"parent_record_offset":7752,"ancestor_record_offset":3060,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":6908,"parent_record_offset":6876,"ancestor_record_offset":3060,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":3060,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":7844,"parent_record_offset":7752,"ancestor_record_offset":3060,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":7928,"parent_record_offset":7752,"ancestor_record_offset":3060,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":6876,"parent_record_offset":6768,"ancestor_record_offset":3060,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":6936,"parent_record_offset":6908,"ancestor_record_offset":3060,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":6332,"parent_record_offset":6308,"ancestor_record_offset":3060,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":6356,"parent_record_offset":6332,"ancestor_record_offset":3060,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":6516,"parent_record_offset":6480,"ancestor_record_offset":3060,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3609,"record_offset":6308,"parent_record_offset":6280,"ancestor_record_offset":3060,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":7752,"parent_record_offset":6516,"ancestor_record_offset":3060,"inlinee":61558,"source_line":201,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$6::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":6768,"parent_record_offset":6516,"ancestor_record_offset":3060,"inlinee":61557,"source_line":192,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3609,"record_offset":7368,"parent_record_offset":7344,"ancestor_record_offset":3060,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":7008,"parent_record_offset":6984,"ancestor_record_offset":3060,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":6984,"parent_record_offset":6960,"ancestor_record_offset":3060,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3060,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":7096,"parent_record_offset":7032,"ancestor_record_offset":3060,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":7276,"parent_record_offset":7252,"ancestor_record_offset":3060,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":7344,"parent_record_offset":7176,"ancestor_record_offset":3060,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":6588,"parent_record_offset":6516,"ancestor_record_offset":3060,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":3060,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":3060,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":3060,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":6684,"parent_record_offset":6516,"ancestor_record_offset":3060,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3609,"record_offset":7868,"parent_record_offset":7844,"ancestor_record_offset":3060,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3609,"record_offset":7956,"parent_record_offset":7928,"ancestor_record_offset":3060,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5272,"parent_record_offset":5244,"ancestor_record_offset":3060,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5324,"parent_record_offset":5300,"ancestor_record_offset":3060,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5300,"parent_record_offset":5272,"ancestor_record_offset":3060,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5212,"parent_record_offset":5184,"ancestor_record_offset":3060,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5244,"parent_record_offset":5212,"ancestor_record_offset":3060,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5416,"parent_record_offset":5244,"ancestor_record_offset":3060,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":5528,"parent_record_offset":5504,"ancestor_record_offset":3060,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3609,"record_offset":5988,"parent_record_offset":5948,"ancestor_record_offset":3060,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3609,"record_offset":8940,"parent_record_offset":8908,"ancestor_record_offset":3060,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3609,"record_offset":8996,"parent_record_offset":8908,"ancestor_record_offset":3060,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3609,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":3060,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3609,"record_offset":3496,"parent_record_offset":3400,"ancestor_record_offset":3060,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":8480,"parent_record_offset":8452,"ancestor_record_offset":3060,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":8392,"parent_record_offset":8364,"ancestor_record_offset":3060,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":3060,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":3060,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":5108,"parent_record_offset":5080,"ancestor_record_offset":3060,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":3060,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":4456,"parent_record_offset":3996,"ancestor_record_offset":3060,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":5156,"parent_record_offset":4980,"ancestor_record_offset":3060,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3609,"record_offset":3808,"parent_record_offset":3780,"ancestor_record_offset":3060,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3609,"record_offset":5480,"parent_record_offset":5184,"ancestor_record_offset":3060,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3609,"record_offset":6428,"parent_record_offset":6404,"ancestor_record_offset":3060,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":3060,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":4208,"parent_record_offset":4184,"ancestor_record_offset":3060,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":4416,"parent_record_offset":4384,"ancestor_record_offset":3060,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3609,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":3060,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3609,"record_offset":5880,"parent_record_offset":4944,"ancestor_record_offset":3060,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3609,"record_offset":6072,"parent_record_offset":6044,"ancestor_record_offset":3060,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3609,"record_offset":6100,"parent_record_offset":6072,"ancestor_record_offset":3060,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":3432,"parent_record_offset":3400,"ancestor_record_offset":3060,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":6380,"parent_record_offset":6356,"ancestor_record_offset":3060,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3609,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":3060,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3609,"record_offset":6404,"parent_record_offset":6380,"ancestor_record_offset":3060,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":4184,"parent_record_offset":4160,"ancestor_record_offset":3060,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":4512,"parent_record_offset":4488,"ancestor_record_offset":3060,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":6620,"parent_record_offset":6516,"ancestor_record_offset":3060,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":4160,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3609,"record_offset":3400,"parent_record_offset":3372,"ancestor_record_offset":3060,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3609,"record_offset":3892,"parent_record_offset":3860,"ancestor_record_offset":3060,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3609,"record_offset":4024,"parent_record_offset":3996,"ancestor_record_offset":3060,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3609,"record_offset":3996,"parent_record_offset":3964,"ancestor_record_offset":3060,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3060,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3060,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3060,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":4488,"parent_record_offset":3964,"ancestor_record_offset":3060,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3596,"parent_record_offset":3564,"ancestor_record_offset":3060,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3756,"parent_record_offset":3596,"ancestor_record_offset":3060,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3564,"parent_record_offset":3540,"ancestor_record_offset":3060,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":6016,"parent_record_offset":5988,"ancestor_record_offset":3060,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3316,"parent_record_offset":3284,"ancestor_record_offset":3060,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":3060,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3372,"parent_record_offset":3340,"ancestor_record_offset":3060,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":3340,"parent_record_offset":3316,"ancestor_record_offset":3060,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":4052,"parent_record_offset":4024,"ancestor_record_offset":3060,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":4300,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":4272,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":4104,"parent_record_offset":4052,"ancestor_record_offset":3060,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3609,"record_offset":5948,"parent_record_offset":5920,"ancestor_record_offset":3060,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3609,"record_offset":12088,"parent_record_offset":12064,"ancestor_record_offset":3060,"inlinee":108535,"source_line":82,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3609,"record_offset":9248,"parent_record_offset":9212,"ancestor_record_offset":3060,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":9152,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8884,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8284,"parent_record_offset":8252,"ancestor_record_offset":3060,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":3060,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8228,"parent_record_offset":8200,"ancestor_record_offset":3060,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8908,"parent_record_offset":8884,"ancestor_record_offset":3060,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8200,"parent_record_offset":8176,"ancestor_record_offset":3060,"inlinee":106795,"source_line":483,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8796,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":9512,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":8708,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":9060,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":10572,"parent_record_offset":10544,"ancestor_record_offset":3060,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":12236,"parent_record_offset":12200,"ancestor_record_offset":3060,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":12200,"parent_record_offset":12176,"ancestor_record_offset":3060,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":12432,"parent_record_offset":12408,"ancestor_record_offset":3060,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3609,"record_offset":9184,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3609,"record_offset":7300,"parent_record_offset":7276,"ancestor_record_offset":3060,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":4908,"parent_record_offset":4872,"ancestor_record_offset":3060,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3609,"record_offset":5020,"parent_record_offset":4980,"ancestor_record_offset":3060,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":5184,"parent_record_offset":4980,"ancestor_record_offset":3060,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":5452,"parent_record_offset":5184,"ancestor_record_offset":3060,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3609,"record_offset":8316,"parent_record_offset":8284,"ancestor_record_offset":3060,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3609,"record_offset":5048,"parent_record_offset":5020,"ancestor_record_offset":3060,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3609,"record_offset":5080,"parent_record_offset":5048,"ancestor_record_offset":3060,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3609,"record_offset":4980,"parent_record_offset":4944,"ancestor_record_offset":3060,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3609,"record_offset":4944,"parent_record_offset":4908,"ancestor_record_offset":3060,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3609,"record_offset":9212,"parent_record_offset":8628,"ancestor_record_offset":3060,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::shared::smooth_critter_velocities
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3609,"record_offset":12664,"rva":28532080,"va":5397241200}
// 编译实例 RVAs: 0x1b35d70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6352802","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241200,"raw_hex":"e8a2ef6000","size":5,"soft_fail":false,"target_va":5403594240,"va":5397241433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":14136,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":13012,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3609,"record_offset":12816,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":13884,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":21436,"source_line":742,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3609,"record_offset":13856,"parent_record_offset":12664,"ancestor_record_offset":12664,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":14160,"parent_record_offset":14136,"ancestor_record_offset":12664,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":13432,"parent_record_offset":13404,"ancestor_record_offset":12664,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":13524,"parent_record_offset":13404,"ancestor_record_offset":12664,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":13116,"parent_record_offset":13092,"ancestor_record_offset":12664,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":12864,"parent_record_offset":12840,"ancestor_record_offset":12664,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":12888,"parent_record_offset":12864,"ancestor_record_offset":12664,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":13040,"parent_record_offset":13012,"ancestor_record_offset":12664,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3609,"record_offset":12840,"parent_record_offset":12816,"ancestor_record_offset":12664,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":13404,"parent_record_offset":13040,"ancestor_record_offset":12664,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":13092,"parent_record_offset":13040,"ancestor_record_offset":12664,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3609,"record_offset":13364,"parent_record_offset":13340,"ancestor_record_offset":12664,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":12664,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":12664,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":13340,"parent_record_offset":13116,"ancestor_record_offset":12664,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":13820,"parent_record_offset":13040,"ancestor_record_offset":12664,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":13144,"parent_record_offset":13116,"ancestor_record_offset":12664,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":13456,"parent_record_offset":13432,"ancestor_record_offset":12664,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":13736,"parent_record_offset":13040,"ancestor_record_offset":12664,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3609,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":12664,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":13988,"parent_record_offset":13912,"ancestor_record_offset":12664,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":14016,"parent_record_offset":13912,"ancestor_record_offset":12664,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":13960,"parent_record_offset":13912,"ancestor_record_offset":12664,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":12664,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3609,"record_offset":13288,"parent_record_offset":13264,"ancestor_record_offset":12664,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3609,"record_offset":12960,"parent_record_offset":12936,"ancestor_record_offset":12664,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":13700,"parent_record_offset":13676,"ancestor_record_offset":12664,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":13648,"parent_record_offset":13612,"ancestor_record_offset":12664,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":13760,"parent_record_offset":13736,"ancestor_record_offset":12664,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":13240,"parent_record_offset":13216,"ancestor_record_offset":12664,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":12912,"parent_record_offset":12888,"ancestor_record_offset":12664,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3609,"record_offset":13784,"parent_record_offset":13760,"ancestor_record_offset":12664,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3609,"record_offset":13264,"parent_record_offset":13240,"ancestor_record_offset":12664,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3609,"record_offset":12936,"parent_record_offset":12912,"ancestor_record_offset":12664,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":13676,"parent_record_offset":13612,"ancestor_record_offset":12664,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":13612,"parent_record_offset":13040,"ancestor_record_offset":12664,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":13912,"parent_record_offset":13884,"ancestor_record_offset":12664,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::shared::move_critters
// 记录形式: compiled_procedure; instances=1
// 源码行: 216
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3609,"record_offset":14196,"rva":28532480,"va":5397241600}
// 编译实例 RVAs: 0x1b35f00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14043c290
//     owner=unresolved_namespace; count=2
//     target_aliases=53 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-24092713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8d75f90fe","size":5,"soft_fail":false,"target_va":5373149840,"va":5397242548,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406c28f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21446581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e84bc0b8fe","size":5,"soft_fail":false,"target_va":5375797488,"va":5397244064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cbdcb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-15174845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8437318ff","size":5,"soft_fail":false,"target_va":5382069424,"va":5397244264,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b2daf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-34628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8bc78ffff","size":5,"soft_fail":false,"target_va":5397207792,"va":5397242415,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a80
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6350848","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e800e86000","size":5,"soft_fail":false,"target_va":5403593344,"va":5397242491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shared.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6351553","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8c1ea6000","size":5,"soft_fail":false,"target_va":5403594240,"va":5397242682,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5732263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8a7775700","size":5,"soft_fail":false,"target_va":5402978688,"va":5397246420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t5732526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8ae785700","size":5,"soft_fail":false,"target_va":5402978880,"va":5397246349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6199491","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8c3985e00","size":5,"soft_fail":false,"target_va":5403445888,"va":5397246392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-10788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8dcd5ffff","size":5,"soft_fail":false,"target_va":5397235456,"va":5397246239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5737941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8d58d5700","size":5,"soft_fail":false,"target_va":5402980128,"va":5397242182,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-6121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e817e8ffff","size":5,"soft_fail":false,"target_va":5397235648,"va":5397241764,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t5738817","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e841915700","size":5,"soft_fail":false,"target_va":5402980656,"va":5397241834,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15311705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8a75c16ff","size":5,"soft_fail":false,"target_va":5381932960,"va":5397244660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21809672","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8f835b3fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5397244643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::legacy_spherecast
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22735792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e85014a5fe","size":5,"soft_fail":false,"target_va":5374509552,"va":5397245339,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22429366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e84ac1a9fe","size":5,"soft_fail":false,"target_va":5374815216,"va":5397244577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::quat::Quat::from_rotation_arc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19896085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8eb68d0fe","size":5,"soft_fail":false,"target_va":5377348352,"va":5397244432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20885442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e83e50c1fe","size":5,"soft_fail":false,"target_va":5376356912,"va":5397242349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20881558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e86a5fc1fe","size":5,"soft_fail":false,"target_va":5376360560,"va":5397242113,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20885589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8ab4fc1fe","size":5,"soft_fail":false,"target_va":5376356144,"va":5397241728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h07fb3e82d412bcf2 (.llvm.169583130843556459)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-35210","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e87676ffff","size":5,"soft_fail":false,"target_va":5397206928,"va":5397242133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Segment2d::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19273393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e84fe9d9fe","size":5,"soft_fail":false,"target_va":5377970240,"va":5397243628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::line_segment
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2896194","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e842312c00","size":5,"soft_fail":false,"target_va":5400139840,"va":5397243641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::line_relation::LineInterval::relate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2895124","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8142d2c00","size":5,"soft_fail":false,"target_va":5400139872,"va":5397244743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::lerp
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t-20303976","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397241600,"raw_hex":"e8982fcafe","size":5,"soft_fail":false,"target_va":5376939952,"va":5397243923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*24(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5397241600,"raw_hex":"ff5018","size":3,"soft_fail":false,"va":5397242511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":20584,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":17752,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3609,"record_offset":17424,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3609,"record_offset":23368,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3609,"record_offset":20556,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":20640,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":17304,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3609,"record_offset":23132,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":23244,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":24012,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3609,"record_offset":17064,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":9047,"source_line":236,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::ssbo::impl$6::mapped_data
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/ssbo.rs
//     evidence={"module":3609,"record_offset":17340,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21852,"source_line":300,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::critters::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3609,"record_offset":21888,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":108535,"source_line":82,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::world_raster::WorldRaster::as_float_grid
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3609,"record_offset":17368,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21417,"source_line":126,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3609,"record_offset":22428,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":66063,"source_line":576,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":20764,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":24332,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":24096,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::try_normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22148,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":22211,"source_line":523,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$13::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22060,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21924,"source_line":1092,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$33::neg
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22400,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":14666,"source_line":1299,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":17652,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":16016,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":14428,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":15004,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powf
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3609,"record_offset":17624,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":6118,"source_line":412,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] utils::components::transform::Transform::compute_affine
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/components/transform.rs
//     evidence={"module":3609,"record_offset":23460,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":23062,"source_line":28,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_direct] utils::float_grid::impl$0::bilinear_sample_at_world_xz
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3609,"record_offset":20940,"parent_record_offset":14196,"ancestor_record_offset":14196,"inlinee":21423,"source_line":26,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3609,"record_offset":20608,"parent_record_offset":20584,"ancestor_record_offset":14196,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":18196,"parent_record_offset":18164,"ancestor_record_offset":14196,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":18464,"parent_record_offset":18164,"ancestor_record_offset":14196,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":20096,"parent_record_offset":20052,"ancestor_record_offset":14196,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":19412,"parent_record_offset":19380,"ancestor_record_offset":14196,"inlinee":6327,"source_line":1372,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":18080,"parent_record_offset":18056,"ancestor_record_offset":14196,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":20188,"parent_record_offset":20052,"ancestor_record_offset":14196,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":20300,"parent_record_offset":20276,"ancestor_record_offset":14196,"inlinee":24860,"source_line":1566,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":18164,"parent_record_offset":18056,"ancestor_record_offset":14196,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3609,"record_offset":18224,"parent_record_offset":18196,"ancestor_record_offset":14196,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":17476,"parent_record_offset":17452,"ancestor_record_offset":14196,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":17500,"parent_record_offset":17476,"ancestor_record_offset":14196,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3609,"record_offset":17796,"parent_record_offset":17752,"ancestor_record_offset":14196,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3609,"record_offset":17452,"parent_record_offset":17424,"ancestor_record_offset":14196,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":20052,"parent_record_offset":17796,"ancestor_record_offset":14196,"inlinee":81020,"source_line":201,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$7::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3609,"record_offset":18056,"parent_record_offset":17796,"ancestor_record_offset":14196,"inlinee":81019,"source_line":192,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3609,"record_offset":18656,"parent_record_offset":18632,"ancestor_record_offset":14196,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":18296,"parent_record_offset":18272,"ancestor_record_offset":14196,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":18272,"parent_record_offset":18248,"ancestor_record_offset":14196,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":18128,"parent_record_offset":18104,"ancestor_record_offset":14196,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":18384,"parent_record_offset":18320,"ancestor_record_offset":14196,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3609,"record_offset":18564,"parent_record_offset":18540,"ancestor_record_offset":14196,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":20012,"parent_record_offset":19724,"ancestor_record_offset":14196,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":18632,"parent_record_offset":18464,"ancestor_record_offset":14196,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":17876,"parent_record_offset":17796,"ancestor_record_offset":14196,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":18104,"parent_record_offset":18080,"ancestor_record_offset":14196,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":18248,"parent_record_offset":18224,"ancestor_record_offset":14196,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":20120,"parent_record_offset":20096,"ancestor_record_offset":14196,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3609,"record_offset":17972,"parent_record_offset":17796,"ancestor_record_offset":14196,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3609,"record_offset":20352,"parent_record_offset":20324,"ancestor_record_offset":14196,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3609,"record_offset":20216,"parent_record_offset":20188,"ancestor_record_offset":14196,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16416,"parent_record_offset":16388,"ancestor_record_offset":14196,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16468,"parent_record_offset":16444,"ancestor_record_offset":14196,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16444,"parent_record_offset":16416,"ancestor_record_offset":14196,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16356,"parent_record_offset":16328,"ancestor_record_offset":14196,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16388,"parent_record_offset":16356,"ancestor_record_offset":14196,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16560,"parent_record_offset":16388,"ancestor_record_offset":14196,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":3609,"record_offset":16672,"parent_record_offset":16648,"ancestor_record_offset":14196,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3609,"record_offset":17132,"parent_record_offset":17092,"ancestor_record_offset":14196,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22576,"parent_record_offset":22492,"ancestor_record_offset":14196,"inlinee":15697,"source_line":31,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_and_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22640,"parent_record_offset":22428,"ancestor_record_offset":14196,"inlinee":47100,"source_line":239,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_div_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":23036,"parent_record_offset":22804,"ancestor_record_offset":14196,"inlinee":21870,"source_line":100,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22520,"parent_record_offset":22492,"ancestor_record_offset":14196,"inlinee":14684,"source_line":77,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_set1_ps
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":23720,"parent_record_offset":23696,"ancestor_record_offset":14196,"inlinee":21867,"source_line":905,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22548,"parent_record_offset":22492,"ancestor_record_offset":14196,"inlinee":14683,"source_line":1011,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sqrt_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":23008,"parent_record_offset":22804,"ancestor_record_offset":14196,"inlinee":22244,"source_line":124,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22696,"parent_record_offset":22428,"ancestor_record_offset":14196,"inlinee":14685,"source_line":54,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_xor_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":3609,"record_offset":22668,"parent_record_offset":22428,"ancestor_record_offset":14196,"inlinee":47089,"source_line":296,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3609,"record_offset":21256,"parent_record_offset":21224,"ancestor_record_offset":14196,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3609,"record_offset":21312,"parent_record_offset":21224,"ancestor_record_offset":14196,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":3609,"record_offset":16492,"parent_record_offset":16468,"ancestor_record_offset":14196,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":3609,"record_offset":14640,"parent_record_offset":14544,"ancestor_record_offset":14196,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":17688,"parent_record_offset":17652,"ancestor_record_offset":14196,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":20792,"parent_record_offset":20764,"ancestor_record_offset":14196,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":21052,"parent_record_offset":21024,"ancestor_record_offset":14196,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":22088,"parent_record_offset":22060,"ancestor_record_offset":14196,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3609,"record_offset":21852,"parent_record_offset":21828,"ancestor_record_offset":14196,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":16252,"parent_record_offset":16224,"ancestor_record_offset":14196,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":18408,"parent_record_offset":18384,"ancestor_record_offset":14196,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":15600,"parent_record_offset":15140,"ancestor_record_offset":14196,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3609,"record_offset":16300,"parent_record_offset":16124,"ancestor_record_offset":14196,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3609,"record_offset":14952,"parent_record_offset":14924,"ancestor_record_offset":14196,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3609,"record_offset":16624,"parent_record_offset":16328,"ancestor_record_offset":14196,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3609,"record_offset":17572,"parent_record_offset":17548,"ancestor_record_offset":14196,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":23332,"parent_record_offset":23308,"ancestor_record_offset":14196,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":15376,"parent_record_offset":15352,"ancestor_record_offset":14196,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":15352,"parent_record_offset":15328,"ancestor_record_offset":14196,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3609,"record_offset":15560,"parent_record_offset":15528,"ancestor_record_offset":14196,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3609,"record_offset":16516,"parent_record_offset":16492,"ancestor_record_offset":14196,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3609,"record_offset":17024,"parent_record_offset":16088,"ancestor_record_offset":14196,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3609,"record_offset":17216,"parent_record_offset":17188,"ancestor_record_offset":14196,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3609,"record_offset":17244,"parent_record_offset":17216,"ancestor_record_offset":14196,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":14576,"parent_record_offset":14544,"ancestor_record_offset":14196,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3609,"record_offset":17524,"parent_record_offset":17500,"ancestor_record_offset":14196,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3609,"record_offset":14604,"parent_record_offset":14576,"ancestor_record_offset":14196,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3609,"record_offset":19880,"parent_record_offset":19848,"ancestor_record_offset":14196,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3609,"record_offset":17548,"parent_record_offset":17524,"ancestor_record_offset":14196,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":23308,"parent_record_offset":23244,"ancestor_record_offset":14196,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":15328,"parent_record_offset":15304,"ancestor_record_offset":14196,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":15656,"parent_record_offset":15632,"ancestor_record_offset":14196,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":17908,"parent_record_offset":17796,"ancestor_record_offset":14196,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3609,"record_offset":15304,"parent_record_offset":15196,"ancestor_record_offset":14196,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3609,"record_offset":14544,"parent_record_offset":14516,"ancestor_record_offset":14196,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":3609,"record_offset":15036,"parent_record_offset":15004,"ancestor_record_offset":14196,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3609,"record_offset":15168,"parent_record_offset":15140,"ancestor_record_offset":14196,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":3609,"record_offset":15140,"parent_record_offset":15108,"ancestor_record_offset":14196,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":15084,"parent_record_offset":15060,"ancestor_record_offset":14196,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":15108,"parent_record_offset":15084,"ancestor_record_offset":14196,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":15060,"parent_record_offset":15036,"ancestor_record_offset":14196,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":3609,"record_offset":15632,"parent_record_offset":15108,"ancestor_record_offset":14196,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14740,"parent_record_offset":14708,"ancestor_record_offset":14196,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14900,"parent_record_offset":14740,"ancestor_record_offset":14196,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":14196,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":17160,"parent_record_offset":17132,"ancestor_record_offset":14196,"inlinee":4531,"source_line":29,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14460,"parent_record_offset":14428,"ancestor_record_offset":14196,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14924,"parent_record_offset":14900,"ancestor_record_offset":14196,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14516,"parent_record_offset":14484,"ancestor_record_offset":14196,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":3609,"record_offset":14484,"parent_record_offset":14460,"ancestor_record_offset":14196,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":15196,"parent_record_offset":15168,"ancestor_record_offset":14196,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":15444,"parent_record_offset":15196,"ancestor_record_offset":14196,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":15416,"parent_record_offset":15196,"ancestor_record_offset":14196,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":3609,"record_offset":15248,"parent_record_offset":15196,"ancestor_record_offset":14196,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::ssbo_library::AssetSsboLibrary::get_dyn
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/ssbo_library.rs
//     evidence={"module":3609,"record_offset":17092,"parent_record_offset":17064,"ancestor_record_offset":14196,"inlinee":9022,"source_line":200,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::critters::impl$18::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/critters.rs
//     evidence={"module":3609,"record_offset":23392,"parent_record_offset":23368,"ancestor_record_offset":14196,"inlinee":108535,"source_line":82,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::world_raster::impl$3::load_index_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/world_raster.rs
//     evidence={"module":3609,"record_offset":21564,"parent_record_offset":21528,"ancestor_record_offset":14196,"inlinee":21432,"source_line":524,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::affine3a::Affine3A::from_scale_rotation_translation
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/affine3a.rs
//     evidence={"module":3609,"record_offset":23488,"parent_record_offset":23460,"ancestor_record_offset":14196,"inlinee":23063,"source_line":231,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::mat3a::Mat3A::from_quat
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/mat3a.rs
//     evidence={"module":3609,"record_offset":23528,"parent_record_offset":23488,"ancestor_record_offset":14196,"inlinee":23064,"source_line":168,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/quat.rs
//     evidence={"module":3609,"record_offset":22780,"parent_record_offset":22428,"ancestor_record_offset":14196,"inlinee":22240,"source_line":485,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3609,"record_offset":23608,"parent_record_offset":23528,"ancestor_record_offset":14196,"inlinee":14680,"source_line":90,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$10::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec3a.rs
//     evidence={"module":3609,"record_offset":23696,"parent_record_offset":23488,"ancestor_record_offset":14196,"inlinee":23066,"source_line":1054,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/sse2/vec4.rs
//     evidence={"module":3609,"record_offset":22804,"parent_record_offset":22780,"ancestor_record_offset":14196,"inlinee":22241,"source_line":522,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::as_uvec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21468,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":21428,"source_line":875,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::clamp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21200,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":21426,"source_line":201,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21224,"parent_record_offset":21200,"ancestor_record_offset":14196,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21112,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21828,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$2::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21024,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":21425,"source_line":904,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3609,"record_offset":21376,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22236,"parent_record_offset":22204,"ancestor_record_offset":14196,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22204,"parent_record_offset":22180,"ancestor_record_offset":14196,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22180,"parent_record_offset":22148,"ancestor_record_offset":14196,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":24360,"parent_record_offset":24332,"ancestor_record_offset":14196,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3609,"record_offset":22308,"parent_record_offset":22148,"ancestor_record_offset":14196,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3609,"record_offset":22492,"parent_record_offset":22464,"ancestor_record_offset":14196,"inlinee":22243,"source_line":52,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot4_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/sse2.rs
//     evidence={"module":3609,"record_offset":22464,"parent_record_offset":22428,"ancestor_record_offset":14196,"inlinee":22242,"source_line":77,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] glam::u32::uvec2::UVec2::as_vec2
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/u32/uvec2.rs
//     evidence={"module":3609,"record_offset":21500,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":17196,"source_line":302,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3609,"record_offset":18588,"parent_record_offset":18564,"ancestor_record_offset":14196,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":16052,"parent_record_offset":16016,"ancestor_record_offset":14196,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3609,"record_offset":16164,"parent_record_offset":16124,"ancestor_record_offset":14196,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":16328,"parent_record_offset":16124,"ancestor_record_offset":14196,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":3609,"record_offset":16596,"parent_record_offset":16328,"ancestor_record_offset":14196,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3609,"record_offset":22268,"parent_record_offset":22236,"ancestor_record_offset":14196,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":3609,"record_offset":16192,"parent_record_offset":16164,"ancestor_record_offset":14196,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":3609,"record_offset":16224,"parent_record_offset":16192,"ancestor_record_offset":14196,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3609,"record_offset":16124,"parent_record_offset":16088,"ancestor_record_offset":14196,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":3609,"record_offset":16088,"parent_record_offset":16052,"ancestor_record_offset":14196,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/03609.jsonl.gz"}
//   [pdb_inline_transitive] utils::float_grid::FloatGrid::load_unchecked
//     owner=private_source_ownership_candidate; count=12
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/float_grid.rs
//     evidence={"module":3609,"record_offset":21528,"parent_record_offset":20940,"ancestor_record_offset":14196,"inlinee":21430,"source_line":8,"archive":"_pdb/A/modules/03609.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>", "bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>", "bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>
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
// 信息 ID: ee514015b3c42d17ff04afea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ab2a9ccff78daedac971e597: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> > @ 0x1583d9f -> 0x1b34bf0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::CritterGoal>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::event::EventWriter<system_critters::spawn_ducks::DespawnDuckCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x15958db -> 0x1b34bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::move_critters
// 信息 ID: ed8c79dff2b605dc6811a699
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 529f3daa130b01f5d1602f50: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> > @ 0x15843f6 -> 0x1b35f00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::WaterRaster>,bevy_ecs::change_detection::Res<country_core::resources::world_raster_defs::GardenRaster>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>)> @ 0x159529e -> 0x1b35f00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::shared::smooth_critter_velocities
// 信息 ID: bc0077ca54eeb0659bf2a630
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 4afd1c41309402f293e71f20: void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1583156 -> 0x1b35d70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::CritterVelocityTarget>,ref_mut$<country_core::resources::critters::Velocity> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x1594ddb -> 0x1b35d70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
