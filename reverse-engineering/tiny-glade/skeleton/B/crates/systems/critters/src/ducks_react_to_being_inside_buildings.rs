// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: bd0a2c3968a986684002c8b7bfc75a0d4628d56f55426de7a81e2c3cf5a91784
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ducks_react_to_being_inside_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 3883
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
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.5.5"}]
// use system_critters; // namespace candidate
//   package identities: [{"name":"system-critters","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3883,"record_offset":264,"rva":36811072,"va":5405520192}
// 编译实例 RVAs: 0x231b140
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ad2950
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ducks_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25464140","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e8b4727bfe","size":5,"soft_fail":false,"target_va":5380057424,"va":5405521559,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6020096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e800dc5b00","size":5,"soft_fail":false,"target_va":5411541776,"va":5405521675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25969609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e837bc73fe","size":5,"soft_fail":false,"target_va":5379552000,"va":5405521604,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-27493394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e8ee7b5cfe","size":5,"soft_fail":false,"target_va":5378028128,"va":5405521517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25103679","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e8c1f280fe","size":5,"soft_fail":false,"target_va":5380416976,"va":5405520650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25104266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e876f080fe","size":5,"soft_fail":false,"target_va":5380417056,"va":5405521317,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25727958","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e82a6c77fe","size":5,"soft_fail":false,"target_va":5379792624,"va":5405520577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-25727766","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5405520192,"raw_hex":"e8ea6c77fe","size":5,"soft_fail":false,"target_va":5379793584,"va":5405521345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::iterators::impl$0::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3883,"record_offset":1172,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":12537,"source_line":14,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::reader::EventReader::read
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/reader.rs
//     evidence={"module":3883,"record_offset":476,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":8034,"source_line":27,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3883,"record_offset":1924,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":11673,"source_line":884,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":1576,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":13704,"source_line":670,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3883,"record_offset":7100,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4980,"source_line":509,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":5512,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":7044,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":1512,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3883,"record_offset":1548,"parent_record_offset":264,"ancestor_record_offset":264,"inlinee":24411,"source_line":113,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":1756,"parent_record_offset":1732,"ancestor_record_offset":264,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":1804,"parent_record_offset":1780,"ancestor_record_offset":264,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":1780,"parent_record_offset":1756,"ancestor_record_offset":264,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":264,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":264,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":2292,"parent_record_offset":2268,"ancestor_record_offset":264,"inlinee":4613,"source_line":2705,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":2316,"parent_record_offset":2292,"ancestor_record_offset":264,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":792,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::entities
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":4956,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":11682,"source_line":466,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":3584,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":11679,"source_line":615,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetype::table_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":3668,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":5495,"source_line":460,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::ArchetypeEntity::table_row
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":5152,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":17012,"source_line":325,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::Archetypes::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":3500,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":11678,"source_line":895,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":5964,"parent_record_offset":5632,"ancestor_record_offset":264,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3883,"record_offset":5632,"parent_record_offset":5588,"ancestor_record_offset":264,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::event_cursor::EventCursor::read_with_id
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/event_cursor.rs
//     evidence={"module":3883,"record_offset":540,"parent_record_offset":500,"ancestor_record_offset":264,"inlinee":8037,"source_line":88,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::EventIteratorWithId::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3883,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":264,"inlinee":8039,"source_line":54,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::iterators::impl$3::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/event/iterators.rs
//     evidence={"module":3883,"record_offset":1196,"parent_record_offset":1172,"ancestor_record_offset":264,"inlinee":12538,"source_line":86,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":4864,"parent_record_offset":4840,"ancestor_record_offset":264,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":3788,"parent_record_offset":3760,"ancestor_record_offset":264,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":2380,"parent_record_offset":2352,"ancestor_record_offset":264,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":4840,"parent_record_offset":4816,"ancestor_record_offset":264,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":4888,"parent_record_offset":4864,"ancestor_record_offset":264,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$61::fetch
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":4816,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":17019,"source_line":2083,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3883,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":264,"inlinee":13709,"source_line":39,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3883,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":264,"inlinee":9292,"source_line":2359,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/iter.rs
//     evidence={"module":3883,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":264,"inlinee":11674,"source_line":2458,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3883,"record_offset":3760,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":13725,"source_line":182,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/world_query.rs
//     evidence={"module":3883,"record_offset":2352,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":17013,"source_line":195,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3883,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":264,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":2480,"parent_record_offset":2456,"ancestor_record_offset":264,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":2456,"parent_record_offset":2432,"ancestor_record_offset":264,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":264,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":2432,"parent_record_offset":2404,"ancestor_record_offset":264,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":264,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":2268,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":11675,"source_line":615,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":4912,"parent_record_offset":4888,"ancestor_record_offset":264,"inlinee":5472,"source_line":127,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":2184,"parent_record_offset":1948,"ancestor_record_offset":264,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3883,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":264,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":2752,"parent_record_offset":2728,"ancestor_record_offset":264,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":264,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":264,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::impl$4::into_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":264,"inlinee":25723,"source_line":2574,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$19::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/system_param.rs
//     evidence={"module":3883,"record_offset":500,"parent_record_offset":476,"ancestor_record_offset":264,"inlinee":10713,"source_line":1234,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3883,"record_offset":5312,"parent_record_offset":5284,"ancestor_record_offset":264,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3883,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":264,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3883,"record_offset":7152,"parent_record_offset":7124,"ancestor_record_offset":264,"inlinee":4984,"source_line":1943,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3883,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":264,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$53::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3883,"record_offset":7176,"parent_record_offset":7152,"ancestor_record_offset":264,"inlinee":4986,"source_line":1788,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3883,"record_offset":1248,"parent_record_offset":1224,"ancestor_record_offset":264,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3883,"record_offset":1224,"parent_record_offset":1196,"ancestor_record_offset":264,"inlinee":4570,"source_line":83,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3883,"record_offset":1328,"parent_record_offset":1304,"ancestor_record_offset":264,"inlinee":4573,"source_line":84,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3883,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":264,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":3883,"record_offset":608,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3883,"record_offset":5908,"parent_record_offset":5632,"ancestor_record_offset":264,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3883,"record_offset":2572,"parent_record_offset":2508,"ancestor_record_offset":264,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\range.rs
//     evidence={"module":3883,"record_offset":7124,"parent_record_offset":7100,"ancestor_record_offset":264,"inlinee":4982,"source_line":817,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":264,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":1272,"parent_record_offset":1248,"ancestor_record_offset":264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":2596,"parent_record_offset":2572,"ancestor_record_offset":264,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":2636,"parent_record_offset":2456,"ancestor_record_offset":264,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":1304,"parent_record_offset":1224,"ancestor_record_offset":264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3883,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":264,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3883,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":264,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":3883,"record_offset":2068,"parent_record_offset":2032,"ancestor_record_offset":264,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3883,"record_offset":664,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3883,"record_offset":1016,"parent_record_offset":564,"ancestor_record_offset":264,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":264,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":264,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":264,"inlinee":8042,"source_line":364,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":264,"inlinee":8041,"source_line":537,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":3883,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":264,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3883,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":264,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3883,"record_offset":1380,"parent_record_offset":1328,"ancestor_record_offset":264,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":3883,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":264,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":5996,"parent_record_offset":5588,"ancestor_record_offset":264,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":264,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":264,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":264,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3883,"record_offset":2684,"parent_record_offset":2660,"ancestor_record_offset":264,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
//     evidence={"module":3883,"record_offset":5540,"parent_record_offset":5512,"ancestor_record_offset":264,"inlinee":122509,"source_line":34,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"system-critters","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
//     evidence={"module":3883,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":264,"inlinee":122510,"source_line":35,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DE8D
// 首条位置: {"module":3883,"inlinee":122509}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":5564,"parent_record_offset":5540,"ancestor_record_offset":5540,"inlinee":4837,"source_line":1385,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":5812,"parent_record_offset":5788,"ancestor_record_offset":5540,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":5860,"parent_record_offset":5836,"ancestor_record_offset":5540,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":3883,"record_offset":5836,"parent_record_offset":5812,"ancestor_record_offset":5540,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":5788,"parent_record_offset":5764,"ancestor_record_offset":5540,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":5764,"parent_record_offset":5664,"ancestor_record_offset":5540,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":3883,"record_offset":6200,"parent_record_offset":6176,"ancestor_record_offset":5540,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$25::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/archetype.rs
//     evidence={"module":3883,"record_offset":5964,"parent_record_offset":5632,"ancestor_record_offset":5540,"inlinee":4846,"source_line":76,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/entity/mod.rs
//     evidence={"module":3883,"record_offset":5632,"parent_record_offset":5588,"ancestor_record_offset":5540,"inlinee":4840,"source_line":842,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::StorageSwitch::extract
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":6912,"parent_record_offset":6888,"ancestor_record_offset":5540,"inlinee":4876,"source_line":2472,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_archetype
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":6436,"parent_record_offset":5588,"ancestor_record_offset":5540,"inlinee":4856,"source_line":1160,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$26::set_table
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":6460,"parent_record_offset":6436,"ancestor_record_offset":5540,"inlinee":4857,"source_line":1175,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":6888,"parent_record_offset":5588,"ancestor_record_offset":5540,"inlinee":4874,"source_line":1229,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$27::fetch::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/query/fetch.rs
//     evidence={"module":3883,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":5540,"inlinee":4878,"source_line":1235,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_array::BlobArray::get_sub_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/blob_array.rs
//     evidence={"module":3883,"record_offset":6836,"parent_record_offset":6812,"ancestor_record_offset":5540,"inlinee":4872,"source_line":143,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":5540,"inlinee":4864,"source_line":58,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":6536,"parent_record_offset":6512,"ancestor_record_offset":5540,"inlinee":4862,"source_line":395,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/sparse_set.rs
//     evidence={"module":3883,"record_offset":6696,"parent_record_offset":6672,"ancestor_record_offset":5540,"inlinee":4866,"source_line":396,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":6512,"parent_record_offset":6484,"ancestor_record_offset":5540,"inlinee":4860,"source_line":475,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_data_slice_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":6484,"parent_record_offset":6460,"ancestor_record_offset":5540,"inlinee":4859,"source_line":363,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":6176,"parent_record_offset":5588,"ancestor_record_offset":5540,"inlinee":4854,"source_line":721,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::column::ThinColumn::get_data_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/column.rs
//     evidence={"module":3883,"record_offset":6812,"parent_record_offset":6788,"ancestor_record_offset":5540,"inlinee":4870,"source_line":303,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::impl$4::get_data_slice_for::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/storage/table/mod.rs
//     evidence={"module":3883,"record_offset":6788,"parent_record_offset":6764,"ancestor_record_offset":5540,"inlinee":4868,"source_line":368,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ecs","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/system/query.rs
//     evidence={"module":3883,"record_offset":5588,"parent_record_offset":5564,"ancestor_record_offset":5540,"inlinee":4838,"source_line":1591,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bevy_ptr","version":"0.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ptr-0.16.0/src/lib.rs
//     evidence={"module":3883,"record_offset":6960,"parent_record_offset":6936,"ancestor_record_offset":5540,"inlinee":4880,"source_line":526,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\bool.rs
//     evidence={"module":3883,"record_offset":6024,"parent_record_offset":5996,"ancestor_record_offset":5540,"inlinee":4781,"source_line":61,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":3883,"record_offset":5932,"parent_record_offset":5908,"ancestor_record_offset":5540,"inlinee":4844,"source_line":1742,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":3883,"record_offset":5908,"parent_record_offset":5632,"ancestor_record_offset":5540,"inlinee":4842,"source_line":200,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":3883,"record_offset":6608,"parent_record_offset":6584,"ancestor_record_offset":5540,"inlinee":4147,"source_line":250,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":5540,"inlinee":4518,"source_line":1447,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":6632,"parent_record_offset":6608,"ancestor_record_offset":5540,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":3883,"record_offset":6672,"parent_record_offset":6536,"ancestor_record_offset":5540,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3883,"record_offset":6128,"parent_record_offset":6096,"ancestor_record_offset":5540,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":3883,"record_offset":5664,"parent_record_offset":5632,"ancestor_record_offset":5540,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":5732,"parent_record_offset":5704,"ancestor_record_offset":5540,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":3883,"record_offset":5704,"parent_record_offset":5664,"ancestor_record_offset":5540,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":5996,"parent_record_offset":5588,"ancestor_record_offset":5540,"inlinee":4848,"source_line":392,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6072,"parent_record_offset":6048,"ancestor_record_offset":5540,"inlinee":4851,"source_line":408,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6096,"parent_record_offset":6072,"ancestor_record_offset":5540,"inlinee":4852,"source_line":171,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3883,"record_offset":6048,"parent_record_offset":6024,"ancestor_record_offset":5540,"inlinee":4850,"source_line":395,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3883,"record_offset":6720,"parent_record_offset":6696,"ancestor_record_offset":5540,"inlinee":4171,"source_line":153,"archive":"_pdb/B/modules/03883.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1DE8E
// 首条位置: {"module":3883,"inlinee":122510}
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
// 完整结构及原始类型记录: ducks_react_to_being_inside_buildings.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: ducks_react_to_being_inside_buildings.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: ducks_react_to_being_inside_buildings.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings
// 信息 ID: 403a88ee781d3a38b142326b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] cb0780083dc4975180c849f6: void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::schedule_system::InfallibleSystemWrapper<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > > @ 0x1d9e857 -> 0x231b140 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::schedule_system::impl$1::run_unsafe<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::event::reader::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0x1da7402 -> 0x231b140 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
// 信息 ID: 5a3fd5b003d7cff36fb0d341
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings; module=3883, inline_record=5540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
// 信息 ID: e99fe882ec75b136196076ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings; module=3883, inline_record=7068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
