// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: d483e87bb4b9026f00f7a3784da4000cf23a8f5836c0e329f7df952a4baa8c72
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: handle_splitting_roof_decorators.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1518
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: compiled_procedure; instances=1
// 源码行: 24
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":1704,"rva":19376016,"va":5388085136}
// 编译实例 RVAs: 0x127a790
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13005317","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8fb8d39ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388086800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13005493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e84b8d39ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388086992,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10052761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8679b66ff","size":5,"soft_fail":false,"target_va":5378033488,"va":5388086244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23453032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e868dd6501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388087955,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-5246556","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8a4f1afff","size":5,"soft_fail":false,"target_va":5382840208,"va":5388086759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23454119","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8a7e16501","size":5,"soft_fail":false,"target_va":5411542064,"va":5388087940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10053097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8179a66ff","size":5,"soft_fail":false,"target_va":5378033360,"va":5388086452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10778820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e83c875bff","size":5,"soft_fail":false,"target_va":5377307568,"va":5388086383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8775139","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e81d1a7aff","size":5,"soft_fail":false,"target_va":5379310144,"va":5388085278,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8291710","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8827a81ff","size":5,"soft_fail":false,"target_va":5379793584,"va":5388085289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierFactory::new
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10343865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8472a62ff","size":5,"soft_fail":false,"target_va":5377743024,"va":5388086884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7659285","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8eb208bff","size":5,"soft_fail":false,"target_va":5380427664,"va":5388086944,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-505009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e84f4bf8ff","size":5,"soft_fail":false,"target_va":5387581248,"va":5388086252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-505372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8e449f8ff","size":5,"soft_fail":false,"target_va":5387580992,"va":5388086359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6152655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e8311ea2ff","size":5,"soft_fail":false,"target_va":5381933648,"va":5388086298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6160228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388085136,"raw_hex":"e89c00a2ff","size":5,"soft_fail":false,"target_va":5381926064,"va":5388086287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1518,"record_offset":1912,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1518,"record_offset":4932,"parent_record_offset":1704,"ancestor_record_offset":1704,"inlinee":71521,"source_line":63,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":1704,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":1704,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":1704,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":1704,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7248,"parent_record_offset":7216,"ancestor_record_offset":1704,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7616,"parent_record_offset":7560,"ancestor_record_offset":1704,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1518,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":1704,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1518,"record_offset":7040,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1518,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":1704,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":2244,"parent_record_offset":2220,"ancestor_record_offset":1704,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":1704,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":1704,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":1704,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":2268,"parent_record_offset":2244,"ancestor_record_offset":1704,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":1704,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1518,"record_offset":7152,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1704,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":2196,"parent_record_offset":2096,"ancestor_record_offset":1704,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":1704,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1518,"record_offset":2396,"parent_record_offset":2064,"ancestor_record_offset":1704,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1518,"record_offset":2064,"parent_record_offset":2020,"ancestor_record_offset":1704,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1518,"record_offset":4808,"parent_record_offset":4784,"ancestor_record_offset":1704,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1518,"record_offset":2868,"parent_record_offset":2020,"ancestor_record_offset":1704,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1518,"record_offset":2892,"parent_record_offset":2868,"ancestor_record_offset":1704,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1518,"record_offset":4784,"parent_record_offset":3480,"ancestor_record_offset":1704,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1518,"record_offset":4832,"parent_record_offset":4808,"ancestor_record_offset":1704,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1518,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":1704,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1518,"record_offset":2992,"parent_record_offset":2968,"ancestor_record_offset":1704,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1518,"record_offset":2968,"parent_record_offset":2944,"ancestor_record_offset":1704,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1518,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":1704,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1518,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":1704,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1518,"record_offset":2916,"parent_record_offset":2892,"ancestor_record_offset":1704,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1518,"record_offset":2608,"parent_record_offset":2020,"ancestor_record_offset":1704,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1518,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":1704,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1518,"record_offset":3264,"parent_record_offset":3240,"ancestor_record_offset":1704,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1518,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1704,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1518,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1704,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1518,"record_offset":2020,"parent_record_offset":1912,"ancestor_record_offset":1704,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1518,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1704,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1518,"record_offset":4856,"parent_record_offset":4832,"ancestor_record_offset":1704,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1518,"record_offset":2456,"parent_record_offset":2428,"ancestor_record_offset":1704,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1518,"record_offset":2364,"parent_record_offset":2340,"ancestor_record_offset":1704,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1518,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":1704,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":1704,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5612,"parent_record_offset":5192,"ancestor_record_offset":1704,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5520,"parent_record_offset":5464,"ancestor_record_offset":1704,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":1704,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5320,"parent_record_offset":5236,"ancestor_record_offset":1704,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5384,"parent_record_offset":5192,"ancestor_record_offset":1704,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1518,"record_offset":7124,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1518,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":1704,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1518,"record_offset":6088,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1518,"record_offset":2340,"parent_record_offset":2064,"ancestor_record_offset":1704,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":1704,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":1704,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":1704,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1518,"record_offset":3084,"parent_record_offset":3020,"ancestor_record_offset":1704,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":1704,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":3148,"parent_record_offset":2968,"ancestor_record_offset":1704,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":1704,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":6208,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":7464,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1518,"record_offset":2560,"parent_record_offset":2528,"ancestor_record_offset":1704,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":7324,"parent_record_offset":7296,"ancestor_record_offset":1704,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1518,"record_offset":2096,"parent_record_offset":2064,"ancestor_record_offset":1704,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1518,"record_offset":2164,"parent_record_offset":2136,"ancestor_record_offset":1704,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1518,"record_offset":2136,"parent_record_offset":2096,"ancestor_record_offset":1704,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1518,"record_offset":2428,"parent_record_offset":2020,"ancestor_record_offset":1704,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1518,"record_offset":2504,"parent_record_offset":2480,"ancestor_record_offset":1704,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1518,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":1704,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1518,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":1704,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":5168,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":1704,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":5640,"parent_record_offset":5192,"ancestor_record_offset":1704,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":1704,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":1704,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":7956,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":5968,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6000,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6944,"parent_record_offset":6240,"ancestor_record_offset":1704,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":5824,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":1704,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":5236,"parent_record_offset":5192,"ancestor_record_offset":1704,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1518,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":1704,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1518,"record_offset":8084,"parent_record_offset":8048,"ancestor_record_offset":1704,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1518,"record_offset":6172,"parent_record_offset":4932,"ancestor_record_offset":1704,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1518,"record_offset":6240,"parent_record_offset":6208,"ancestor_record_offset":1704,"inlinee":71523,"source_line":127,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11761
// 首条位置: {"module":1518,"inlinee":71521}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1518,"record_offset":7040,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":7051,"source_line":644,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1518,"record_offset":7152,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4783,"source_line":386,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hint.rs
//     evidence={"module":1518,"record_offset":7124,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":7064,"source_line":599,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1518,"record_offset":6088,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":6208,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":7464,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":5168,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":7956,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":13631,"source_line":477,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":5968,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6000,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":7376,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":5824,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1518,"record_offset":6172,"parent_record_offset":4932,"ancestor_record_offset":4932,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7272,"parent_record_offset":7248,"ancestor_record_offset":4932,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7296,"parent_record_offset":7272,"ancestor_record_offset":4932,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7640,"parent_record_offset":7616,"ancestor_record_offset":4932,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7216,"parent_record_offset":7184,"ancestor_record_offset":4932,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7248,"parent_record_offset":7216,"ancestor_record_offset":4932,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":7616,"parent_record_offset":7560,"ancestor_record_offset":4932,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1518,"record_offset":7184,"parent_record_offset":7152,"ancestor_record_offset":4932,"inlinee":4280,"source_line":273,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\fmt.rs
//     evidence={"module":1518,"record_offset":7088,"parent_record_offset":7064,"ancestor_record_offset":4932,"inlinee":7053,"source_line":654,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7588,"parent_record_offset":7560,"ancestor_record_offset":4932,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7560,"parent_record_offset":7536,"ancestor_record_offset":4932,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":7536,"parent_record_offset":7512,"ancestor_record_offset":4932,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1518,"record_offset":6136,"parent_record_offset":6112,"ancestor_record_offset":4932,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":4932,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5612,"parent_record_offset":5192,"ancestor_record_offset":4932,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5520,"parent_record_offset":5464,"ancestor_record_offset":4932,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5288,"parent_record_offset":5264,"ancestor_record_offset":4932,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5320,"parent_record_offset":5236,"ancestor_record_offset":4932,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":5384,"parent_record_offset":5192,"ancestor_record_offset":4932,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1518,"record_offset":6112,"parent_record_offset":6088,"ancestor_record_offset":4932,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":4932,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6028,"parent_record_offset":6000,"ancestor_record_offset":4932,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":5848,"parent_record_offset":5824,"ancestor_record_offset":4932,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":7064,"parent_record_offset":7040,"ancestor_record_offset":4932,"inlinee":6345,"source_line":1218,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":7488,"parent_record_offset":7464,"ancestor_record_offset":4932,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":7324,"parent_record_offset":7296,"ancestor_record_offset":4932,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":6320,"parent_record_offset":6240,"ancestor_record_offset":4932,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":5192,"parent_record_offset":5168,"ancestor_record_offset":4932,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":5640,"parent_record_offset":5192,"ancestor_record_offset":4932,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":4932,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":4932,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":8048,"parent_record_offset":7980,"ancestor_record_offset":4932,"inlinee":4895,"source_line":203,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":4932,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6944,"parent_record_offset":6240,"ancestor_record_offset":4932,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":4932,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":5264,"parent_record_offset":5236,"ancestor_record_offset":4932,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":5236,"parent_record_offset":5192,"ancestor_record_offset":4932,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1518,"record_offset":8084,"parent_record_offset":8048,"ancestor_record_offset":4932,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1518,"record_offset":6240,"parent_record_offset":6208,"ancestor_record_offset":4932,"inlinee":71523,"source_line":127,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11763
// 首条位置: {"module":1518,"inlinee":71523}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":6320,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6264,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1518,"record_offset":6944,"parent_record_offset":6240,"ancestor_record_offset":6240,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":6240,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":6668,"parent_record_offset":6428,"ancestor_record_offset":6240,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":6576,"parent_record_offset":6520,"ancestor_record_offset":6240,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":6468,"parent_record_offset":6428,"ancestor_record_offset":6240,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":6756,"parent_record_offset":6696,"ancestor_record_offset":6240,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1518,"record_offset":6816,"parent_record_offset":6696,"ancestor_record_offset":6240,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6972,"parent_record_offset":6944,"ancestor_record_offset":6240,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":6288,"parent_record_offset":6264,"ancestor_record_offset":6240,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1518,"record_offset":6428,"parent_record_offset":6320,"ancestor_record_offset":6240,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6696,"parent_record_offset":6428,"ancestor_record_offset":6240,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6392,"parent_record_offset":6368,"ancestor_record_offset":6240,"inlinee":13677,"source_line":89,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::impl$101::from
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1518,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":6240,"inlinee":13676,"source_line":1868,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":6520,"parent_record_offset":6496,"ancestor_record_offset":6240,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1518,"record_offset":6496,"parent_record_offset":6428,"ancestor_record_offset":6240,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 138
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":9820,"rva":19379568,"va":5388088688}
// 编译实例 RVAs: 0x127b570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13007266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e85e8639ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388088749,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13007613","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e8038539ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388089112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23451946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e82ad96501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388089041,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23451895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e8f7d86501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388089063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10344873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e8572662ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388088884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10344773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e8bb2662ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388088976,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e88c2362ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388088863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345078","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e88a2562ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388088945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10344942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388088688,"raw_hex":"e8122662ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388088905,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10044,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10728,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":10192,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":10812,"parent_record_offset":9820,"ancestor_record_offset":9820,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":9820,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":9820,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10884,"parent_record_offset":10860,"ancestor_record_offset":9820,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10076,"parent_record_offset":10044,"ancestor_record_offset":9820,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":10860,"parent_record_offset":10836,"ancestor_record_offset":9820,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1518,"record_offset":10836,"parent_record_offset":10812,"ancestor_record_offset":9820,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":10452,"parent_record_offset":10424,"ancestor_record_offset":9820,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":10512,"parent_record_offset":10480,"ancestor_record_offset":9820,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":10480,"parent_record_offset":10452,"ancestor_record_offset":9820,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10388,"parent_record_offset":10360,"ancestor_record_offset":9820,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10752,"parent_record_offset":10728,"ancestor_record_offset":9820,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10360,"parent_record_offset":10332,"ancestor_record_offset":9820,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10304,"parent_record_offset":10276,"ancestor_record_offset":9820,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10332,"parent_record_offset":10304,"ancestor_record_offset":9820,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":10276,"parent_record_offset":10248,"ancestor_record_offset":9820,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1518,"record_offset":10248,"parent_record_offset":10220,"ancestor_record_offset":9820,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1518,"record_offset":10220,"parent_record_offset":10192,"ancestor_record_offset":9820,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":9820,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":10424,"parent_record_offset":10388,"ancestor_record_offset":9820,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1518,"record_offset":10664,"parent_record_offset":10640,"ancestor_record_offset":9820,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1518,"record_offset":10640,"parent_record_offset":10388,"ancestor_record_offset":9820,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9820,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 165
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":10928,"rva":19380016,"va":5388089136}
// 编译实例 RVAs: 0x127b730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13007714","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e89e8439ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388089197,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13008061","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8438339ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388089560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23451498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e86ad76501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388089489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23451447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e837d76501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388089511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8972462ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388089332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345221","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8fb2462ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388089424,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10346036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8cc2162ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388089311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8ca2362ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388089393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089136,"raw_hex":"e8522462ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388089353,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11152,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11836,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":11300,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":11920,"parent_record_offset":10928,"ancestor_record_offset":10928,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":10928,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11232,"parent_record_offset":11208,"ancestor_record_offset":10928,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11992,"parent_record_offset":11968,"ancestor_record_offset":10928,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11184,"parent_record_offset":11152,"ancestor_record_offset":10928,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":11968,"parent_record_offset":11944,"ancestor_record_offset":10928,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1518,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":10928,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":11560,"parent_record_offset":11532,"ancestor_record_offset":10928,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":11620,"parent_record_offset":11588,"ancestor_record_offset":10928,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":11588,"parent_record_offset":11560,"ancestor_record_offset":10928,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11496,"parent_record_offset":11468,"ancestor_record_offset":10928,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11860,"parent_record_offset":11836,"ancestor_record_offset":10928,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11468,"parent_record_offset":11440,"ancestor_record_offset":10928,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11412,"parent_record_offset":11384,"ancestor_record_offset":10928,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11440,"parent_record_offset":11412,"ancestor_record_offset":10928,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":11384,"parent_record_offset":11356,"ancestor_record_offset":10928,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1518,"record_offset":11356,"parent_record_offset":11328,"ancestor_record_offset":10928,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1518,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":10928,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":11884,"parent_record_offset":11860,"ancestor_record_offset":10928,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":11532,"parent_record_offset":11496,"ancestor_record_offset":10928,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1518,"record_offset":11772,"parent_record_offset":11748,"ancestor_record_offset":10928,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1518,"record_offset":11748,"parent_record_offset":11496,"ancestor_record_offset":10928,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":11256,"parent_record_offset":11232,"ancestor_record_offset":10928,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":12036,"rva":19380464,"va":5388089584}
// 编译实例 RVAs: 0x127b8f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13008162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e8de8239ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5388089645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13008509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e8838139ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388090008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23451050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e8aad56501","size":5,"soft_fail":false,"target_va":5411540992,"va":5388089937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23450999","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e877d56501","size":5,"soft_fail":false,"target_va":5411540963,"va":5388089959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_duration
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345769","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e8d72262ff","size":5,"soft_fail":false,"target_va":5377744016,"va":5388089780,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::build
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e83b2362ff","size":5,"soft_fail":false,"target_va":5377744208,"va":5388089872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10346484","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e80c2062ff","size":5,"soft_fail":false,"target_va":5377743280,"va":5388089759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e80a2262ff","size":5,"soft_fail":false,"target_va":5377743872,"va":5388089841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::signifiers::SignifierEntityBuilder::scale
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10345838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388089584,"raw_hex":"e8922262ff","size":5,"soft_fail":false,"target_va":5377743968,"va":5388089801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":12260,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":4281,"source_line":347,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12944,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":1518,"record_offset":12408,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":4585,"source_line":760,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":13028,"parent_record_offset":12036,"ancestor_record_offset":12036,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":12036,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":12036,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":13100,"parent_record_offset":13076,"ancestor_record_offset":12036,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":12292,"parent_record_offset":12260,"ancestor_record_offset":12036,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":13076,"parent_record_offset":13052,"ancestor_record_offset":12036,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":1518,"record_offset":13052,"parent_record_offset":13028,"ancestor_record_offset":12036,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":12668,"parent_record_offset":12640,"ancestor_record_offset":12036,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":12728,"parent_record_offset":12696,"ancestor_record_offset":12036,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":12696,"parent_record_offset":12668,"ancestor_record_offset":12036,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12604,"parent_record_offset":12576,"ancestor_record_offset":12036,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12968,"parent_record_offset":12944,"ancestor_record_offset":12036,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12576,"parent_record_offset":12548,"ancestor_record_offset":12036,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12520,"parent_record_offset":12492,"ancestor_record_offset":12036,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12548,"parent_record_offset":12520,"ancestor_record_offset":12036,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1518,"record_offset":12492,"parent_record_offset":12464,"ancestor_record_offset":12036,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1518,"record_offset":12464,"parent_record_offset":12436,"ancestor_record_offset":12036,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$47::from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\string.rs
//     evidence={"module":1518,"record_offset":12436,"parent_record_offset":12408,"ancestor_record_offset":12036,"inlinee":5924,"source_line":2956,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12036,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":12640,"parent_record_offset":12604,"ancestor_record_offset":12036,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1518,"record_offset":12880,"parent_record_offset":12856,"ancestor_record_offset":12036,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1518,"record_offset":12856,"parent_record_offset":12604,"ancestor_record_offset":12036,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":12036,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 记录形式: compiled_procedure; instances=1
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":13144,"rva":19380912,"va":5388090032}
// 编译实例 RVAs: 0x127bab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=211 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19138579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8edf7dbfe","size":5,"soft_fail":false,"target_va":5368953712,"va":5388092286,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13011160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8287739ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388092659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=1
//     target_aliases=14 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10783258","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8e6755bff","size":5,"soft_fail":false,"target_va":5377307216,"va":5388090469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7692270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e812a08aff","size":5,"soft_fail":false,"target_va":5380398224,"va":5388090489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412592f0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-144389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8fbcbfdff","size":5,"soft_fail":false,"target_va":5387948784,"va":5388093168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-126227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8ed12feff","size":5,"soft_fail":false,"target_va":5387964304,"va":5388090526,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1428f82f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23577774","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8aec46701","size":5,"soft_fail":false,"target_va":5411668720,"va":5388090941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-7456603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8a5388eff","size":5,"soft_fail":false,"target_va":5380634736,"va":5388091334,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23448962","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e882cd6501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388092809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23449303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8d7ce6501","size":5,"soft_fail":false,"target_va":5411542064,"va":5388092756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t23448991","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e89fcd6501","size":5,"soft_fail":false,"target_va":5411541904,"va":5388092908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10782754","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8de775bff","size":5,"soft_fail":false,"target_va":5377307568,"va":5388090317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::TransientDecoratorIdGen::generate
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10782552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8a8785bff","size":5,"soft_fail":false,"target_va":5377309152,"va":5388091699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8778596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e89c0c7aff","size":5,"soft_fail":false,"target_va":5379311616,"va":5388090207,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7688849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e86fad8aff","size":5,"soft_fail":false,"target_va":5380401520,"va":5388090364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::apply_decorator_op::apply_decorator_op
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-21174","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e84aadffff","size":5,"soft_fail":false,"target_va":5388070448,"va":5388091617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388090032,"raw_hex":"e8eb070000","size":5,"soft_fail":false,"target_va":5388093424,"va":5388091392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":14884,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":15016,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1518,"record_offset":16092,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1518,"record_offset":13704,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1518,"record_offset":14568,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":15572,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":15444,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":13404,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":15656,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1518,"record_offset":13504,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":25099,"source_line":291,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1518,"record_offset":14456,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$28::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1518,"record_offset":14512,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":19787,"source_line":1232,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1518,"record_offset":14212,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":13440,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":10136,"source_line":779,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14268,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":11441,"source_line":1114,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14912,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14636,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":35795,"source_line":2290,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::decorator_visual::cottage_dormers::cottage_dormer::cottage_dormer_dims
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/decorator_visual/cottage_dormers/cottage_dormer.rs
//     evidence={"module":1518,"record_offset":13468,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":25174,"source_line":32,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1518,"record_offset":13676,"parent_record_offset":13144,"ancestor_record_offset":13144,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":16028,"parent_record_offset":16004,"ancestor_record_offset":13144,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1518,"record_offset":16004,"parent_record_offset":15920,"ancestor_record_offset":13144,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15072,"parent_record_offset":15048,"ancestor_record_offset":13144,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15948,"parent_record_offset":15920,"ancestor_record_offset":13144,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15920,"parent_record_offset":15896,"ancestor_record_offset":13144,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15120,"parent_record_offset":15096,"ancestor_record_offset":13144,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15096,"parent_record_offset":15072,"ancestor_record_offset":13144,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1518,"record_offset":15896,"parent_record_offset":15872,"ancestor_record_offset":13144,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1518,"record_offset":15048,"parent_record_offset":15016,"ancestor_record_offset":13144,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1518,"record_offset":13780,"parent_record_offset":13752,"ancestor_record_offset":13144,"inlinee":4557,"source_line":235,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\array\iter.rs
//     evidence={"module":1518,"record_offset":13896,"parent_record_offset":13804,"ancestor_record_offset":13144,"inlinee":4564,"source_line":241,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1518,"record_offset":13632,"parent_record_offset":13608,"ancestor_record_offset":13144,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1518,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":13144,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1518,"record_offset":14080,"parent_record_offset":13752,"ancestor_record_offset":13144,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1518,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":13144,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1518,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":13144,"inlinee":4575,"source_line":4031,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1518,"record_offset":15824,"parent_record_offset":15796,"ancestor_record_offset":13144,"inlinee":6215,"source_line":940,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1518,"record_offset":13864,"parent_record_offset":13840,"ancestor_record_offset":13144,"inlinee":4562,"source_line":560,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1518,"record_offset":15972,"parent_record_offset":15948,"ancestor_record_offset":13144,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":14480,"parent_record_offset":14456,"ancestor_record_offset":13144,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$417::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":14536,"parent_record_offset":14512,"ancestor_record_offset":13144,"inlinee":13665,"source_line":886,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1518,"record_offset":14236,"parent_record_offset":14212,"ancestor_record_offset":13144,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1518,"record_offset":14104,"parent_record_offset":14080,"ancestor_record_offset":13144,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1518,"record_offset":13840,"parent_record_offset":13804,"ancestor_record_offset":13144,"inlinee":4561,"source_line":55,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\index_range.rs
//     evidence={"module":1518,"record_offset":13804,"parent_record_offset":13780,"ancestor_record_offset":13144,"inlinee":4559,"source_line":121,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":15596,"parent_record_offset":15572,"ancestor_record_offset":13144,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1518,"record_offset":13576,"parent_record_offset":13544,"ancestor_record_offset":13144,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1518,"record_offset":15224,"parent_record_offset":15192,"ancestor_record_offset":13144,"inlinee":4384,"source_line":933,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1518,"record_offset":14008,"parent_record_offset":13920,"ancestor_record_offset":13144,"inlinee":4567,"source_line":1270,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":15772,"parent_record_offset":15656,"ancestor_record_offset":13144,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1518,"record_offset":14388,"parent_record_offset":14268,"ancestor_record_offset":13144,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":13144,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1518,"record_offset":15160,"parent_record_offset":15016,"ancestor_record_offset":13144,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1518,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13144,"inlinee":5263,"source_line":656,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1518,"record_offset":13976,"parent_record_offset":13952,"ancestor_record_offset":13144,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1518,"record_offset":13952,"parent_record_offset":13920,"ancestor_record_offset":13144,"inlinee":4162,"source_line":242,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1518,"record_offset":13544,"parent_record_offset":13504,"ancestor_record_offset":13144,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1518,"record_offset":13608,"parent_record_offset":13576,"ancestor_record_offset":13144,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14948,"parent_record_offset":14912,"ancestor_record_offset":13144,"inlinee":48744,"source_line":1716,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14696,"parent_record_offset":14636,"ancestor_record_offset":13144,"inlinee":11444,"source_line":952,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14852,"parent_record_offset":14636,"ancestor_record_offset":13144,"inlinee":35796,"source_line":919,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14356,"parent_record_offset":14324,"ancestor_record_offset":13144,"inlinee":4604,"source_line":1001,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14744,"parent_record_offset":14696,"ancestor_record_offset":13144,"inlinee":4603,"source_line":971,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14324,"parent_record_offset":14268,"ancestor_record_offset":13144,"inlinee":11432,"source_line":984,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":14424,"parent_record_offset":14268,"ancestor_record_offset":13144,"inlinee":6214,"source_line":659,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":15796,"parent_record_offset":15772,"ancestor_record_offset":13144,"inlinee":6212,"source_line":2118,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":15684,"parent_record_offset":15656,"ancestor_record_offset":13144,"inlinee":43484,"source_line":2228,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1518,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":13144,"inlinee":35793,"source_line":2237,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1518,"record_offset":14128,"parent_record_offset":14104,"ancestor_record_offset":13144,"inlinee":71525,"source_line":271,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs
//     evidence={"module":1518,"record_offset":15476,"parent_record_offset":15444,"ancestor_record_offset":13144,"inlinee":71526,"source_line":382,"archive":"_pdb/B/modules/01518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 271
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11765
// 首条位置: {"module":1518,"inlinee":71525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 记录形式: compiled_procedure; instances=1
// 源码行: 311
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1518,"record_offset":16732,"rva":19384304,"va":5388093424}
// 编译实例 RVAs: 0x127c7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb70
//     owner=unresolved_namespace; count=1
//     target_aliases=211 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19140001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e85ff2dbfe","size":5,"soft_fail":false,"target_va":5368953712,"va":5388093708,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408e4750
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: handle_splitting_roof_decorators.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10060029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e8037f66ff","size":5,"soft_fail":false,"target_va":5378033488,"va":5388093512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-512277","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e8eb2ef8ff","size":5,"soft_fail":false,"target_va":5387581248,"va":5388093520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-512656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e8702df8ff","size":5,"soft_fail":false,"target_va":5387580992,"va":5388093643,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::edge_length
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6159929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e8c701a2ff","size":5,"soft_fail":false,"target_va":5381933648,"va":5388093572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_closest_edge_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6167502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388093424,"raw_hex":"e832e4a1ff","size":5,"soft_fail":false,"target_va":5381926064,"va":5388093561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 382
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11766
// 首条位置: {"module":1518,"inlinee":71526}
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
// 完整结构及原始类型记录: handle_splitting_roof_decorators.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>", "constructor": "array$", "arguments": ["system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >", "constructor": "smallvec::IntoIter", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["smallvec::IntoIter<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: handle_splitting_roof_decorators.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: handle_splitting_roof_decorators.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: d6c2547c33054ea128226358
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=1518, inline_record=4932; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$0
// 信息 ID: ec5a2a925e42defb49713223
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id; module=1518, inline_record=6240; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$1
// 信息 ID: ecbeb4ac48cd9ba90027f85c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$1,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x127a742 -> 0x127b570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$2
// 信息 ID: ac8660e1fe087e678f4c87e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$2,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x127a782 -> 0x127b730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure$3
// 信息 ID: 53ef15c0cf792aa532c493ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3 <- core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] core::ops::function::FnOnce::call_once<system_decorator::ui_systems::handle_splitting_roof_decorators::display_roof_decorator_parts_and_get_hovered_part_id::closure_env$3,tuple$<ref$<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,ref$<country_core::asset_libraries::shader_library::AssetShaderLibrary>,ref_mut$<bevy_ecs::system::commands::Commands> > > @ 0x127a762 -> 0x127b8f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part
// 信息 ID: 3968bd5ec7bc25a765be3d72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a5c4b -> 0x127bab0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$0
// 信息 ID: 2152b1090f6f1f4315735784
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=1518, inline_record=14128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$1
// 信息 ID: f0b84e29eb22d5d0ec2dad47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x127c000 -> 0x127c7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x127c184 -> 0x127c7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part @ 0x127c34c -> 0x127c7f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::closure$3
// 信息 ID: a0925c007b18cd91347061ce
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part; module=1518, inline_record=15476; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id
// 信息 ID: 8c4ba4e6c92792e326aabe06
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a721c -> 0x127a790 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
