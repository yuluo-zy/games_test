// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 1cdfe72ba7eb2b535e22eb8c995b3b4d5fd4538369cce62dc9e7d2baa23ee8dc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: decorator_interaction_intent.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 1522, 1527, 1576, 3051, 3053
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
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.20.5"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.25.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.27.0"},{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.29.3"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_decorator; // namespace candidate
//   package identities: [{"name":"system-decorator","source":"local_or_path","version":"0.1.0"}]
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1522,"record_offset":14064,"rva":19394784,"va":5388103904}
// 编译实例 RVAs: 0x127f0e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7705855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8016b8aff","size":5,"soft_fail":false,"target_va":5380398224,"va":5388104074,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6171407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8f1d4a1ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5388105962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411f7dc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-553971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e80d8cf7ff","size":5,"soft_fail":false,"target_va":5387550144,"va":5388104110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412898e0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8b9a00000","size":5,"soft_fail":false,"target_va":5388146912,"va":5388105762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23435065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e839976501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388106706,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23434938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8ba966501","size":5,"soft_fail":false,"target_va":5411542064,"va":5388107121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23435158","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e896976501","size":5,"soft_fail":false,"target_va":5411542296,"va":5388107133,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23435741","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8dd996501","size":5,"soft_fail":false,"target_va":5411541904,"va":5388106158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10076870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e83a3d66ff","size":5,"soft_fail":false,"target_va":5378028416,"va":5388105281,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::collision::world::RaycastWorld::raycast_simple
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10169616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8f0d264ff","size":5,"soft_fail":false,"target_va":5377936416,"va":5388106027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::signed_distance
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6190628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8dc89a1ff","size":5,"soft_fail":false,"target_va":5381914912,"va":5388105535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::contains_point_padding
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6180951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388103904,"raw_hex":"e8a9afa1ff","size":5,"soft_fail":false,"target_va":5381924512,"va":5388105458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1522,"record_offset":17508,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":17100,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":19900,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::Range::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":19844,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":7522,"source_line":120,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":14268,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":25059,"source_line":223,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":1522,"record_offset":19816,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":4919,"source_line":69,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":16924,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":17012,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_2d_ws_distance
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":19928,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":25132,"source_line":1161,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":20168,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1522,"record_offset":15568,"parent_record_offset":14064,"ancestor_record_offset":14064,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":17340,"parent_record_offset":17316,"ancestor_record_offset":14064,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":17388,"parent_record_offset":17364,"ancestor_record_offset":14064,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":17364,"parent_record_offset":17340,"ancestor_record_offset":14064,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":17316,"parent_record_offset":17292,"ancestor_record_offset":14064,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":17292,"parent_record_offset":17260,"ancestor_record_offset":14064,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":17876,"parent_record_offset":17852,"ancestor_record_offset":14064,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":17900,"parent_record_offset":17876,"ancestor_record_offset":14064,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":17260,"parent_record_offset":17232,"ancestor_record_offset":14064,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":19416,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":18636,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":18720,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":19612,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":18552,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":19328,"parent_record_offset":19304,"ancestor_record_offset":14064,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":18812,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":17936,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":19304,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":19352,"parent_record_offset":19328,"ancestor_record_offset":14064,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1522,"record_offset":17208,"parent_record_offset":17124,"ancestor_record_offset":14064,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1522,"record_offset":17232,"parent_record_offset":17208,"ancestor_record_offset":14064,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":1522,"record_offset":17532,"parent_record_offset":17508,"ancestor_record_offset":14064,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1522,"record_offset":18352,"parent_record_offset":18328,"ancestor_record_offset":14064,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":18036,"parent_record_offset":18012,"ancestor_record_offset":14064,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":18012,"parent_record_offset":17988,"ancestor_record_offset":14064,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":18212,"parent_record_offset":18188,"ancestor_record_offset":14064,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":17988,"parent_record_offset":17960,"ancestor_record_offset":14064,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":17960,"parent_record_offset":17936,"ancestor_record_offset":14064,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":17852,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":19376,"parent_record_offset":19352,"ancestor_record_offset":14064,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":17768,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1522,"record_offset":18328,"parent_record_offset":18304,"ancestor_record_offset":14064,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":18304,"parent_record_offset":18280,"ancestor_record_offset":14064,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":14064,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":14064,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":17152,"parent_record_offset":17124,"ancestor_record_offset":14064,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":21400,"parent_record_offset":21376,"ancestor_record_offset":14064,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1522,"record_offset":19740,"parent_record_offset":19716,"ancestor_record_offset":14064,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":15212,"parent_record_offset":15184,"ancestor_record_offset":14064,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1522,"record_offset":14624,"parent_record_offset":14596,"ancestor_record_offset":14064,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1522,"record_offset":14720,"parent_record_offset":14692,"ancestor_record_offset":14064,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":20752,"parent_record_offset":20728,"ancestor_record_offset":14064,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1522,"record_offset":14652,"parent_record_offset":14624,"ancestor_record_offset":14064,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1522,"record_offset":15452,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1522,"record_offset":14908,"parent_record_offset":14872,"ancestor_record_offset":14064,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1522,"record_offset":14872,"parent_record_offset":14836,"ancestor_record_offset":14064,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":17040,"parent_record_offset":17012,"ancestor_record_offset":14064,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":16952,"parent_record_offset":16924,"ancestor_record_offset":14064,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":20392,"parent_record_offset":20368,"ancestor_record_offset":14064,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1522,"record_offset":18124,"parent_record_offset":18064,"ancestor_record_offset":14064,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":14064,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":18148,"parent_record_offset":18124,"ancestor_record_offset":14064,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":18188,"parent_record_offset":18012,"ancestor_record_offset":14064,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":24180,"parent_record_offset":24148,"ancestor_record_offset":14064,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":15508,"parent_record_offset":14268,"ancestor_record_offset":14064,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1522,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":14064,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1522,"record_offset":15052,"parent_record_offset":15024,"ancestor_record_offset":14064,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1522,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":14064,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1522,"record_offset":17652,"parent_record_offset":17616,"ancestor_record_offset":14064,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1522,"record_offset":17792,"parent_record_offset":17768,"ancestor_record_offset":14064,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":20212,"parent_record_offset":20168,"ancestor_record_offset":14064,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":17816,"parent_record_offset":17792,"ancestor_record_offset":14064,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":20240,"parent_record_offset":20212,"ancestor_record_offset":14064,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1522,"record_offset":17432,"parent_record_offset":17260,"ancestor_record_offset":14064,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":14064,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":17616,"parent_record_offset":17532,"ancestor_record_offset":14064,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":17680,"parent_record_offset":17616,"ancestor_record_offset":14064,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$22::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":15240,"parent_record_offset":15212,"ancestor_record_offset":14064,"inlinee":24799,"source_line":94,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::get_collision_bbx_ls::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":15532,"parent_record_offset":15508,"ancestor_record_offset":14064,"inlinee":25042,"source_line":226,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1522,"record_offset":20424,"parent_record_offset":20168,"ancestor_record_offset":14064,"inlinee":4590,"source_line":1245,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1522,"record_offset":20480,"parent_record_offset":20168,"ancestor_record_offset":14064,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1522,"record_offset":20368,"parent_record_offset":20168,"ancestor_record_offset":14064,"inlinee":4743,"source_line":1473,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1522,"record_offset":14792,"parent_record_offset":14756,"ancestor_record_offset":14064,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1522,"record_offset":14836,"parent_record_offset":14792,"ancestor_record_offset":14064,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1522,"record_offset":14756,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1522,"record_offset":14596,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1522,"record_offset":15288,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1522,"record_offset":14692,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1522,"record_offset":14568,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1522,"record_offset":15184,"parent_record_offset":15156,"ancestor_record_offset":14064,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1522,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":14064,"inlinee":4719,"source_line":1304,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1522,"record_offset":14344,"parent_record_offset":14320,"ancestor_record_offset":14064,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1522,"record_offset":15156,"parent_record_offset":14956,"ancestor_record_offset":14064,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1522,"record_offset":14376,"parent_record_offset":14344,"ancestor_record_offset":14064,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":15088,"parent_record_offset":14984,"ancestor_record_offset":14064,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":15024,"parent_record_offset":14984,"ancestor_record_offset":14064,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":15408,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14984,"parent_record_offset":14956,"ancestor_record_offset":14064,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14428,"parent_record_offset":14404,"ancestor_record_offset":14064,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14404,"parent_record_offset":14344,"ancestor_record_offset":14064,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14456,"parent_record_offset":14428,"ancestor_record_offset":14064,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14540,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1522,"record_offset":14956,"parent_record_offset":14456,"ancestor_record_offset":14064,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1522,"record_offset":18236,"parent_record_offset":18212,"ancestor_record_offset":14064,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1522,"record_offset":14296,"parent_record_offset":14268,"ancestor_record_offset":14064,"inlinee":4718,"source_line":894,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":20276,"parent_record_offset":20168,"ancestor_record_offset":14064,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":20108,"parent_record_offset":19928,"ancestor_record_offset":14064,"inlinee":24439,"source_line":338,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":20080,"parent_record_offset":19928,"ancestor_record_offset":14064,"inlinee":24465,"source_line":236,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":19964,"parent_record_offset":19928,"ancestor_record_offset":14064,"inlinee":25128,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":19992,"parent_record_offset":19964,"ancestor_record_offset":14064,"inlinee":24435,"source_line":110,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":20048,"parent_record_offset":20024,"ancestor_record_offset":14064,"inlinee":24464,"source_line":196,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1522,"record_offset":20024,"parent_record_offset":19928,"ancestor_record_offset":14064,"inlinee":24463,"source_line":220,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1522,"record_offset":1440,"rva":19386224,"va":5388095344}
// 编译实例 RVAs: 0x127cf70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002370
//     owner=unresolved_namespace; count=3
//     target_aliases=1937 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19377779","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e88d51d8fe","size":5,"soft_fail":false,"target_va":5368718192,"va":5388095966,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14003bb80
//     owner=unresolved_namespace; count=1
//     target_aliases=532 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-19142229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8abe9dbfe","size":5,"soft_fail":false,"target_va":5368953728,"va":5388095952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13014819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8dd6839ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5388096318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833180
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10789547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8555d5bff","size":5,"soft_fail":false,"target_va":5377307008,"va":5388096550,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140833250
//     owner=unresolved_namespace; count=2
//     target_aliases=14 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10788594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e80e615bff","size":5,"soft_fail":false,"target_va":5377307216,"va":5388095805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1408c5670
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10192708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8bc7864ff","size":5,"soft_fail":false,"target_va":5377906288,"va":5388098991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140959e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9588035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8bdb26dff","size":5,"soft_fail":false,"target_va":5378514544,"va":5388102574,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a1e190
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8777898","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8560f7aff","size":5,"soft_fail":false,"target_va":5379318160,"va":5388096053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b25c90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7697602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e83e8b8aff","size":5,"soft_fail":false,"target_va":5380398224,"va":5388095821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6212065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e81f36a1ff","size":5,"soft_fail":false,"target_va":5381889600,"va":5388101660,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91eb0
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6209241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e82741a1ff","size":5,"soft_fail":false,"target_va":5381889712,"va":5388098948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91f60
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6210414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8923ca1ff","size":5,"soft_fail":false,"target_va":5381889888,"va":5388100297,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c9cde0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6166703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e851e7a1ff","size":5,"soft_fail":false,"target_va":5381934560,"va":5388101258,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14125cf90
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-131552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e820fefdff","size":5,"soft_fail":false,"target_va":5387964304,"va":5388095851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141269710
//     owner=unresolved_namespace; count=5
//     target_aliases=6 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-81314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e85ec2feff","size":5,"soft_fail":false,"target_va":5388015376,"va":5388096685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14126a040
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-78188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e894cefeff","size":5,"soft_fail":false,"target_va":5388017728,"va":5388095911,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141294480
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t88241","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8b1580100","size":5,"soft_fail":false,"target_va":5388190848,"va":5388102602,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe5c20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14059907","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e88389d600","size":5,"soft_fail":false,"target_va":5402156064,"va":5388096152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141fe7280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: decorator_interaction_intent.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14062134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e83692d600","size":5,"soft_fail":false,"target_va":5402161792,"va":5388099653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9470562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e89e7d6fff","size":5,"soft_fail":false,"target_va":5378631328,"va":5388101885,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-4819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e82dedffff","size":5,"soft_fail":false,"target_va":5388094944,"va":5388099758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t23439207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e867a76501","size":5,"soft_fail":false,"target_va":5411541776,"va":5388102564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t23442130","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8d2b26501","size":5,"soft_fail":false,"target_va":5411542064,"va":5388099929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t23439904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e820aa6501","size":5,"soft_fail":false,"target_va":5411542296,"va":5388102387,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t23442778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e85ab56501","size":5,"soft_fail":false,"target_va":5411541904,"va":5388099121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t23440068","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8c4aa6501","size":5,"soft_fail":false,"target_va":5411542880,"va":5388102807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10069787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8e55866ff","size":5,"soft_fail":false,"target_va":5378028416,"va":5388098198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10067946","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8166066ff","size":5,"soft_fail":false,"target_va":5378030352,"va":5388098293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::WallId::is_valid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7641245","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e863678bff","size":5,"soft_fail":false,"target_va":5380460320,"va":5388101560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10793993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8f74b5bff","size":5,"soft_fail":false,"target_va":5377307552,"va":5388101540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10787226","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e866665bff","size":5,"soft_fail":false,"target_va":5377309216,"va":5388096437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10790939","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8e5575bff","size":5,"soft_fail":false,"target_va":5377309184,"va":5388100118,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorRotationXZ::offset
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10790573","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e853595bff","size":5,"soft_fail":false,"target_va":5377309568,"va":5388100136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10789370","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8065e5bff","size":5,"soft_fail":false,"target_va":5377307056,"va":5388096421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::max_rank
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10792424","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e818525bff","size":5,"soft_fail":false,"target_va":5377306928,"va":5388099347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10791904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e820545bff","size":5,"soft_fail":false,"target_va":5377307072,"va":5388098971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::wall_space
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7678381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e853d68aff","size":5,"soft_fail":false,"target_va":5380417344,"va":5388095720,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8303081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8174e81ff","size":5,"soft_fail":false,"target_va":5379792624,"va":5388095700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-8304433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8cf4881ff","size":5,"soft_fail":false,"target_va":5379793584,"va":5388098012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7704337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8ef708aff","size":5,"soft_fail":false,"target_va":5380396736,"va":5388101068,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-9426634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8362970ff","size":5,"soft_fail":false,"target_va":5378674096,"va":5388100725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::dashed
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7838325","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e88b6588ff","size":5,"soft_fail":false,"target_va":5380263696,"va":5388102016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::line_width
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7838263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8c96588ff","size":5,"soft_fail":false,"target_va":5380263744,"va":5388102002,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7838442","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8166588ff","size":5,"soft_fail":false,"target_va":5380263536,"va":5388101973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugLine::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7838244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8dc6588ff","size":5,"soft_fail":false,"target_va":5380263792,"va":5388102031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7847981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8d33f88ff","size":5,"soft_fail":false,"target_va":5380252816,"va":5388100792,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::radius
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7847930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8064088ff","size":5,"soft_fail":false,"target_va":5380252896,"va":5388100821,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::show
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7847895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8294088ff","size":5,"soft_fail":false,"target_va":5380252960,"va":5388100850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::utils::debug_viz::DebugSphere::solid
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-7847912","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8184088ff","size":5,"soft_fail":false,"target_va":5380252928,"va":5388100835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] glam::f32::sse2::vec4::Vec4::is_finite_mask
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"glam","version":"0.20.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.25.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.27.0","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t241366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8d6ae0300","size":5,"soft_fail":false,"target_va":5388336976,"va":5388095605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10931393","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e83f3359ff","size":5,"soft_fail":false,"target_va":5377169664,"va":5388101052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::derive_decorator_subtype::derive_decorator_subtype
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-10916915","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e8cd6b59ff","size":5,"soft_fail":false,"target_va":5377178880,"va":5388095790,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::merge_proposal_wall_decorators::find_best_wall_decorator_merge_position
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t14173545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e86945d800","size":5,"soft_fail":false,"target_va":5402269824,"va":5388096274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e867090000","size":5,"soft_fail":false,"target_va":5388103904,"va":5388101492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-6178447","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5388095344,"raw_hex":"e871b9a1ff","size":5,"soft_fail":false,"target_va":5381920464,"va":5388098906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1522,"record_offset":13336,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":7065,"source_line":526,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":12452,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":9632,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":11232,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":9540,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":12548,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1522,"record_offset":13768,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1522,"record_offset":5088,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":5172,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4466,"source_line":296,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator::impl$58::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1522,"record_offset":12480,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":25001,"source_line":277,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":12308,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1522,"record_offset":1768,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":25835,"source_line":522,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":13196,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":12080,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4745,"source_line":516,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":12276,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4741,"source_line":891,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":13108,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1522,"record_offset":1712,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":25834,"source_line":420,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec3.rs
//     evidence={"module":1522,"record_offset":13224,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4752,"source_line":1333,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":1852,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":71552,"source_line":477,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":5348,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":71558,"source_line":408,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":11328,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":11268,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":11580,"parent_record_offset":1440,"ancestor_record_offset":1440,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":1440,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":2760,"parent_record_offset":2668,"ancestor_record_offset":1440,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":1440,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":1440,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":1440,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":10028,"parent_record_offset":10004,"ancestor_record_offset":1440,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":10004,"parent_record_offset":9980,"ancestor_record_offset":1440,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1440,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1522,"record_offset":13360,"parent_record_offset":13336,"ancestor_record_offset":1440,"inlinee":7066,"source_line":99,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":1440,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":9932,"parent_record_offset":9832,"ancestor_record_offset":1440,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":13384,"parent_record_offset":13360,"ancestor_record_offset":1440,"inlinee":7067,"source_line":934,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":2500,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":3696,"parent_record_offset":3644,"ancestor_record_offset":1440,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":10368,"parent_record_offset":10344,"ancestor_record_offset":1440,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":1522,"record_offset":10132,"parent_record_offset":9800,"ancestor_record_offset":1440,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":1522,"record_offset":9800,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":11080,"parent_record_offset":11056,"ancestor_record_offset":1440,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":10604,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":10628,"parent_record_offset":10604,"ancestor_record_offset":1440,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":11056,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":1522,"record_offset":11104,"parent_record_offset":11080,"ancestor_record_offset":1440,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":1522,"record_offset":11004,"parent_record_offset":10980,"ancestor_record_offset":1440,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":10728,"parent_record_offset":10704,"ancestor_record_offset":1440,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":10704,"parent_record_offset":10680,"ancestor_record_offset":1440,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":1522,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":1440,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":10680,"parent_record_offset":10652,"ancestor_record_offset":1440,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":1440,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":10344,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":1522,"record_offset":10980,"parent_record_offset":10956,"ancestor_record_offset":1440,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":1522,"record_offset":10956,"parent_record_offset":10932,"ancestor_record_offset":1440,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::as_readonly
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":9664,"parent_record_offset":9632,"ancestor_record_offset":1440,"inlinee":13705,"source_line":542,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::copy_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":1440,"inlinee":13707,"source_line":636,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":9748,"parent_record_offset":9632,"ancestor_record_offset":1440,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::reborrow_unsafe
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":1522,"record_offset":9688,"parent_record_offset":9664,"ancestor_record_offset":1440,"inlinee":13706,"source_line":618,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":1522,"record_offset":11128,"parent_record_offset":11104,"ancestor_record_offset":1440,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":1522,"record_offset":10192,"parent_record_offset":10164,"ancestor_record_offset":1440,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":9240,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":1440,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":10100,"parent_record_offset":10076,"ancestor_record_offset":1440,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":1440,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":1440,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":8952,"parent_record_offset":8900,"ancestor_record_offset":1440,"inlinee":14050,"source_line":1502,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4548,"parent_record_offset":4128,"ancestor_record_offset":1440,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4456,"parent_record_offset":4400,"ancestor_record_offset":1440,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":1440,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4256,"parent_record_offset":4172,"ancestor_record_offset":1440,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4320,"parent_record_offset":4128,"ancestor_record_offset":1440,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":3228,"parent_record_offset":3152,"ancestor_record_offset":1440,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1440,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_finite
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":1736,"parent_record_offset":1712,"ancestor_record_offset":1440,"inlinee":9544,"source_line":572,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":7604,"parent_record_offset":7428,"ancestor_record_offset":1440,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":9368,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":9340,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":7632,"parent_record_offset":7428,"ancestor_record_offset":1440,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8796,"parent_record_offset":8764,"ancestor_record_offset":1440,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8732,"parent_record_offset":8700,"ancestor_record_offset":1440,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":1440,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8700,"parent_record_offset":8584,"ancestor_record_offset":1440,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8848,"parent_record_offset":8796,"ancestor_record_offset":1440,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":1440,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1522,"record_offset":8492,"parent_record_offset":7288,"ancestor_record_offset":1440,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1522,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":1440,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1522,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":1440,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1522,"record_offset":7204,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8552,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8900,"parent_record_offset":8848,"ancestor_record_offset":1440,"inlinee":14049,"source_line":3265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8584,"parent_record_offset":8552,"ancestor_record_offset":1440,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1522,"record_offset":9212,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1522,"record_offset":2728,"parent_record_offset":2696,"ancestor_record_offset":1440,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1522,"record_offset":10076,"parent_record_offset":9800,"ancestor_record_offset":1440,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":1440,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":1440,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":1440,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":1440,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1522,"record_offset":10776,"parent_record_offset":10752,"ancestor_record_offset":1440,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1522,"record_offset":9004,"parent_record_offset":8952,"ancestor_record_offset":1440,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":1440,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7476,"parent_record_offset":7428,"ancestor_record_offset":1440,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2528,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":10800,"parent_record_offset":10776,"ancestor_record_offset":1440,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":1440,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2832,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":1440,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":4760,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":10840,"parent_record_offset":10704,"ancestor_record_offset":1440,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2120,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":1440,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":2596,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1522,"record_offset":5136,"parent_record_offset":5112,"ancestor_record_offset":1440,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1522,"record_offset":5228,"parent_record_offset":5200,"ancestor_record_offset":1440,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1522,"record_offset":11196,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1522,"record_offset":9832,"parent_record_offset":9800,"ancestor_record_offset":1440,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":9900,"parent_record_offset":9872,"ancestor_record_offset":1440,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":1440,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":9872,"parent_record_offset":9832,"ancestor_record_offset":1440,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":1440,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":1522,"record_offset":5112,"parent_record_offset":5088,"ancestor_record_offset":1440,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":8764,"parent_record_offset":8732,"ancestor_record_offset":1440,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":5200,"parent_record_offset":5172,"ancestor_record_offset":1440,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1522,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":1440,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1522,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":1440,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$89::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1522,"record_offset":12516,"parent_record_offset":12480,"ancestor_record_offset":1440,"inlinee":25003,"source_line":351,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2284,"parent_record_offset":2192,"ancestor_record_offset":1440,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2192,"parent_record_offset":2156,"ancestor_record_offset":1440,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2156,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":1440,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1522,"record_offset":2472,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1522,"record_offset":10164,"parent_record_offset":9748,"ancestor_record_offset":1440,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1522,"record_offset":10240,"parent_record_offset":10216,"ancestor_record_offset":1440,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1522,"record_offset":10264,"parent_record_offset":10240,"ancestor_record_offset":1440,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":1522,"record_offset":10216,"parent_record_offset":10192,"ancestor_record_offset":1440,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::bool::sse2::bvec4a::BVec4A::all
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/bool/sse2/bvec4a.rs
//     evidence={"module":1522,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1440,"inlinee":25838,"source_line":91,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1522,"record_offset":4104,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1522,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":1440,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1522,"record_offset":4576,"parent_record_offset":4128,"ancestor_record_offset":1440,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec4::Vec4::is_finite
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec4.rs
//     evidence={"module":1522,"record_offset":1792,"parent_record_offset":1768,"ancestor_record_offset":1440,"inlinee":25836,"source_line":421,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6160,"parent_record_offset":5460,"ancestor_record_offset":1440,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":12164,"parent_record_offset":12140,"ancestor_record_offset":1440,"inlinee":4748,"source_line":396,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":12140,"parent_record_offset":12108,"ancestor_record_offset":1440,"inlinee":4747,"source_line":415,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6244,"parent_record_offset":5460,"ancestor_record_offset":1440,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":3920,"parent_record_offset":3892,"ancestor_record_offset":1440,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":12108,"parent_record_offset":12080,"ancestor_record_offset":1440,"inlinee":4746,"source_line":499,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":1440,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":4032,"parent_record_offset":3920,"ancestor_record_offset":1440,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6104,"parent_record_offset":5460,"ancestor_record_offset":1440,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6188,"parent_record_offset":5460,"ancestor_record_offset":1440,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1522,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":1440,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1522,"record_offset":4172,"parent_record_offset":4128,"ancestor_record_offset":1440,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":1522,"record_offset":10888,"parent_record_offset":10864,"ancestor_record_offset":1440,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\f32.rs
//     evidence={"module":1522,"record_offset":12228,"parent_record_offset":12196,"ancestor_record_offset":1440,"inlinee":4750,"source_line":369,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1522,"record_offset":2376,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":71554,"source_line":114,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1522,"record_offset":3152,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":25626,"source_line":1168,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":4728,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":71565,"source_line":920,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":9572,"parent_record_offset":9540,"ancestor_record_offset":1440,"inlinee":71568,"source_line":135,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":5260,"parent_record_offset":5172,"ancestor_record_offset":1440,"inlinee":71557,"source_line":362,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":13452,"parent_record_offset":13420,"ancestor_record_offset":1440,"inlinee":71567,"source_line":363,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":1440,"inlinee":71556,"source_line":553,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":1440,"inlinee":71566,"source_line":633,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":5460,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":71560,"source_line":418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7336,"parent_record_offset":7288,"ancestor_record_offset":1440,"inlinee":71562,"source_line":436,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7288,"parent_record_offset":5348,"ancestor_record_offset":1440,"inlinee":71561,"source_line":441,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":9056,"parent_record_offset":9004,"ancestor_record_offset":1440,"inlinee":71563,"source_line":455,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":1440,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3620,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":1440,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3780,"parent_record_offset":3644,"ancestor_record_offset":1440,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":7428,"parent_record_offset":7368,"ancestor_record_offset":1440,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1440,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5504,"parent_record_offset":5460,"ancestor_record_offset":1440,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1522,"record_offset":2336,"parent_record_offset":2284,"ancestor_record_offset":1440,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1522,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1440,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1522,"record_offset":3892,"parent_record_offset":3644,"ancestor_record_offset":1440,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":3124,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":4897,"source_line":116,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":3200,"parent_record_offset":3152,"ancestor_record_offset":1440,"inlinee":25627,"source_line":284,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":4820,"parent_record_offset":1852,"ancestor_record_offset":1440,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11790
// 首条位置: {"module":1522,"inlinee":71568}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":9596,"parent_record_offset":9572,"ancestor_record_offset":9572,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 266
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EF9
// 首条位置: {"module":3053,"inlinee":106233}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3053,"record_offset":3552,"parent_record_offset":3532,"ancestor_record_offset":3532,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03053.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3053,"record_offset":3576,"parent_record_offset":3552,"ancestor_record_offset":3532,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/03053.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 275
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1177E
// 首条位置: {"module":1522,"inlinee":71550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 362
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11785
// 首条位置: {"module":1522,"inlinee":71557}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::stairs::stairs_state::impl$16::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/stairs/stairs_state.rs
//     evidence={"module":1522,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":5260,"inlinee":4501,"source_line":38,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":1522,"record_offset":5308,"parent_record_offset":5284,"ancestor_record_offset":5260,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 363
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1178F
// 首条位置: {"module":1522,"inlinee":71567}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":13496,"parent_record_offset":13452,"ancestor_record_offset":13452,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":13524,"parent_record_offset":13452,"ancestor_record_offset":13452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":13712,"parent_record_offset":13688,"ancestor_record_offset":13452,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":13688,"parent_record_offset":13596,"ancestor_record_offset":13452,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":13624,"parent_record_offset":13596,"ancestor_record_offset":13452,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":13596,"parent_record_offset":13572,"ancestor_record_offset":13452,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":13572,"parent_record_offset":13548,"ancestor_record_offset":13452,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1522,"record_offset":13656,"parent_record_offset":13624,"ancestor_record_offset":13452,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":13452,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 408
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11786
// 首条位置: {"module":1522,"inlinee":71558}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":9240,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":9368,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4551,"source_line":890,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":9340,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4754,"source_line":912,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":9508,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1522,"record_offset":7204,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8552,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":6059,"source_line":3258,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1522,"record_offset":9212,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":9396,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":5460,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":71560,"source_line":418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":9268,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":71562,"source_line":436,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7288,"parent_record_offset":5348,"ancestor_record_offset":5348,"inlinee":71561,"source_line":441,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":5348,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":5348,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":5348,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$76::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7252,"parent_record_offset":7228,"ancestor_record_offset":5348,"inlinee":10985,"source_line":1824,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":8952,"parent_record_offset":8900,"ancestor_record_offset":5348,"inlinee":14050,"source_line":1502,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":6300,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":7604,"parent_record_offset":7428,"ancestor_record_offset":5348,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":7632,"parent_record_offset":7428,"ancestor_record_offset":5348,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::copy_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8796,"parent_record_offset":8764,"ancestor_record_offset":5348,"inlinee":9582,"source_line":30,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8732,"parent_record_offset":8700,"ancestor_record_offset":5348,"inlinee":9580,"source_line":71,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\copied.rs
//     evidence={"module":1522,"record_offset":8640,"parent_record_offset":8616,"ancestor_record_offset":5348,"inlinee":5527,"source_line":45,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8700,"parent_record_offset":8584,"ancestor_record_offset":5348,"inlinee":14219,"source_line":96,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8848,"parent_record_offset":8796,"ancestor_record_offset":5348,"inlinee":9871,"source_line":106,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1522,"record_offset":8616,"parent_record_offset":8584,"ancestor_record_offset":5348,"inlinee":4732,"source_line":46,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1522,"record_offset":8492,"parent_record_offset":7288,"ancestor_record_offset":5348,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1522,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":5348,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":1522,"record_offset":7228,"parent_record_offset":7204,"ancestor_record_offset":5348,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::min_by::fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8900,"parent_record_offset":8848,"ancestor_record_offset":5348,"inlinee":14049,"source_line":3265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1522,"record_offset":8584,"parent_record_offset":8552,"ancestor_record_offset":5348,"inlinee":6060,"source_line":2576,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":5348,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5348,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":5348,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5348,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1522,"record_offset":9004,"parent_record_offset":8952,"ancestor_record_offset":5348,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":5348,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7476,"parent_record_offset":7428,"ancestor_record_offset":5348,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":8664,"parent_record_offset":8640,"ancestor_record_offset":5348,"inlinee":4665,"source_line":1892,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":9108,"parent_record_offset":9056,"ancestor_record_offset":5348,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":5560,"parent_record_offset":5504,"ancestor_record_offset":5348,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":5348,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1522,"record_offset":8764,"parent_record_offset":8732,"ancestor_record_offset":5348,"inlinee":4977,"source_line":212,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6160,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6244,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":5348,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":5952,"parent_record_offset":5924,"ancestor_record_offset":5348,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6036,"parent_record_offset":5924,"ancestor_record_offset":5348,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6104,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6188,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7368,"parent_record_offset":7288,"ancestor_record_offset":5348,"inlinee":71560,"source_line":418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7336,"parent_record_offset":7288,"ancestor_record_offset":5348,"inlinee":71562,"source_line":436,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":9056,"parent_record_offset":9004,"ancestor_record_offset":5348,"inlinee":71563,"source_line":455,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5704,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":5348,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5616,"parent_record_offset":5504,"ancestor_record_offset":5348,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":7428,"parent_record_offset":7368,"ancestor_record_offset":5348,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5504,"parent_record_offset":5460,"ancestor_record_offset":5348,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1522,"record_offset":5896,"parent_record_offset":5704,"ancestor_record_offset":5348,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 418
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11788
// 首条位置: {"module":1522,"inlinee":71560}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":6300,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6160,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6244,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6104,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6188,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5704,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":7428,"parent_record_offset":7368,"ancestor_record_offset":7368,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5504,"parent_record_offset":5460,"ancestor_record_offset":5460,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":5728,"parent_record_offset":5704,"ancestor_record_offset":5460,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":7368,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":7368,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":7604,"parent_record_offset":7428,"ancestor_record_offset":7368,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":7632,"parent_record_offset":7428,"ancestor_record_offset":7368,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6060,"parent_record_offset":6036,"ancestor_record_offset":5460,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":5976,"parent_record_offset":5952,"ancestor_record_offset":5460,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6212,"parent_record_offset":6188,"ancestor_record_offset":5460,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5460,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":7368,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7476,"parent_record_offset":7428,"ancestor_record_offset":7368,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":5560,"parent_record_offset":5504,"ancestor_record_offset":5460,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":5584,"parent_record_offset":5560,"ancestor_record_offset":5460,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":5460,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":5924,"parent_record_offset":5896,"ancestor_record_offset":5460,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":5952,"parent_record_offset":5924,"ancestor_record_offset":5460,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":6036,"parent_record_offset":5924,"ancestor_record_offset":5460,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5532,"parent_record_offset":5504,"ancestor_record_offset":5460,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":5616,"parent_record_offset":5504,"ancestor_record_offset":5460,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1522,"record_offset":5896,"parent_record_offset":5704,"ancestor_record_offset":5460,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 436
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1178A
// 首条位置: {"module":1522,"inlinee":71562}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 441
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11789
// 首条位置: {"module":1522,"inlinee":71561}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::adapters::skip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\skip.rs
//     evidence={"module":1522,"record_offset":8492,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":9541,"source_line":40,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7368,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":71560,"source_line":418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":7336,"parent_record_offset":7288,"ancestor_record_offset":7288,"inlinee":71562,"source_line":436,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":7888,"parent_record_offset":7864,"ancestor_record_offset":7288,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7532,"parent_record_offset":7504,"ancestor_record_offset":7288,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1522,"record_offset":7560,"parent_record_offset":7532,"ancestor_record_offset":7288,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":8460,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":7604,"parent_record_offset":7428,"ancestor_record_offset":7288,"inlinee":5885,"source_line":524,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":1522,"record_offset":7632,"parent_record_offset":7428,"ancestor_record_offset":7288,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\take.rs
//     evidence={"module":1522,"record_offset":8516,"parent_record_offset":8492,"ancestor_record_offset":7288,"inlinee":11744,"source_line":36,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":8220,"parent_record_offset":8196,"ancestor_record_offset":7288,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":8136,"parent_record_offset":8112,"ancestor_record_offset":7288,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$385::sub_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7288,"inlinee":14286,"source_line":828,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":8288,"parent_record_offset":8264,"ancestor_record_offset":7288,"inlinee":4715,"source_line":214,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7504,"parent_record_offset":7476,"ancestor_record_offset":7288,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":1522,"record_offset":7476,"parent_record_offset":7428,"ancestor_record_offset":7288,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":7720,"parent_record_offset":7664,"ancestor_record_offset":7288,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":7744,"parent_record_offset":7720,"ancestor_record_offset":7288,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8320,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":4749,"source_line":166,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_squared
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8404,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":25585,"source_line":406,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8084,"parent_record_offset":8056,"ancestor_record_offset":7288,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8112,"parent_record_offset":8084,"ancestor_record_offset":7288,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8196,"parent_record_offset":8084,"ancestor_record_offset":7288,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8264,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":9561,"source_line":1418,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$54::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":8348,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":14284,"source_line":1452,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":7864,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":7692,"parent_record_offset":7664,"ancestor_record_offset":7288,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":7776,"parent_record_offset":7664,"ancestor_record_offset":7288,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":7428,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":4979,"source_line":89,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":7664,"parent_record_offset":7368,"ancestor_record_offset":7288,"inlinee":4637,"source_line":1392,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1522,"record_offset":8056,"parent_record_offset":7864,"ancestor_record_offset":7288,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 455
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1178B
// 首条位置: {"module":1522,"inlinee":71563}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":9108,"parent_record_offset":9056,"ancestor_record_offset":9056,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 477
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11780
// 首条位置: {"module":1522,"inlinee":71552}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":2500,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2528,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2832,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":5657,"source_line":651,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":4760,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4224,"source_line":628,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2120,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":2596,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2156,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":25147,"source_line":285,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1522,"record_offset":2472,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::sse2::quat::Quat::mul_vec3
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1522,"record_offset":4104,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":11318,"source_line":711,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::collision_detection::DecoratorWorldSpace::project_to_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/collision_detection.rs
//     evidence={"module":1522,"record_offset":2376,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":71554,"source_line":114,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::cull_oob_decorators::should_snap_to_corner
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/cull_oob_decorators.rs
//     evidence={"module":1522,"record_offset":3152,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":25626,"source_line":1168,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":4728,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":71565,"source_line":920,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_pos_at_u
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3620,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":24575,"source_line":739,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":3560,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::new
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":3124,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":4897,"source_line":116,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::with_offset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":4820,"parent_record_offset":1852,"ancestor_record_offset":1852,"inlinee":25625,"source_line":213,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":1852,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1522,"record_offset":2760,"parent_record_offset":2668,"ancestor_record_offset":1852,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2696,"parent_record_offset":2668,"ancestor_record_offset":1852,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2668,"parent_record_offset":2644,"ancestor_record_offset":1852,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1522,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":1852,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1522,"record_offset":3696,"parent_record_offset":3644,"ancestor_record_offset":1852,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4548,"parent_record_offset":4128,"ancestor_record_offset":1852,"inlinee":11327,"source_line":33,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_add_ss
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4456,"parent_record_offset":4400,"ancestor_record_offset":1852,"inlinee":11326,"source_line":21,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_mul_ps
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4224,"parent_record_offset":4200,"ancestor_record_offset":1852,"inlinee":11323,"source_line":81,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_shuffle_ps
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4256,"parent_record_offset":4172,"ancestor_record_offset":1852,"inlinee":11324,"source_line":1016,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse::_mm_sub_ps
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse.rs
//     evidence={"module":1522,"record_offset":4320,"parent_record_offset":4128,"ancestor_record_offset":1852,"inlinee":11325,"source_line":57,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::abs
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":3228,"parent_record_offset":3152,"ancestor_record_offset":1852,"inlinee":10511,"source_line":1446,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\f32.rs
//     evidence={"module":1522,"record_offset":2432,"parent_record_offset":2408,"ancestor_record_offset":1852,"inlinee":6356,"source_line":1409,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1522,"record_offset":2728,"parent_record_offset":2696,"ancestor_record_offset":1852,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":4056,"parent_record_offset":4032,"ancestor_record_offset":1852,"inlinee":4704,"source_line":105,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\arith.rs
//     evidence={"module":1522,"record_offset":3972,"parent_record_offset":3948,"ancestor_record_offset":1852,"inlinee":4700,"source_line":344,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2856,"parent_record_offset":2832,"ancestor_record_offset":1852,"inlinee":4512,"source_line":607,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1522,"record_offset":2220,"parent_record_offset":2192,"ancestor_record_offset":1852,"inlinee":4197,"source_line":1017,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1522,"record_offset":2620,"parent_record_offset":2596,"ancestor_record_offset":1852,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":3720,"parent_record_offset":3696,"ancestor_record_offset":1852,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1522,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":1852,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2284,"parent_record_offset":2192,"ancestor_record_offset":1852,"inlinee":25150,"source_line":118,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2192,"parent_record_offset":2156,"ancestor_record_offset":1852,"inlinee":25148,"source_line":241,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator_subtype::impl$4::dims_wall_space::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs
//     evidence={"module":1522,"record_offset":2252,"parent_record_offset":2220,"ancestor_record_offset":1852,"inlinee":25160,"source_line":244,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::quat::Quat::mul_vec3a
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/quat.rs
//     evidence={"module":1522,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":1852,"inlinee":11319,"source_line":788,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::sse2::vec3a::Vec3A::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/sse2/vec3a.rs
//     evidence={"module":1522,"record_offset":4576,"parent_record_offset":4128,"ancestor_record_offset":1852,"inlinee":11329,"source_line":229,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":3920,"parent_record_offset":3892,"ancestor_record_offset":1852,"inlinee":4696,"source_line":693,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$24::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":3948,"parent_record_offset":3920,"ancestor_record_offset":1852,"inlinee":4698,"source_line":1198,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$34::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/f32/vec2.rs
//     evidence={"module":1522,"record_offset":4032,"parent_record_offset":3920,"ancestor_record_offset":1852,"inlinee":4702,"source_line":1283,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_in_x
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1522,"record_offset":4200,"parent_record_offset":4172,"ancestor_record_offset":1852,"inlinee":11322,"source_line":42,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] glam::sse2::dot3_into_m128
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.29.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.29.3/src/sse2.rs
//     evidence={"module":1522,"record_offset":4172,"parent_record_offset":4128,"ancestor_record_offset":1852,"inlinee":11321,"source_line":66,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":2556,"parent_record_offset":2528,"ancestor_record_offset":1852,"inlinee":71556,"source_line":553,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-decorator","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs
//     evidence={"module":1522,"record_offset":4788,"parent_record_offset":4760,"ancestor_record_offset":1852,"inlinee":71566,"source_line":633,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_pos_at_coord
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":1852,"inlinee":9575,"source_line":799,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":1852,"inlinee":4639,"source_line":1332,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1522,"record_offset":3780,"parent_record_offset":3644,"ancestor_record_offset":1852,"inlinee":4641,"source_line":1337,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_u::CurveUBoundless::clamp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1522,"record_offset":2408,"parent_record_offset":2376,"ancestor_record_offset":1852,"inlinee":24561,"source_line":207,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::height
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1522,"record_offset":2336,"parent_record_offset":2284,"ancestor_record_offset":1852,"inlinee":10294,"source_line":79,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::geometry::rectangle::Rectangle3d::width
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/rectangle.rs
//     evidence={"module":1522,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1852,"inlinee":10292,"source_line":84,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$0::lerp
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1522,"record_offset":3892,"parent_record_offset":3644,"ancestor_record_offset":1852,"inlinee":9576,"source_line":78,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   [pdb_inline_transitive] utils::wall_space::WallSpaceCoord::subtract
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1522,"record_offset":3200,"parent_record_offset":3152,"ancestor_record_offset":1852,"inlinee":25627,"source_line":284,"archive":"_pdb/B/modules/01522.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 546
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19EE6
// 首条位置: {"module":3051,"inlinee":106214}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3051,"record_offset":3756,"parent_record_offset":3736,"ancestor_record_offset":3736,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::decorator::impl$29::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator.rs
//     evidence={"module":3051,"record_offset":3780,"parent_record_offset":3756,"ancestor_record_offset":3736,"inlinee":4664,"source_line":155,"archive":"_pdb/B/modules/03051.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 553
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11784
// 首条位置: {"module":1522,"inlinee":71556}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 633
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1178E
// 首条位置: {"module":1522,"inlinee":71566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 920
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1178D
// 首条位置: {"module":1522,"inlinee":71565}
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
// 完整结构及原始类型记录: decorator_interaction_intent.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: decorator_interaction_intent.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: decorator_interaction_intent.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::BalconySnapResult::get
// 信息 ID: 2bb4c6b5ba09e41b2e0641a1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=4728; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
// 信息 ID: a1dfaecd1d3a902d1f4493b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::ui_move_decorator::ui_move_decorator @ 0x7a6c25 -> 0x127cf70 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::ui_systems::ui_place_decorator::ui_place_decorator @ 0x7ac507 -> 0x127cf70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$0
// 信息 ID: 921f801a24b6a91bf5734f92
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=9572; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$1
// 信息 ID: c77caeff1a7848f5c27de300
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1 <- system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::merge_proposal_roof_decorators::intersect_roof_decorators::closure_env$0<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$1> >; module=3053, inline_record=3532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$2
// 信息 ID: 2957f2265e4bb2d91130f9ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2 <- core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::option::Option::and_then<ref$<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env$2>; module=1522, inline_record=928; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$3
// 信息 ID: ffb3a7dea160b4c3a63b1e7e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=5260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure$4
// 信息 ID: 552a244666f1d404b5cbbc31
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=13452; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent
// 信息 ID: 4d39d1188279039655584279
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=1852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$0
// 信息 ID: cf20a8e4940513f0972e8175
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0 <- system_decorator::collision_detection::intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::CircularComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3051, inline_record=3736; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<tuple$<ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorInfo> > >,system_decorator::collision_detection::intersect_wall_decorators_impl::closure_env$0<utils::comparative_space::ZeroBoundedComparativeSpace,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env$0> >; module=3051, inline_record=4952; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$1
// 信息 ID: e6a017b9bebe073ac9b717cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=2556; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure$2
// 信息 ID: b5405827c5f56e4de147d452
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=4788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t
// 信息 ID: 50f8149d0c1d60a761c4ad5b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=5348; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$0
// 信息 ID: b1518c87c4d02101fca2b1ff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=5460; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=6332; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=7368; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$1
// 信息 ID: 4ac4a00f8cdc0eacdd7afaf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=7336; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=9268; 不是独立调用边。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=9304; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$2
// 信息 ID: 0249a60fd83f26a4e17463f5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=7288; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t::closure$3
// 信息 ID: 4d1fbc4e07c0109635615eaf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_decorator::ui_systems::decorator_interaction_intent::find_closest_norm_curve_t", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent; module=1522, inline_record=9056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_decorator::ui_systems::decorator_interaction_intent::snap_balcony_door
// 信息 ID: 27fa5e107fc57e19e62b307c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent @ 0x127e774 -> 0x127f0e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
