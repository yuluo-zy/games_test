// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/wall-constructor/src/plaster_systems.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 84aed4387af04433960051ba8a3e4bcb9c22088637a0124b263c7e29b83fc769
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: plaster_systems.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 372, 1226, 1257, 1264
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"PDB_registry_path_only","version":"0.14.1"}]
// use smallvec; // namespace candidate
//   package identities: [{"name":"smallvec","source":"PDB_registry_path_only","version":"1.13.2"}]
// use std; // namespace candidate
//   package identities: []
// use system_wall_constructor; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":372,"record_offset":3068,"rva":5165168,"va":5373874288}
// 编译实例 RVAs: 0x4ed070
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=4
//     target_aliases=9 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4261788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e864f8beff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373875303,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140628e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1291692","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8acb51300","size":5,"soft_fail":false,"target_va":5375168144,"va":5373876447,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8036825","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8d9a17a00","size":5,"soft_fail":false,"target_va":5381913760,"va":5373876930,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ca2140
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8080574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8be4c7b00","size":5,"soft_fail":false,"target_va":5381955904,"va":5373875325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ce2ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8343557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e805507f00","size":5,"soft_fail":false,"target_va":5382220480,"va":5373876918,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3369909","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8b56b3300","size":5,"soft_fail":false,"target_va":5377245296,"va":5373875382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t29095003","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e85bf4bb01","size":5,"soft_fail":false,"target_va":5402972080,"va":5373877072,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3430110","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8de563400","size":5,"soft_fail":false,"target_va":5377305552,"va":5373875437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29101571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8030ebc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373877112,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29101809","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8f10ebc01","size":5,"soft_fail":false,"target_va":5402978912,"va":5373877098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29101820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8fc0ebc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373877055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-3997","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e863f0ffff","size":5,"soft_fail":false,"target_va":5373873024,"va":5373877016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29105318","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8a61cbc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373874805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-1182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e862fbffff","size":5,"soft_fail":false,"target_va":5373873216,"va":5373874393,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29106188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e80c20bc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373874463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t690921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e8e98a0a00","size":5,"soft_fail":false,"target_va":5374567616,"va":5373876690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1559309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e80dcb1700","size":5,"soft_fail":false,"target_va":5375436032,"va":5373876718,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1292164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e884b71300","size":5,"soft_fail":false,"target_va":5375168608,"va":5373876439,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2481936","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e810df2500","size":5,"soft_fail":false,"target_va":5376356912,"va":5373874971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2485819","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e83bee2500","size":5,"soft_fail":false,"target_va":5376360560,"va":5373874736,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2481789","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e87dde2500","size":5,"soft_fail":false,"target_va":5376356144,"va":5373874350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7990663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373874288,"raw_hex":"e887ed7900","size":5,"soft_fail":false,"target_va":5381865424,"va":5373874756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":372,"record_offset":6820,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":5900,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":6448,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":372,"record_offset":6992,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":372,"record_offset":11004,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":11964,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":11928,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":10920,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":12972,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":4852,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":3264,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":3840,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":372,"record_offset":7952,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::remove
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":372,"record_offset":6876,"parent_record_offset":3068,"ancestor_record_offset":3068,"inlinee":7345,"source_line":1160,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":372,"record_offset":11220,"parent_record_offset":11068,"ancestor_record_offset":3068,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":372,"record_offset":6844,"parent_record_offset":6820,"ancestor_record_offset":3068,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":372,"record_offset":11068,"parent_record_offset":11028,"ancestor_record_offset":3068,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":5988,"parent_record_offset":5964,"ancestor_record_offset":3068,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":5964,"parent_record_offset":5924,"ancestor_record_offset":3068,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":6472,"parent_record_offset":6448,"ancestor_record_offset":3068,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":6780,"parent_record_offset":6756,"ancestor_record_offset":3068,"inlinee":10427,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":12872,"parent_record_offset":12040,"ancestor_record_offset":3068,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":11496,"parent_record_offset":11028,"ancestor_record_offset":3068,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":11520,"parent_record_offset":11496,"ancestor_record_offset":3068,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":372,"record_offset":11028,"parent_record_offset":11004,"ancestor_record_offset":3068,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":372,"record_offset":11824,"parent_record_offset":11800,"ancestor_record_offset":3068,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":11596,"parent_record_offset":11572,"ancestor_record_offset":3068,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":3068,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":3068,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":11732,"parent_record_offset":11708,"ancestor_record_offset":3068,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":11800,"parent_record_offset":11520,"ancestor_record_offset":3068,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":11548,"parent_record_offset":11520,"ancestor_record_offset":3068,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":11468,"parent_record_offset":11028,"ancestor_record_offset":3068,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":372,"record_offset":7028,"parent_record_offset":6992,"ancestor_record_offset":3068,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":372,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":3068,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":372,"record_offset":5924,"parent_record_offset":5900,"ancestor_record_offset":3068,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":372,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":3068,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":372,"record_offset":11284,"parent_record_offset":11252,"ancestor_record_offset":3068,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":372,"record_offset":7052,"parent_record_offset":7028,"ancestor_record_offset":3068,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5252,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5304,"parent_record_offset":5280,"ancestor_record_offset":3068,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5280,"parent_record_offset":5252,"ancestor_record_offset":3068,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5192,"parent_record_offset":5164,"ancestor_record_offset":3068,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5224,"parent_record_offset":5192,"ancestor_record_offset":3068,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5396,"parent_record_offset":5224,"ancestor_record_offset":3068,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":5508,"parent_record_offset":5484,"ancestor_record_offset":3068,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":3068,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":8676,"parent_record_offset":8648,"ancestor_record_offset":3068,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":372,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":3068,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":372,"record_offset":8296,"parent_record_offset":8272,"ancestor_record_offset":3068,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":372,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":3068,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":6528,"parent_record_offset":6504,"ancestor_record_offset":3068,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":6504,"parent_record_offset":6472,"ancestor_record_offset":3068,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":3068,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":372,"record_offset":5328,"parent_record_offset":5304,"ancestor_record_offset":3068,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":372,"record_offset":6036,"parent_record_offset":5988,"ancestor_record_offset":3068,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":372,"record_offset":3476,"parent_record_offset":3380,"ancestor_record_offset":3068,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":3068,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":11164,"parent_record_offset":11068,"ancestor_record_offset":3068,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":6552,"parent_record_offset":6528,"ancestor_record_offset":3068,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":5088,"parent_record_offset":5060,"ancestor_record_offset":3068,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":4436,"parent_record_offset":3976,"ancestor_record_offset":3068,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":5136,"parent_record_offset":4960,"ancestor_record_offset":3068,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":6584,"parent_record_offset":6504,"ancestor_record_offset":3068,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":372,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":3068,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":5460,"parent_record_offset":5164,"ancestor_record_offset":3068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":6340,"parent_record_offset":6316,"ancestor_record_offset":3068,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":3068,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":3068,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":3068,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":4212,"parent_record_offset":4188,"ancestor_record_offset":3068,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":4188,"parent_record_offset":4164,"ancestor_record_offset":3068,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":4396,"parent_record_offset":4364,"ancestor_record_offset":3068,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":5352,"parent_record_offset":5328,"ancestor_record_offset":3068,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":5860,"parent_record_offset":4924,"ancestor_record_offset":3068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":11440,"parent_record_offset":11028,"ancestor_record_offset":3068,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":3412,"parent_record_offset":3380,"ancestor_record_offset":3068,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":6292,"parent_record_offset":5988,"ancestor_record_offset":3068,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":3440,"parent_record_offset":3412,"ancestor_record_offset":3068,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":6164,"parent_record_offset":6140,"ancestor_record_offset":3068,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":6140,"parent_record_offset":6100,"ancestor_record_offset":3068,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":372,"record_offset":6316,"parent_record_offset":6292,"ancestor_record_offset":3068,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":6684,"parent_record_offset":6660,"ancestor_record_offset":3068,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":4164,"parent_record_offset":4140,"ancestor_record_offset":3068,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":4492,"parent_record_offset":4468,"ancestor_record_offset":3068,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":6660,"parent_record_offset":6608,"ancestor_record_offset":3068,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":4140,"parent_record_offset":4032,"ancestor_record_offset":3068,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":372,"record_offset":3380,"parent_record_offset":3352,"ancestor_record_offset":3068,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":372,"record_offset":3872,"parent_record_offset":3840,"ancestor_record_offset":3068,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":372,"record_offset":4004,"parent_record_offset":3976,"ancestor_record_offset":3068,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":372,"record_offset":3976,"parent_record_offset":3944,"ancestor_record_offset":3068,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":3920,"parent_record_offset":3896,"ancestor_record_offset":3068,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":3944,"parent_record_offset":3920,"ancestor_record_offset":3068,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":3896,"parent_record_offset":3872,"ancestor_record_offset":3068,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":4468,"parent_record_offset":3944,"ancestor_record_offset":3068,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3576,"parent_record_offset":3544,"ancestor_record_offset":3068,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3736,"parent_record_offset":3576,"ancestor_record_offset":3068,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3544,"parent_record_offset":3520,"ancestor_record_offset":3068,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3296,"parent_record_offset":3264,"ancestor_record_offset":3068,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3760,"parent_record_offset":3736,"ancestor_record_offset":3068,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3352,"parent_record_offset":3320,"ancestor_record_offset":3068,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":3320,"parent_record_offset":3296,"ancestor_record_offset":3068,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":4032,"parent_record_offset":4004,"ancestor_record_offset":3068,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":4280,"parent_record_offset":4032,"ancestor_record_offset":3068,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":4252,"parent_record_offset":4032,"ancestor_record_offset":3068,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":4084,"parent_record_offset":4032,"ancestor_record_offset":3068,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":372,"record_offset":8704,"parent_record_offset":8676,"ancestor_record_offset":3068,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":372,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":3068,"inlinee":21543,"source_line":440,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":11252,"parent_record_offset":11028,"ancestor_record_offset":3068,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":11336,"parent_record_offset":11312,"ancestor_record_offset":3068,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":11360,"parent_record_offset":11336,"ancestor_record_offset":3068,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":11312,"parent_record_offset":11284,"ancestor_record_offset":3068,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":372,"record_offset":8648,"parent_record_offset":8620,"ancestor_record_offset":3068,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":7976,"parent_record_offset":7952,"ancestor_record_offset":3068,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":8000,"parent_record_offset":7976,"ancestor_record_offset":3068,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":6900,"parent_record_offset":6876,"ancestor_record_offset":3068,"inlinee":7346,"source_line":1895,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::remove_entry
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":6924,"parent_record_offset":6900,"ancestor_record_offset":3068,"inlinee":7347,"source_line":1934,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":8620,"parent_record_offset":8480,"ancestor_record_offset":3068,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=4
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":6952,"parent_record_offset":6924,"ancestor_record_offset":3068,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":3068,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8508,"parent_record_offset":8480,"ancestor_record_offset":3068,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":9476,"parent_record_offset":9452,"ancestor_record_offset":3068,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8840,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":9452,"parent_record_offset":9424,"ancestor_record_offset":3068,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8084,"parent_record_offset":8060,"ancestor_record_offset":3068,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8060,"parent_record_offset":8000,"ancestor_record_offset":3068,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":3068,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":8360,"parent_record_offset":8328,"ancestor_record_offset":3068,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":8388,"parent_record_offset":8360,"ancestor_record_offset":3068,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":8448,"parent_record_offset":8328,"ancestor_record_offset":3068,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":8328,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8160,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":8480,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":8188,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":8272,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":8748,"parent_record_offset":8108,"ancestor_record_offset":3068,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":372,"record_offset":11756,"parent_record_offset":11732,"ancestor_record_offset":3068,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":4888,"parent_record_offset":4852,"ancestor_record_offset":3068,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":372,"record_offset":5000,"parent_record_offset":4960,"ancestor_record_offset":3068,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":5164,"parent_record_offset":4960,"ancestor_record_offset":3068,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":5432,"parent_record_offset":5164,"ancestor_record_offset":3068,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":372,"record_offset":5028,"parent_record_offset":5000,"ancestor_record_offset":3068,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":372,"record_offset":5060,"parent_record_offset":5028,"ancestor_record_offset":3068,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":372,"record_offset":4960,"parent_record_offset":4924,"ancestor_record_offset":3068,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":372,"record_offset":4924,"parent_record_offset":4888,"ancestor_record_offset":3068,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":10944,"parent_record_offset":10920,"ancestor_record_offset":3068,"inlinee":21541,"source_line":64,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::maintain_plaster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":11992,"parent_record_offset":11964,"ancestor_record_offset":3068,"inlinee":21544,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 64
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5425
// 首条位置: {"module":372,"inlinee":21541}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":372,"record_offset":10968,"parent_record_offset":10944,"ancestor_record_offset":10944,"inlinee":21543,"source_line":440,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::maintain_plaster::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5428
// 首条位置: {"module":372,"inlinee":21544}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":372,"record_offset":12016,"parent_record_offset":11992,"ancestor_record_offset":11992,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":372,"record_offset":12228,"parent_record_offset":12084,"ancestor_record_offset":11992,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":372,"record_offset":12084,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":12872,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":12504,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":12528,"parent_record_offset":12504,"ancestor_record_offset":11992,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":372,"record_offset":12040,"parent_record_offset":12016,"ancestor_record_offset":11992,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":372,"record_offset":12832,"parent_record_offset":12808,"ancestor_record_offset":11992,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":12604,"parent_record_offset":12580,"ancestor_record_offset":11992,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":12580,"parent_record_offset":12556,"ancestor_record_offset":11992,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":12652,"parent_record_offset":12628,"ancestor_record_offset":11992,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":12740,"parent_record_offset":12716,"ancestor_record_offset":11992,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":12808,"parent_record_offset":12528,"ancestor_record_offset":11992,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":12556,"parent_record_offset":12528,"ancestor_record_offset":11992,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":12476,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":372,"record_offset":12896,"parent_record_offset":12872,"ancestor_record_offset":11992,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":372,"record_offset":12292,"parent_record_offset":12260,"ancestor_record_offset":11992,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":12196,"parent_record_offset":12172,"ancestor_record_offset":11992,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":12172,"parent_record_offset":12084,"ancestor_record_offset":11992,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":12676,"parent_record_offset":12652,"ancestor_record_offset":11992,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":12628,"parent_record_offset":12604,"ancestor_record_offset":11992,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":12400,"parent_record_offset":12368,"ancestor_record_offset":11992,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":12448,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":12116,"parent_record_offset":12084,"ancestor_record_offset":11992,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":12140,"parent_record_offset":12116,"ancestor_record_offset":11992,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":12260,"parent_record_offset":12040,"ancestor_record_offset":11992,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":11992,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":11992,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":12320,"parent_record_offset":12292,"ancestor_record_offset":11992,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":372,"record_offset":12764,"parent_record_offset":12740,"ancestor_record_offset":11992,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster
// 记录形式: compiled_procedure; instances=1
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":372,"record_offset":13004,"rva":5168288,"va":5373877408}
// 编译实例 RVAs: 0x4edca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dccd0
//     owner=unresolved_namespace; count=3
//     target_aliases=9 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4265173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e82bebbeff","size":5,"soft_fail":false,"target_va":5369613520,"va":5373878688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140596a60
//     owner=unresolved_namespace; count=1
//     target_aliases=8 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t689360","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8d0840a00","size":5,"soft_fail":false,"target_va":5374569056,"va":5373879691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140628e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1289100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e88cab1300","size":5,"soft_fail":false,"target_va":5375168144,"va":5373879039,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c91b30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8008983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e817357a00","size":5,"soft_fail":false,"target_va":5381888816,"va":5373879828,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ce2ac0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t8340943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8cf457f00","size":5,"soft_fail":false,"target_va":5382220480,"va":5373879532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::spawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8199113","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8c91b7d00","size":5,"soft_fail":false,"target_va":5382078912,"va":5373879794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29098720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8e002bc01","size":5,"soft_fail":false,"target_va":5402978688,"va":5373879963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t29098940","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8bc03bc01","size":5,"soft_fail":false,"target_va":5402978880,"va":5373879935,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-7098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e846e4ffff","size":5,"soft_fail":false,"target_va":5373873024,"va":5373880117,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t29102115","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e82310bc01","size":5,"soft_fail":false,"target_va":5402980128,"va":5373878008,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8faeeffff","size":5,"soft_fail":false,"target_va":5373873216,"va":5373877569,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t29103012","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8a413bc01","size":5,"soft_fail":false,"target_va":5402980656,"va":5373877639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::mesh_library::AssetMeshLibrary::add
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t686436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e864790a00","size":5,"soft_fail":false,"target_va":5374566080,"va":5373879639,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1812744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e808a91b00","size":5,"soft_fail":false,"target_va":5375692416,"va":5373879667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::components::transient_mesh::TransientMesh::new_with_registry
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8365413","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e865a57f00","size":5,"soft_fail":false,"target_va":5382245120,"va":5373879702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1556734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8fec01700","size":5,"soft_fail":false,"target_va":5375436032,"va":5373879293,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::wall_holes::WallHoleStorage::as_finalized
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1289572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e864ad1300","size":5,"soft_fail":false,"target_va":5375168608,"va":5373879031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2478726","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e886d22500","size":5,"soft_fail":false,"target_va":5376356912,"va":5373878181,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t2482616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e8b8e12500","size":5,"soft_fail":false,"target_va":5376360560,"va":5373877939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2478620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e81cd22500","size":5,"soft_fail":false,"target_va":5376356144,"va":5373877519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t7987460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373877408,"raw_hex":"e804e17900","size":5,"soft_fail":false,"target_va":5381865424,"va":5373877959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-6904","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5373877408,"raw_hex":"e908e5ffff","size":5,"soft_fail":false,"target_va":5373873024,"va":5373879923,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":372,"record_offset":20856,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":15828,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":16380,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":19784,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":18780,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":20820,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":372,"record_offset":20792,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":21549,"source_line":31,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":14780,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":13192,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":7284,"source_line":626,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":13768,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":372,"record_offset":18864,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":372,"record_offset":16732,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":7329,"source_line":877,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":372,"record_offset":20912,"parent_record_offset":13004,"ancestor_record_offset":13004,"inlinee":8309,"source_line":1105,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":372,"record_offset":20048,"parent_record_offset":19904,"ancestor_record_offset":13004,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":372,"record_offset":20880,"parent_record_offset":20856,"ancestor_record_offset":13004,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":372,"record_offset":19904,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":15916,"parent_record_offset":15892,"ancestor_record_offset":13004,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":15892,"parent_record_offset":15852,"ancestor_record_offset":13004,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":372,"record_offset":16412,"parent_record_offset":16380,"ancestor_record_offset":13004,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20692,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20324,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":13004,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":372,"record_offset":19860,"parent_record_offset":19836,"ancestor_record_offset":13004,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":372,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":13004,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":13004,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":13004,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":13004,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":13004,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20628,"parent_record_offset":20348,"ancestor_record_offset":13004,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20376,"parent_record_offset":20348,"ancestor_record_offset":13004,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20296,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":372,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":13004,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":372,"record_offset":15852,"parent_record_offset":15828,"ancestor_record_offset":13004,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":372,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":13004,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":372,"record_offset":20112,"parent_record_offset":20080,"ancestor_record_offset":13004,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15180,"parent_record_offset":15152,"ancestor_record_offset":13004,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":13004,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":13004,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15120,"parent_record_offset":15092,"ancestor_record_offset":13004,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15152,"parent_record_offset":15120,"ancestor_record_offset":13004,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15324,"parent_record_offset":15152,"ancestor_record_offset":13004,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":372,"record_offset":15436,"parent_record_offset":15412,"ancestor_record_offset":13004,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":13004,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":17552,"parent_record_offset":17524,"ancestor_record_offset":13004,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":372,"record_offset":17004,"parent_record_offset":16980,"ancestor_record_offset":13004,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":372,"record_offset":17088,"parent_record_offset":17064,"ancestor_record_offset":13004,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":372,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":13004,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":16488,"parent_record_offset":16456,"ancestor_record_offset":13004,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":16456,"parent_record_offset":16412,"ancestor_record_offset":13004,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":372,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":13004,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":372,"record_offset":15256,"parent_record_offset":15232,"ancestor_record_offset":13004,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":372,"record_offset":15960,"parent_record_offset":15916,"ancestor_record_offset":13004,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":372,"record_offset":13404,"parent_record_offset":13308,"ancestor_record_offset":13004,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":17204,"parent_record_offset":17180,"ancestor_record_offset":13004,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":19992,"parent_record_offset":19904,"ancestor_record_offset":13004,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":16516,"parent_record_offset":16488,"ancestor_record_offset":13004,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":15016,"parent_record_offset":14988,"ancestor_record_offset":13004,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":14364,"parent_record_offset":13904,"ancestor_record_offset":13004,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":15064,"parent_record_offset":14888,"ancestor_record_offset":13004,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":16552,"parent_record_offset":16456,"ancestor_record_offset":13004,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":372,"record_offset":13716,"parent_record_offset":13688,"ancestor_record_offset":13004,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":15388,"parent_record_offset":15092,"ancestor_record_offset":13004,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":16272,"parent_record_offset":16248,"ancestor_record_offset":13004,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":17456,"parent_record_offset":17432,"ancestor_record_offset":13004,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":17348,"parent_record_offset":17324,"ancestor_record_offset":13004,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":16676,"parent_record_offset":16652,"ancestor_record_offset":13004,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":14140,"parent_record_offset":14116,"ancestor_record_offset":13004,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":14116,"parent_record_offset":14092,"ancestor_record_offset":13004,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":372,"record_offset":14324,"parent_record_offset":14292,"ancestor_record_offset":13004,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":15280,"parent_record_offset":15256,"ancestor_record_offset":13004,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":15788,"parent_record_offset":14852,"ancestor_record_offset":13004,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":20268,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":13340,"parent_record_offset":13308,"ancestor_record_offset":13004,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":16224,"parent_record_offset":15916,"ancestor_record_offset":13004,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":13368,"parent_record_offset":13340,"ancestor_record_offset":13004,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":16084,"parent_record_offset":16060,"ancestor_record_offset":13004,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":16060,"parent_record_offset":16024,"ancestor_record_offset":13004,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":372,"record_offset":16248,"parent_record_offset":16224,"ancestor_record_offset":13004,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":16652,"parent_record_offset":16628,"ancestor_record_offset":13004,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":14092,"parent_record_offset":14068,"ancestor_record_offset":13004,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":14420,"parent_record_offset":14396,"ancestor_record_offset":13004,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":16628,"parent_record_offset":16576,"ancestor_record_offset":13004,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":372,"record_offset":14068,"parent_record_offset":13960,"ancestor_record_offset":13004,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":372,"record_offset":13308,"parent_record_offset":13280,"ancestor_record_offset":13004,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":372,"record_offset":13800,"parent_record_offset":13768,"ancestor_record_offset":13004,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":372,"record_offset":13932,"parent_record_offset":13904,"ancestor_record_offset":13004,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":372,"record_offset":13904,"parent_record_offset":13872,"ancestor_record_offset":13004,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":13004,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13004,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":13004,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":372,"record_offset":14396,"parent_record_offset":13872,"ancestor_record_offset":13004,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13504,"parent_record_offset":13472,"ancestor_record_offset":13004,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13664,"parent_record_offset":13504,"ancestor_record_offset":13004,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13472,"parent_record_offset":13448,"ancestor_record_offset":13004,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13224,"parent_record_offset":13192,"ancestor_record_offset":13004,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13688,"parent_record_offset":13664,"ancestor_record_offset":13004,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13280,"parent_record_offset":13248,"ancestor_record_offset":13004,"inlinee":6112,"source_line":333,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$9::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":372,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":13004,"inlinee":6111,"source_line":363,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":13960,"parent_record_offset":13932,"ancestor_record_offset":13004,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":14208,"parent_record_offset":13960,"ancestor_record_offset":13004,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":14180,"parent_record_offset":13960,"ancestor_record_offset":13004,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":372,"record_offset":14012,"parent_record_offset":13960,"ancestor_record_offset":13004,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::impl$9::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/mod.rs
//     evidence={"module":372,"record_offset":17580,"parent_record_offset":17552,"ancestor_record_offset":13004,"inlinee":8043,"source_line":65,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":372,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":13004,"inlinee":21543,"source_line":440,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20080,"parent_record_offset":19860,"ancestor_record_offset":13004,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20164,"parent_record_offset":20140,"ancestor_record_offset":13004,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20188,"parent_record_offset":20164,"ancestor_record_offset":13004,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20140,"parent_record_offset":20112,"ancestor_record_offset":13004,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":372,"record_offset":17524,"parent_record_offset":17496,"ancestor_record_offset":13004,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":18888,"parent_record_offset":18864,"ancestor_record_offset":13004,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":16760,"parent_record_offset":16732,"ancestor_record_offset":13004,"inlinee":5561,"source_line":1308,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":16788,"parent_record_offset":16760,"ancestor_record_offset":13004,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":17496,"parent_record_offset":17272,"ancestor_record_offset":13004,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":372,"record_offset":16824,"parent_record_offset":16788,"ancestor_record_offset":13004,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17408,"parent_record_offset":17384,"ancestor_record_offset":13004,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17384,"parent_record_offset":17272,"ancestor_record_offset":13004,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17324,"parent_record_offset":17300,"ancestor_record_offset":13004,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17716,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17300,"parent_record_offset":17272,"ancestor_record_offset":13004,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":16876,"parent_record_offset":16852,"ancestor_record_offset":13004,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":16852,"parent_record_offset":16788,"ancestor_record_offset":13004,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":16900,"parent_record_offset":16876,"ancestor_record_offset":13004,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":17152,"parent_record_offset":17120,"ancestor_record_offset":13004,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":17180,"parent_record_offset":17152,"ancestor_record_offset":13004,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":17240,"parent_record_offset":17120,"ancestor_record_offset":13004,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":372,"record_offset":17120,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":16952,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":372,"record_offset":17272,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":16980,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":17064,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":372,"record_offset":17624,"parent_record_offset":16900,"ancestor_record_offset":13004,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":372,"record_offset":20584,"parent_record_offset":20560,"ancestor_record_offset":13004,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":14816,"parent_record_offset":14780,"ancestor_record_offset":13004,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":372,"record_offset":14928,"parent_record_offset":14888,"ancestor_record_offset":13004,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":15092,"parent_record_offset":14888,"ancestor_record_offset":13004,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":372,"record_offset":15360,"parent_record_offset":15092,"ancestor_record_offset":13004,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":372,"record_offset":14956,"parent_record_offset":14928,"ancestor_record_offset":13004,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":372,"record_offset":14988,"parent_record_offset":14956,"ancestor_record_offset":13004,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":372,"record_offset":14888,"parent_record_offset":14852,"ancestor_record_offset":13004,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":372,"record_offset":14852,"parent_record_offset":14816,"ancestor_record_offset":13004,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::create_plaster::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":18804,"parent_record_offset":18780,"ancestor_record_offset":13004,"inlinee":21546,"source_line":112,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::create_plaster::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":19812,"parent_record_offset":19784,"ancestor_record_offset":13004,"inlinee":21547,"source_line":120,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x542A
// 首条位置: {"module":372,"inlinee":21546}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::walls::wall_holes::FinalizedHoleStorage::holes_no_duplicates
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":372,"record_offset":18828,"parent_record_offset":18804,"ancestor_record_offset":18804,"inlinee":21543,"source_line":440,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::create_plaster::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x542B
// 首条位置: {"module":372,"inlinee":21547}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":372,"record_offset":19836,"parent_record_offset":19812,"ancestor_record_offset":19812,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":372,"record_offset":20048,"parent_record_offset":19904,"ancestor_record_offset":19812,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":372,"record_offset":19904,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20692,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20324,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":372,"record_offset":20348,"parent_record_offset":20324,"ancestor_record_offset":19812,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":372,"record_offset":19860,"parent_record_offset":19836,"ancestor_record_offset":19812,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":372,"record_offset":20652,"parent_record_offset":20628,"ancestor_record_offset":19812,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20424,"parent_record_offset":20400,"ancestor_record_offset":19812,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20400,"parent_record_offset":20376,"ancestor_record_offset":19812,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20472,"parent_record_offset":20448,"ancestor_record_offset":19812,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":372,"record_offset":20560,"parent_record_offset":20536,"ancestor_record_offset":19812,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20628,"parent_record_offset":20348,"ancestor_record_offset":19812,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20376,"parent_record_offset":20348,"ancestor_record_offset":19812,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":372,"record_offset":20296,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":372,"record_offset":20716,"parent_record_offset":20692,"ancestor_record_offset":19812,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":372,"record_offset":20112,"parent_record_offset":20080,"ancestor_record_offset":19812,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":20016,"parent_record_offset":19992,"ancestor_record_offset":19812,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":372,"record_offset":19992,"parent_record_offset":19904,"ancestor_record_offset":19812,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":20496,"parent_record_offset":20472,"ancestor_record_offset":19812,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":20448,"parent_record_offset":20424,"ancestor_record_offset":19812,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":20220,"parent_record_offset":20188,"ancestor_record_offset":19812,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":372,"record_offset":20268,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":372,"record_offset":19936,"parent_record_offset":19904,"ancestor_record_offset":19812,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":372,"record_offset":19960,"parent_record_offset":19936,"ancestor_record_offset":19812,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20080,"parent_record_offset":19860,"ancestor_record_offset":19812,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20164,"parent_record_offset":20140,"ancestor_record_offset":19812,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20188,"parent_record_offset":20164,"ancestor_record_offset":19812,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":372,"record_offset":20140,"parent_record_offset":20112,"ancestor_record_offset":19812,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":372,"record_offset":20584,"parent_record_offset":20560,"ancestor_record_offset":19812,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 149
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1257,"record_offset":1568,"rva":13511360,"va":5382220480}
// 编译实例 RVAs: 0xce2ac0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140002d20
//     owner=unresolved_namespace; count=1
//     target_aliases=1889 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-13503196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e824f531ff","size":5,"soft_fail":false,"target_va":5368720672,"va":5382223863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9471042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8be7b6fff","size":5,"soft_fail":false,"target_va":5372749696,"va":5382220733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=14
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9479283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e88d5b6fff","size":5,"soft_fail":false,"target_va":5372749712,"va":5382228990,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407da760
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5280363","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8956dafff","size":5,"soft_fail":false,"target_va":5376943968,"va":5382224326,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c97ca0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-317689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e80727fbff","size":5,"soft_fail":false,"target_va":5381913760,"va":5382231444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140cc7340
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-116007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8d93afeff","size":5,"soft_fail":false,"target_va":5382107968,"va":5382223970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177d260
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11109525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e89584a900","size":5,"soft_fail":false,"target_va":5393338976,"va":5382229446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177d6d0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11110566","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8a688a900","size":5,"soft_fail":false,"target_va":5393340112,"va":5382229541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177db40
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t11112001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8418ea900","size":5,"soft_fail":false,"target_va":5393341248,"va":5382229242,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144cf0
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21368555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8eb0e4601","size":5,"soft_fail":false,"target_va":5403593968,"va":5382225408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144d80
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21368032","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8e00c4601","size":5,"soft_fail":false,"target_va":5403594112,"va":5382226075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t20747353","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e859943c01","size":5,"soft_fail":false,"target_va":5402978144,"va":5382230786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20698708","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e854d63b01","size":5,"soft_fail":false,"target_va":5402919600,"va":5382220887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-4703646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8623ab8ff","size":5,"soft_fail":false,"target_va":5377527120,"va":5382230761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20748262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8e6973c01","size":5,"soft_fail":false,"target_va":5402978688,"va":5382230421,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20748671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e87f993c01","size":5,"soft_fail":false,"target_va":5402978880,"va":5382230204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t20749018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8da9a3c01","size":5,"soft_fail":false,"target_va":5402979152,"va":5382230129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=11
//     evidence={"assembly":"\tcallq\t20749443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8839c3c01","size":5,"soft_fail":false,"target_va":5402979344,"va":5382229896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t20748564","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e814993c01","size":5,"soft_fail":false,"target_va":5402978944,"va":5382230375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<core::option::Option<puffin::ProfilerScope> > >
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-2751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e841f5ffff","size":5,"soft_fail":false,"target_va":5382220288,"va":5382223034,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20749608","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8289d3c01","size":5,"soft_fail":false,"target_va":5402980128,"va":5382230515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t20749950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e87e9e3c01","size":5,"soft_fail":false,"target_va":5402980656,"va":5382230701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2338327","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e817ae2300","size":5,"soft_fail":false,"target_va":5384567376,"va":5382229044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::set_indices
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2338439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e887ae2300","size":5,"soft_fail":false,"target_va":5384567584,"va":5382229140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_flat
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6457822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e822769dff","size":5,"soft_fail":false,"target_va":5375763552,"va":5382221369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5866632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8787ba6ff","size":5,"soft_fail":false,"target_va":5376356912,"va":5382223539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-5862792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8788aa6ff","size":5,"soft_fail":false,"target_va":5376360560,"va":5382223347,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-5867047","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8d979a6ff","size":5,"soft_fail":false,"target_va":5376356144,"va":5382223186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::thread_local::fast_local::Key$LT$T$GT$::try_initialize::h5f74f2ca8f801a08 (.llvm.9829343827527839789)
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-357944","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8c889faff","size":5,"soft_fail":false,"target_va":5381865424,"va":5382223363,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] system_wall_constructor::plaster_systems::split_wall_into_sides
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-8341237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e80bb980ff","size":5,"soft_fail":false,"target_va":5373880160,"va":5382221392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::center
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2199348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8348f2100","size":5,"soft_fail":false,"target_va":5384420192,"va":5382220839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb2::half_dims
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2199382","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8568f2100","size":5,"soft_fail":false,"target_va":5384420240,"va":5382220853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::circle::Circle2d::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4256167","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e8590ebfff","size":5,"soft_fail":false,"target_va":5377968384,"va":5382224546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_normal_at_u
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4266616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e888e5beff","size":5,"soft_fail":false,"target_va":5377957680,"va":5382224291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::to_world_space
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t28958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e81e710000","size":5,"soft_fail":false,"target_va":5382256592,"va":5382227629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::wall_space::WallSpaceCoord::with_y_offset
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t32671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382220480,"raw_hex":"e89f7f0000","size":5,"soft_fail":false,"target_va":5382257600,"va":5382224924,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":5140,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":18492,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":17164,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":12892,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1257,"record_offset":16808,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":10688,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1257,"record_offset":7128,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":17068,"source_line":3154,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1257,"record_offset":4500,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":9992,"source_line":3573,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":4344,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":2376,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":7872,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":19152,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":12780,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1257,"record_offset":7788,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":12948,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_enclosed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1257,"record_offset":4308,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21382,"source_line":101,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":1257,"record_offset":2300,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::as_ivec2
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13716,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":59452,"source_line":868,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13632,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":24393,"source_line":600,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":12568,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":12540,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":14690,"source_line":814,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13408,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$14::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13464,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":17347,"source_line":1021,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13296,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$19::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13352,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":45882,"source_line":1070,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$7::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":13576,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":17349,"source_line":953,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":17504,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$9::mul
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":17228,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":14672,"source_line":1048,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":1257,"record_offset":10772,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1257,"record_offset":9612,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":7309,"source_line":589,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1257,"record_offset":8592,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":7291,"source_line":659,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$17::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":4404,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":46937,"source_line":1846,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":8452,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] smallvec::impl$49::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":8292,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":34978,"source_line":2290,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::mul_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":14192,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":24388,"source_line":292,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::sd_box
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":14252,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21566,"source_line":766,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":17028,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":59974,"source_line":774,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_unchecked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":17596,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":59980,"source_line":785,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_coord_at_wall_space
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":10912,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":59684,"source_line":954,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::get_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":12040,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21578,"source_line":637,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::Curve::length_ws
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":2348,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21590,"source_line":145,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":1257,"record_offset":11832,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::impl$12::sample_at_coord
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":11200,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21356,"source_line":1168,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1257,"record_offset":17820,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] utils::wall_space::WallSpaceCoord::as_vec2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/wall_space.rs
//     evidence={"module":1257,"record_offset":17136,"parent_record_offset":1568,"ancestor_record_offset":1568,"inlinee":26222,"source_line":146,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":1568,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":1568,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":1568,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2824,"parent_record_offset":2728,"ancestor_record_offset":1568,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4228,"parent_record_offset":4128,"ancestor_record_offset":1568,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":1568,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3672,"parent_record_offset":3632,"ancestor_record_offset":1568,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":1568,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":1568,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2692,"parent_record_offset":2652,"ancestor_record_offset":1568,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2944,"parent_record_offset":2692,"ancestor_record_offset":1568,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":1568,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3632,"parent_record_offset":3176,"ancestor_record_offset":1568,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":1568,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3576,"parent_record_offset":3176,"ancestor_record_offset":1568,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3704,"parent_record_offset":3176,"ancestor_record_offset":1568,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2612,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2652,"parent_record_offset":2612,"ancestor_record_offset":1568,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":12100,"parent_record_offset":12068,"ancestor_record_offset":1568,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3128,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1257,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":1568,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2768,"parent_record_offset":2728,"ancestor_record_offset":1568,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":1568,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\iter.rs
//     evidence={"module":1257,"record_offset":16844,"parent_record_offset":16808,"ancestor_record_offset":1568,"inlinee":6222,"source_line":239,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10040,"parent_record_offset":10012,"ancestor_record_offset":1568,"inlinee":4785,"source_line":1782,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10092,"parent_record_offset":10068,"ancestor_record_offset":1568,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10068,"parent_record_offset":10040,"ancestor_record_offset":1568,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":9980,"parent_record_offset":9948,"ancestor_record_offset":1568,"inlinee":4782,"source_line":1058,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10012,"parent_record_offset":9980,"ancestor_record_offset":1568,"inlinee":4783,"source_line":1090,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10184,"parent_record_offset":10012,"ancestor_record_offset":1568,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1257,"record_offset":10296,"parent_record_offset":10272,"ancestor_record_offset":1568,"inlinee":4787,"source_line":1773,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":11916,"parent_record_offset":11892,"ancestor_record_offset":1568,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":11940,"parent_record_offset":11916,"ancestor_record_offset":1568,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":1568,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$72::lt
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":13260,"parent_record_offset":13080,"ancestor_record_offset":1568,"inlinee":10908,"source_line":1553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":11980,"parent_record_offset":11832,"ancestor_record_offset":1568,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":11112,"parent_record_offset":10912,"ancestor_record_offset":1568,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":14648,"parent_record_offset":14252,"ancestor_record_offset":1568,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1257,"record_offset":12008,"parent_record_offset":11832,"ancestor_record_offset":1568,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":1568,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":1568,"inlinee":18213,"source_line":116,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":4572,"parent_record_offset":4536,"ancestor_record_offset":1568,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":3220,"parent_record_offset":3176,"ancestor_record_offset":1568,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":2988,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":4708,"parent_record_offset":4608,"ancestor_record_offset":1568,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$40::forward_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":13144,"parent_record_offset":13080,"ancestor_record_offset":1568,"inlinee":10910,"source_line":191,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$43::forward_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":18128,"parent_record_offset":18052,"ancestor_record_offset":1568,"inlinee":4620,"source_line":207,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":1568,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3744,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1257,"record_offset":4536,"parent_record_offset":4500,"ancestor_record_offset":1568,"inlinee":13759,"source_line":49,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::accum::impl$48::sum::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\accum.rs
//     evidence={"module":1257,"record_offset":4792,"parent_record_offset":4708,"ancestor_record_offset":1568,"inlinee":13764,"source_line":53,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::max_by
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1257,"record_offset":7164,"parent_record_offset":7128,"ancestor_record_offset":1568,"inlinee":17069,"source_line":3248,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::reduce
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1257,"record_offset":7200,"parent_record_offset":7164,"ancestor_record_offset":1568,"inlinee":8710,"source_line":2618,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$0::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1257,"record_offset":19200,"parent_record_offset":19176,"ancestor_record_offset":1568,"inlinee":6886,"source_line":4105,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1257,"record_offset":19316,"parent_record_offset":19288,"ancestor_record_offset":1568,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1257,"record_offset":10116,"parent_record_offset":10092,"ancestor_record_offset":1568,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1257,"record_offset":4836,"parent_record_offset":4608,"ancestor_record_offset":1568,"inlinee":4621,"source_line":520,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1257,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":1568,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::checked_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1257,"record_offset":13168,"parent_record_offset":13144,"ancestor_record_offset":1568,"inlinee":10911,"source_line":534,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1257,"record_offset":13216,"parent_record_offset":13192,"ancestor_record_offset":1568,"inlinee":10913,"source_line":2138,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$2::overflowing_add_unsigned
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\int_macros.rs
//     evidence={"module":1257,"record_offset":13192,"parent_record_offset":13168,"ancestor_record_offset":1568,"inlinee":10912,"source_line":2221,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_utf8_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\mod.rs
//     evidence={"module":1257,"record_offset":9476,"parent_record_offset":9384,"ancestor_record_offset":1568,"inlinee":6114,"source_line":1101,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1257,"record_offset":10824,"parent_record_offset":10772,"ancestor_record_offset":1568,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1257,"record_offset":10880,"parent_record_offset":10772,"ancestor_record_offset":1568,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1257,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":1568,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":1568,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":1568,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::IndexRange::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1257,"record_offset":16904,"parent_record_offset":16872,"ancestor_record_offset":1568,"inlinee":6100,"source_line":54,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\index_range.rs
//     evidence={"module":1257,"record_offset":16872,"parent_record_offset":16844,"ancestor_record_offset":1568,"inlinee":6069,"source_line":118,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1257,"record_offset":11892,"parent_record_offset":11868,"ancestor_record_offset":1568,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":1257,"record_offset":11868,"parent_record_offset":11832,"ancestor_record_offset":1568,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":9872,"parent_record_offset":9844,"ancestor_record_offset":1568,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":3484,"parent_record_offset":3220,"ancestor_record_offset":1568,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":9920,"parent_record_offset":9744,"ancestor_record_offset":1568,"inlinee":7261,"source_line":1206,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":11012,"parent_record_offset":10980,"ancestor_record_offset":1568,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1257,"record_offset":9560,"parent_record_offset":9532,"ancestor_record_offset":1568,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1257,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":1568,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":4076,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":1568,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":7340,"parent_record_offset":7312,"ancestor_record_offset":1568,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":8968,"parent_record_offset":8944,"ancestor_record_offset":1568,"inlinee":7305,"source_line":531,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":1568,"inlinee":7304,"source_line":709,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":1568,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3436,"parent_record_offset":3392,"ancestor_record_offset":1568,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":1568,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":3896,"parent_record_offset":3840,"ancestor_record_offset":1568,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1257,"record_offset":10648,"parent_record_offset":9700,"ancestor_record_offset":1568,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1257,"record_offset":9420,"parent_record_offset":9384,"ancestor_record_offset":1568,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1257,"record_offset":11236,"parent_record_offset":11200,"ancestor_record_offset":1568,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1257,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":1568,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1257,"record_offset":11264,"parent_record_offset":11236,"ancestor_record_offset":1568,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1257,"record_offset":7812,"parent_record_offset":7788,"ancestor_record_offset":1568,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":7312,"parent_record_offset":7276,"ancestor_record_offset":1568,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":8920,"parent_record_offset":8896,"ancestor_record_offset":1568,"inlinee":7303,"source_line":119,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::any
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":1568,"inlinee":4966,"source_line":279,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":1568,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":4608,"parent_record_offset":4572,"ancestor_record_offset":1568,"inlinee":4668,"source_line":212,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3392,"parent_record_offset":3304,"ancestor_record_offset":1568,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":1568,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next_back
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":8896,"parent_record_offset":8788,"ancestor_record_offset":1568,"inlinee":7302,"source_line":392,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1257,"record_offset":9384,"parent_record_offset":9356,"ancestor_record_offset":1568,"inlinee":6113,"source_line":217,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1257,"record_offset":8628,"parent_record_offset":8592,"ancestor_record_offset":1568,"inlinee":7292,"source_line":1310,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1257,"record_offset":8760,"parent_record_offset":8732,"ancestor_record_offset":1568,"inlinee":7272,"source_line":131,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\iter.rs
//     evidence={"module":1257,"record_offset":8732,"parent_record_offset":8700,"ancestor_record_offset":1568,"inlinee":7300,"source_line":221,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1257,"record_offset":8676,"parent_record_offset":8652,"ancestor_record_offset":1568,"inlinee":7296,"source_line":301,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1257,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":1568,"inlinee":7298,"source_line":690,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1257,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":1568,"inlinee":7294,"source_line":779,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1257,"record_offset":9224,"parent_record_offset":8700,"ancestor_record_offset":1568,"inlinee":7307,"source_line":640,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1257,"record_offset":9356,"parent_record_offset":9324,"ancestor_record_offset":1568,"inlinee":7287,"source_line":402,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1257,"record_offset":9508,"parent_record_offset":9356,"ancestor_record_offset":1568,"inlinee":7288,"source_line":422,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1257,"record_offset":9324,"parent_record_offset":9296,"ancestor_record_offset":1568,"inlinee":7286,"source_line":434,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1257,"record_offset":9296,"parent_record_offset":8592,"ancestor_record_offset":1568,"inlinee":6109,"source_line":60,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\traits.rs
//     evidence={"module":1257,"record_offset":9532,"parent_record_offset":9508,"ancestor_record_offset":1568,"inlinee":7290,"source_line":192,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1257,"record_offset":8788,"parent_record_offset":8760,"ancestor_record_offset":1568,"inlinee":7273,"source_line":79,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1257,"record_offset":9036,"parent_record_offset":8788,"ancestor_record_offset":1568,"inlinee":4936,"source_line":17,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1257,"record_offset":9008,"parent_record_offset":8788,"ancestor_record_offset":1568,"inlinee":4935,"source_line":11,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\validations.rs
//     evidence={"module":1257,"record_offset":8840,"parent_record_offset":8788,"ancestor_record_offset":1568,"inlinee":7274,"source_line":24,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1257,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":1568,"inlinee":59594,"source_line":126,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1257,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":1568,"inlinee":21561,"source_line":151,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::ceil
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1257,"record_offset":13656,"parent_record_offset":13632,"ancestor_record_offset":1568,"inlinee":24394,"source_line":212,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14288,"parent_record_offset":14252,"ancestor_record_offset":1568,"inlinee":21559,"source_line":311,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":12668,"parent_record_offset":12632,"ancestor_record_offset":1568,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":12632,"parent_record_offset":12592,"ancestor_record_offset":1568,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":12592,"parent_record_offset":12568,"ancestor_record_offset":1568,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14436,"parent_record_offset":14252,"ancestor_record_offset":1568,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14372,"parent_record_offset":14252,"ancestor_record_offset":1568,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":12400,"parent_record_offset":12368,"ancestor_record_offset":1568,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":12368,"parent_record_offset":12336,"ancestor_record_offset":1568,"inlinee":21582,"source_line":436,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":12336,"parent_record_offset":12312,"ancestor_record_offset":1568,"inlinee":21581,"source_line":455,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":12312,"parent_record_offset":12288,"ancestor_record_offset":1568,"inlinee":21580,"source_line":508,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":1257,"record_offset":12196,"parent_record_offset":12068,"ancestor_record_offset":1568,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1257,"record_offset":9656,"parent_record_offset":9612,"ancestor_record_offset":1568,"inlinee":7259,"source_line":425,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::__getit
//     owner=unresolved_namespace; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1257,"record_offset":9784,"parent_record_offset":9744,"ancestor_record_offset":1568,"inlinee":7260,"source_line":85,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1257,"record_offset":9948,"parent_record_offset":9744,"ancestor_record_offset":1568,"inlinee":7263,"source_line":429,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/puffin-0.14.1/src/lib.rs
//     evidence={"module":1257,"record_offset":10220,"parent_record_offset":9948,"ancestor_record_offset":1568,"inlinee":7311,"source_line":591,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":8492,"parent_record_offset":8452,"ancestor_record_offset":1568,"inlinee":59979,"source_line":1716,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":8324,"parent_record_offset":8292,"ancestor_record_offset":1568,"inlinee":34970,"source_line":952,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":8416,"parent_record_offset":8292,"ancestor_record_offset":1568,"inlinee":34979,"source_line":919,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":4460,"parent_record_offset":4432,"ancestor_record_offset":1568,"inlinee":18839,"source_line":1001,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":4432,"parent_record_offset":4404,"ancestor_record_offset":1568,"inlinee":34971,"source_line":971,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":21920,"parent_record_offset":21864,"ancestor_record_offset":1568,"inlinee":21558,"source_line":727,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":19288,"parent_record_offset":19264,"ancestor_record_offset":1568,"inlinee":18837,"source_line":2118,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$43::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":19176,"parent_record_offset":19152,"ancestor_record_offset":1568,"inlinee":34929,"source_line":2228,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$44::next
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":1257,"record_offset":19224,"parent_record_offset":19200,"ancestor_record_offset":1568,"inlinee":34931,"source_line":2237,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":1568,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::ceil
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":13680,"parent_record_offset":13656,"ancestor_record_offset":1568,"inlinee":14652,"source_line":73,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":12432,"parent_record_offset":12400,"ancestor_record_offset":1568,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":1257,"record_offset":9812,"parent_record_offset":9784,"ancestor_record_offset":1568,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":1257,"record_offset":9844,"parent_record_offset":9812,"ancestor_record_offset":1568,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1257,"record_offset":9744,"parent_record_offset":9700,"ancestor_record_offset":1568,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":1257,"record_offset":9700,"parent_record_offset":9656,"ancestor_record_offset":1568,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":2492,"parent_record_offset":2436,"ancestor_record_offset":1568,"inlinee":59973,"source_line":594,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":1568,"inlinee":59596,"source_line":601,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::plaster_mesh::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":2436,"parent_record_offset":2376,"ancestor_record_offset":1568,"inlinee":59972,"source_line":160,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::plaster_mesh::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":4372,"parent_record_offset":4344,"ancestor_record_offset":1568,"inlinee":59975,"source_line":168,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::plaster_mesh::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":4748,"parent_record_offset":4708,"ancestor_record_offset":1568,"inlinee":59976,"source_line":191,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::plaster_mesh::closure$3
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":5044,"parent_record_offset":5012,"ancestor_record_offset":1568,"inlinee":59977,"source_line":197,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::plaster_mesh::closure$4
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":1568,"inlinee":59978,"source_line":222,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3840,"parent_record_offset":2492,"ancestor_record_offset":1568,"inlinee":59974,"source_line":774,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::Curve::get_unnormalized_tangent_at_coord
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":12068,"parent_record_offset":12040,"ancestor_record_offset":1568,"inlinee":21585,"source_line":633,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::CurveInternalCoord::i1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/mod.rs
//     evidence={"module":1257,"record_offset":11300,"parent_record_offset":11200,"ancestor_record_offset":1568,"inlinee":21359,"source_line":1113,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::end
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":11168,"parent_record_offset":10912,"ancestor_record_offset":1568,"inlinee":21611,"source_line":335,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::extend_right
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":11140,"parent_record_offset":10912,"ancestor_record_offset":1568,"inlinee":21485,"source_line":233,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_curve_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":10952,"parent_record_offset":10912,"ancestor_record_offset":1568,"inlinee":21742,"source_line":86,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::from_point_range
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":10980,"parent_record_offset":10952,"ancestor_record_offset":1568,"inlinee":21607,"source_line":107,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":11076,"parent_record_offset":11048,"ancestor_record_offset":1568,"inlinee":21484,"source_line":193,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::curve::curve_slice::CurveSlice::remap_2d
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_slice.rs
//     evidence={"module":1257,"record_offset":11048,"parent_record_offset":10912,"ancestor_record_offset":1568,"inlinee":21483,"source_line":217,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] utils::impl$1::normalize
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":1257,"record_offset":12288,"parent_record_offset":12040,"ancestor_record_offset":1568,"inlinee":21579,"source_line":96,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA44
// 首条位置: {"module":1257,"inlinee":59972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::mirror_holes_as_needed
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":2492,"parent_record_offset":2436,"ancestor_record_offset":2436,"inlinee":59973,"source_line":594,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2436,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":2436,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":2436,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2824,"parent_record_offset":2728,"ancestor_record_offset":2436,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4228,"parent_record_offset":4128,"ancestor_record_offset":2436,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":2436,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3672,"parent_record_offset":3632,"ancestor_record_offset":2436,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":2436,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":2436,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2692,"parent_record_offset":2652,"ancestor_record_offset":2436,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2944,"parent_record_offset":2692,"ancestor_record_offset":2436,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":2436,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3632,"parent_record_offset":3176,"ancestor_record_offset":2436,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2436,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3576,"parent_record_offset":3176,"ancestor_record_offset":2436,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3704,"parent_record_offset":3176,"ancestor_record_offset":2436,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2612,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2652,"parent_record_offset":2612,"ancestor_record_offset":2436,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3128,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1257,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":2436,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2768,"parent_record_offset":2728,"ancestor_record_offset":2436,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2436,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":2436,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":2436,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2436,"inlinee":18213,"source_line":116,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":3220,"parent_record_offset":3176,"ancestor_record_offset":2436,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":2988,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":2436,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3744,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1257,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":2436,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":2436,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2436,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":3484,"parent_record_offset":3220,"ancestor_record_offset":2436,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1257,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2436,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":4076,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":2436,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2436,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3436,"parent_record_offset":3392,"ancestor_record_offset":2436,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":2436,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":3896,"parent_record_offset":3840,"ancestor_record_offset":2436,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":2436,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3392,"parent_record_offset":3304,"ancestor_record_offset":2436,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2436,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1257,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2436,"inlinee":59594,"source_line":126,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2436,"inlinee":59596,"source_line":601,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3840,"parent_record_offset":2492,"ancestor_record_offset":2436,"inlinee":59974,"source_line":774,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 168
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA47
// 首条位置: {"module":1257,"inlinee":59975}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 191
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA48
// 首条位置: {"module":1257,"inlinee":59976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh::closure$3
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 197
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA49
// 首条位置: {"module":1257,"inlinee":59977}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::plaster_mesh::closure$4
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA4A
// 首条位置: {"module":1257,"inlinee":59978}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 594
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA45
// 首条位置: {"module":1257,"inlinee":59973}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2612,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3128,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":2988,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3744,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":4076,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3840,"parent_record_offset":2492,"ancestor_record_offset":2492,"inlinee":59974,"source_line":774,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2848,"parent_record_offset":2824,"ancestor_record_offset":2492,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2872,"parent_record_offset":2848,"ancestor_record_offset":2492,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":2492,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":2824,"parent_record_offset":2728,"ancestor_record_offset":2492,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1257,"record_offset":4228,"parent_record_offset":4128,"ancestor_record_offset":2492,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4160,"parent_record_offset":4128,"ancestor_record_offset":2492,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3672,"parent_record_offset":3632,"ancestor_record_offset":2492,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":3600,"parent_record_offset":3576,"ancestor_record_offset":2492,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2728,"parent_record_offset":2692,"ancestor_record_offset":2492,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2692,"parent_record_offset":2652,"ancestor_record_offset":2492,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":2944,"parent_record_offset":2692,"ancestor_record_offset":2492,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1257,"record_offset":4128,"parent_record_offset":4100,"ancestor_record_offset":2492,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3632,"parent_record_offset":3176,"ancestor_record_offset":2492,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_desugared
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3176,"parent_record_offset":3152,"ancestor_record_offset":2492,"inlinee":8573,"source_line":2980,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3576,"parent_record_offset":3176,"ancestor_record_offset":2492,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":3704,"parent_record_offset":3176,"ancestor_record_offset":2492,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1257,"record_offset":2652,"parent_record_offset":2612,"ancestor_record_offset":2492,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$0::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":1257,"record_offset":3152,"parent_record_offset":3128,"ancestor_record_offset":2492,"inlinee":8592,"source_line":16,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2768,"parent_record_offset":2728,"ancestor_record_offset":2492,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1257,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":2492,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1257,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":2492,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3276,"parent_record_offset":3248,"ancestor_record_offset":2492,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1257,"record_offset":3012,"parent_record_offset":2988,"ancestor_record_offset":2492,"inlinee":18213,"source_line":116,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1257,"record_offset":3220,"parent_record_offset":3176,"ancestor_record_offset":2492,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1257,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":2492,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1257,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":2492,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3336,"parent_record_offset":3304,"ancestor_record_offset":2492,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1257,"record_offset":3508,"parent_record_offset":3484,"ancestor_record_offset":2492,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1257,"record_offset":3484,"parent_record_offset":3220,"ancestor_record_offset":2492,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1257,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2492,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2492,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":2492,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2492,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1257,"record_offset":3436,"parent_record_offset":3392,"ancestor_record_offset":2492,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":2900,"parent_record_offset":2872,"ancestor_record_offset":2492,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":3896,"parent_record_offset":3840,"ancestor_record_offset":2492,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3304,"parent_record_offset":3276,"ancestor_record_offset":2492,"inlinee":8711,"source_line":296,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3392,"parent_record_offset":3304,"ancestor_record_offset":2492,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1257,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2492,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_holes.rs
//     evidence={"module":1257,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":2492,"inlinee":59594,"source_line":126,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":1257,"record_offset":3532,"parent_record_offset":3508,"ancestor_record_offset":2492,"inlinee":59596,"source_line":601,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 601
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xE8CC
// 首条位置: {"module":1226,"inlinee":59596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::WallSide::new
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 652
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x5431
// 首条位置: {"module":372,"inlinee":21553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":372,"record_offset":21232,"parent_record_offset":21200,"ancestor_record_offset":21200,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides
// 记录形式: compiled_procedure; instances=1
// 源码行: 681
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":372,"record_offset":20972,"rva":5171040,"va":5373880160}
// 编译实例 RVAs: 0x4ee760
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1133751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e849b3eeff","size":5,"soft_fail":false,"target_va":5372749712,"va":5373883458,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144e30
//     owner=unresolved_namespace; count=10
//     target_aliases=1 candidates; complete list: plaster_systems.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t29713795","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e88365c501","size":5,"soft_fail":false,"target_va":5403594288,"va":5373880488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29096606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e89efabb01","size":5,"soft_fail":false,"target_va":5402979152,"va":5373882541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t29096427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e8ebf9bb01","size":5,"soft_fail":false,"target_va":5402978944,"va":5373882512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t1883116","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e8ecbb1c00","size":5,"soft_fail":false,"target_va":5375763504,"va":5373880383,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::is_gable
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1883233","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e861bc1c00","size":5,"soft_fail":false,"target_va":5375763568,"va":5373880330,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=9
//     evidence={"assembly":"\tcallq\t8052508","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e81cdf7a00","size":5,"soft_fail":false,"target_va":5381932864,"va":5373880351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] smallvec::SmallVec$LT$A$GT$::reserve_one_unchecked
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t29190928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e8106bbd01","size":5,"soft_fail":false,"target_va":5403071872,"va":5373880939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::get_point_u
//     owner=unresolved_namespace; count=8
//     evidence={"assembly":"\tcallq\t4075159","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5373880160,"raw_hex":"e8972e3e00","size":5,"soft_fail":false,"target_va":5377955840,"va":5373880676,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":372,"record_offset":21140,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":24028,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":21328,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":21555,"source_line":779,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] smallvec::SmallVec::push
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":21264,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":21554,"source_line":1114,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] system_wall_constructor::plaster_systems::WallSide::new
//     owner=private_source_ownership_candidate; count=5
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":21200,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":21553,"source_line":652,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_direct] utils::curve::curve_u::CurveU::new
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/curve/curve_u.rs
//     evidence={"module":372,"record_offset":21356,"parent_record_offset":20972,"ancestor_record_offset":20972,"inlinee":14654,"source_line":87,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":372,"record_offset":24320,"parent_record_offset":24296,"ancestor_record_offset":20972,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":372,"record_offset":24296,"parent_record_offset":24208,"ancestor_record_offset":20972,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":372,"record_offset":24236,"parent_record_offset":24208,"ancestor_record_offset":20972,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":372,"record_offset":24208,"parent_record_offset":24184,"ancestor_record_offset":20972,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":21444,"parent_record_offset":21420,"ancestor_record_offset":20972,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":372,"record_offset":21468,"parent_record_offset":21444,"ancestor_record_offset":20972,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::is_nan
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":372,"record_offset":21508,"parent_record_offset":21356,"ancestor_record_offset":20972,"inlinee":5051,"source_line":502,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":372,"record_offset":21536,"parent_record_offset":21356,"ancestor_record_offset":20972,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":372,"record_offset":24136,"parent_record_offset":24052,"ancestor_record_offset":20972,"inlinee":5355,"source_line":992,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":372,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":20972,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":372,"record_offset":21420,"parent_record_offset":21396,"ancestor_record_offset":20972,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":372,"record_offset":21396,"parent_record_offset":21356,"ancestor_record_offset":20972,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":24160,"parent_record_offset":24136,"ancestor_record_offset":20972,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":372,"record_offset":21976,"parent_record_offset":21844,"ancestor_record_offset":20972,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":372,"record_offset":21292,"parent_record_offset":21264,"ancestor_record_offset":20972,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::spilled
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":21916,"parent_record_offset":21892,"ancestor_record_offset":20972,"inlinee":18839,"source_line":1001,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVec::triple_mut
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":21892,"parent_record_offset":21844,"ancestor_record_offset":20972,"inlinee":21556,"source_line":984,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::SmallVecData::heap_mut
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":21948,"parent_record_offset":21844,"ancestor_record_offset":20972,"inlinee":21558,"source_line":727,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] smallvec::impl$33::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"smallvec","version":"1.13.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/smallvec-1.13.2/src/lib.rs
//     evidence={"module":372,"record_offset":24052,"parent_record_offset":24028,"ancestor_record_offset":20972,"inlinee":18837,"source_line":2118,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::round
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":372,"record_offset":21232,"parent_record_offset":21200,"ancestor_record_offset":20972,"inlinee":6599,"source_line":101,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   [pdb_inline_transitive] system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/wall-constructor/src/plaster_systems.rs
//     evidence={"module":372,"record_offset":21168,"parent_record_offset":21140,"ancestor_record_offset":20972,"inlinee":21551,"source_line":686,"archive":"_pdb/A/modules/00372.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 686
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x542F
// 首条位置: {"module":372,"inlinee":21551}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::sd_box
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 766
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x543E
// 首条位置: {"module":1257,"inlinee":21566}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":14616,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":14648,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14288,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":21559,"source_line":311,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14500,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14436,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14372,"parent_record_offset":14252,"ancestor_record_offset":14252,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":1257,"record_offset":14464,"parent_record_offset":14436,"ancestor_record_offset":14252,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":1257,"record_offset":14400,"parent_record_offset":14372,"ancestor_record_offset":14252,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::math::std_math::abs
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/math.rs
//     evidence={"module":1257,"record_offset":14312,"parent_record_offset":14288,"ancestor_record_offset":14252,"inlinee":21561,"source_line":151,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":1257,"record_offset":14528,"parent_record_offset":14500,"ancestor_record_offset":14252,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::abs
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":14336,"parent_record_offset":14312,"ancestor_record_offset":14252,"inlinee":15239,"source_line":198,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":1257,"record_offset":14576,"parent_record_offset":14528,"ancestor_record_offset":14252,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 774
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA46
// 首条位置: {"module":1257,"inlinee":59974}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":3868,"parent_record_offset":3840,"ancestor_record_offset":3840,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":3896,"parent_record_offset":3840,"ancestor_record_offset":3840,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_wall_constructor::plaster_systems::vec_push_within_capacity_unchecked
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 785
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xEA4C
// 首条位置: {"module":1257,"inlinee":59980}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1257,"record_offset":17696,"parent_record_offset":17660,"ancestor_record_offset":17660,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01257.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1257,"record_offset":17628,"parent_record_offset":17596,"ancestor_record_offset":17596,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01257.jsonl.gz"}
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
// 完整结构及原始类型记录: plaster_systems.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::WallHole
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<system_wall_constructor::plaster_systems::WallHole,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["system_wall_constructor::plaster_systems::WallHole", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] system_wall_constructor::plaster_systems::WallSide
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "array$<system_wall_constructor::plaster_systems::WallSide,4>", "constructor": "array$", "arguments": ["system_wall_constructor::plaster_systems::WallSide", "4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "smallvec::SmallVec::reserve_one_unchecked<array$<system_wall_constructor::plaster_systems::WallSide,4> >", "constructor": "smallvec::SmallVec::reserve_one_unchecked", "arguments": ["array$<system_wall_constructor::plaster_systems::WallSide,4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: plaster_systems.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: plaster_systems.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 4; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] glam::f32::vec3::Vec3 (*)(usize,ref$<glam::f32::vec3::Vec3>)
// 候选参数类型: ["usize", "ref$<glam::f32::vec3::Vec3>"]
// [文件级接口候选，具体函数归属 unknown] glam::f32::vec2::Vec2 (*)(glam::f32::vec3::Vec3)
// 候选参数类型: ["glam::f32::vec3::Vec3"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>", "bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::commands::Commands", "bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::WallSide::new
// 信息 ID: 8d3a62843dad3a9afb16ecf6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=21200; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=21780; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=22440; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=23112; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=23784; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster
// 信息 ID: 8dc538991f96aa5b58e7b878
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fdb8fc8e75dc3b038300e22e: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> > @ 0xc0a49 -> 0x4edca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::CreatePlasterCmd>)> @ 0x14517f -> 0x4edca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster::closure$0
// 信息 ID: 3962e3a8639ff37ccf3ad0b1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::create_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::create_plaster; module=372, inline_record=18804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::create_plaster::closure$1
// 信息 ID: b435ef8a5f39d38c6586145a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::create_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::create_plaster; module=372, inline_record=19812; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster
// 信息 ID: 71307147bcd36eb6178379ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 41fe8c1ac0e0691e0ab9f761: void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>)> > @ 0xba8a1 -> 0x4ed070 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>),void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallPlasters>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::query::Query<ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::event::EventReader<country_core::resources::walls::plaster::DeletePlasterCmd>)> @ 0x144a15 -> 0x4ed070 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$0
// 信息 ID: f7baa45d1e19686aac8ae40b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=372, inline_record=10944; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::maintain_plaster::closure$1
// 信息 ID: cbeadaf62f6e534d43533462
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::maintain_plaster", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::maintain_plaster; module=372, inline_record=11992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed
// 信息 ID: f85c11e86b7a3dd5ae92f969
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=2492; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::mirror_holes_as_needed::closure$0
// 信息 ID: bff4ed0bc4692b3048a8feb0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::mirror_holes_as_needed", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=3532; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh::closure$0
// 信息 ID: 779fee9611d2c9646bf6d66b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=2436; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh::closure$1
// 信息 ID: 022cb9cb7582b981c833a7bf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_wall_constructor::plaster_systems::plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=4372; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh::closure$2
// 信息 ID: 584ac1f323867086f7441175
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "system_wall_constructor::plaster_systems::plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=4748; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh::closure$3
// 信息 ID: c44d817b4e9e49400076b454
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "system_wall_constructor::plaster_systems::plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=5044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh::closure$4
// 信息 ID: b363a43480bc2865f1547ecb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 4, "lexical_owner": "system_wall_constructor::plaster_systems::plaster_mesh", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=7432; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=7668; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >
// 信息 ID: bae209ee8d804d824f633c5f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["utils::geometry::aabb::Aabb2", "country_core::resources::walls::wall_holes::HoleType", "country_core::resources::walls::wall_holes::HoleOrigin"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::filter::Filter -> ["core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>", "country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1"]; 字段布局 = unknown。
// [类型关系] system_wall_constructor::plaster_systems::plaster_mesh -> ["core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::maintain_plaster @ 0x4edab6 -> 0xce2ac0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::create_plaster @ 0x4ee4ec -> 0xce2ac0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::sd_box
// 信息 ID: bd579ae8012274b8dd51287b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=14252; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides
// 信息 ID: 99e903fb5c8f1e47b50f3cad
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0xce2e50 -> 0x4ee760 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::split_wall_into_sides::closure$0
// 信息 ID: a90b337e81e82f1138bc90d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_wall_constructor::plaster_systems::split_wall_into_sides", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::split_wall_into_sides; module=372, inline_record=21168; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::vec_push_within_capacity_assert
// 信息 ID: d4affb01896a14ec20906bcd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=3840; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=3928; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=3984; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=4012; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=4044; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=17028; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=18204; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=18972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_wall_constructor::plaster_systems::vec_push_within_capacity_unchecked
// 信息 ID: 20dbeb7c2d358262fbf34cd1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=17596; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=17660; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=17756; 不是独立调用边。
// [PDB 内联宿主] system_wall_constructor::plaster_systems::plaster_mesh<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> >; module=1257, inline_record=17960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
