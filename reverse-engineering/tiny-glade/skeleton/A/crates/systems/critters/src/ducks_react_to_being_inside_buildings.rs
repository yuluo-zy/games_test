// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3678c54dc163a80bf70882d30a26b15fd950592ef3c8c80d6d52784c7c797492
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: ducks_react_to_being_inside_buildings.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3612
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
// use fixedbitset; // namespace candidate
//   package identities: [{"name":"fixedbitset","source":"PDB_registry_path_only","version":"0.5.7"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use system_critters; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings
// 记录形式: compiled_procedure; instances=1
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3612,"record_offset":252,"rva":28539776,"va":5397248896}
// 编译实例 RVAs: 0x1b37b80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406b09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: ducks_react_to_being_inside_buildings.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21525866","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8968ab7fe","size":5,"soft_fail":false,"target_va":5375723952,"va":5397249813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5728938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8aa6a5700","size":5,"soft_fail":false,"target_va":5402978880,"va":5397249937,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::audio_resources::AudioInstance::start_if_not_playing
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22567215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8d1a6a7fe","size":5,"soft_fail":false,"target_va":5374682656,"va":5397249866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::roofs::roof_shape::Roof::height
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21486259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e84d25b8fe","size":5,"soft_fail":false,"target_va":5375763504,"va":5397249758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15316571","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8a54916ff","size":5,"soft_fail":false,"target_va":5381932784,"va":5397249350,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_wall_state::PublicWallState::max_y
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-15316698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8264916ff","size":5,"soft_fail":false,"target_va":5381932864,"va":5397249557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21814305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8df23b3fe","size":5,"soft_fail":false,"target_va":5375434976,"va":5397249276,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::roof_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21813551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397248896,"raw_hex":"e8d126b3fe","size":5,"soft_fail":false,"target_va":5375436032,"va":5397249578,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::event::EventReader::read
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3612,"record_offset":464,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":6366,"source_line":468,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::impl$20::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3612,"record_offset":1004,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":9712,"source_line":770,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3612,"record_offset":1576,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3612,"record_offset":1380,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::ops::range::RangeInclusive::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3612,"record_offset":3064,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7456,"source_line":509,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":2000,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4151,"source_line":1408,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":3008,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":1316,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::public_wall_state::PublicWallState::is_premade_shape
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/public_wall_state.rs
//     evidence={"module":3612,"record_offset":1352,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21375,"source_line":96,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3612,"record_offset":2264,"parent_record_offset":2120,"ancestor_record_offset":252,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3612,"record_offset":2120,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::EventIteratorWithId::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3612,"record_offset":552,"parent_record_offset":528,"ancestor_record_offset":252,"inlinee":6371,"source_line":810,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::ManualEventReader::read_with_id
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3612,"record_offset":528,"parent_record_offset":488,"ancestor_record_offset":252,"inlinee":6369,"source_line":722,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::impl$23::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3612,"record_offset":1028,"parent_record_offset":1004,"ancestor_record_offset":252,"inlinee":9713,"source_line":842,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":2908,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":2540,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":1904,"parent_record_offset":1880,"ancestor_record_offset":252,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3612,"record_offset":1428,"parent_record_offset":1404,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3612,"record_offset":1452,"parent_record_offset":1428,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3612,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3612,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":252,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3612,"record_offset":1404,"parent_record_offset":1380,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3612,"record_offset":1880,"parent_record_offset":1600,"ancestor_record_offset":252,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3612,"record_offset":1952,"parent_record_offset":1928,"ancestor_record_offset":252,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":252,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":252,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":252,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":252,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":1928,"parent_record_offset":1904,"ancestor_record_offset":252,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":1852,"parent_record_offset":1600,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":252,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":1768,"parent_record_offset":1600,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3612,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":252,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::system_param::impl$14::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/system_param.rs
//     evidence={"module":3612,"record_offset":488,"parent_record_offset":464,"ancestor_record_offset":252,"inlinee":7509,"source_line":805,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3612,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":252,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3612,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":252,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$10::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3612,"record_offset":3116,"parent_record_offset":3088,"ancestor_record_offset":252,"inlinee":6743,"source_line":1676,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3612,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":252,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$50::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3612,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":252,"inlinee":5484,"source_line":1517,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3612,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3612,"record_offset":1056,"parent_record_offset":1028,"ancestor_record_offset":252,"inlinee":8620,"source_line":50,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":3612,"record_offset":1160,"parent_record_offset":1136,"ancestor_record_offset":252,"inlinee":8625,"source_line":51,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3612,"record_offset":596,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3612,"record_offset":2208,"parent_record_offset":2120,"ancestor_record_offset":252,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::range::RangeBounds::contains
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\range.rs
//     evidence={"module":3612,"record_offset":3088,"parent_record_offset":3064,"ancestor_record_offset":252,"inlinee":6738,"source_line":817,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":252,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":252,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":1136,"parent_record_offset":1056,"ancestor_record_offset":252,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3612,"record_offset":896,"parent_record_offset":872,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3612,"record_offset":1260,"parent_record_offset":1236,"ancestor_record_offset":252,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3612,"record_offset":1680,"parent_record_offset":1644,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3612,"record_offset":2484,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3612,"record_offset":660,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3612,"record_offset":848,"parent_record_offset":552,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3612,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3612,"record_offset":720,"parent_record_offset":696,"ancestor_record_offset":252,"inlinee":7968,"source_line":353,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3612,"record_offset":696,"parent_record_offset":660,"ancestor_record_offset":252,"inlinee":8098,"source_line":479,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3612,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3612,"record_offset":1236,"parent_record_offset":1212,"ancestor_record_offset":252,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3612,"record_offset":1212,"parent_record_offset":1160,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2296,"parent_record_offset":2076,"ancestor_record_offset":252,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":252,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":252,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2356,"parent_record_offset":2328,"ancestor_record_offset":252,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3612,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":252,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
//     evidence={"module":3612,"record_offset":2028,"parent_record_offset":2000,"ancestor_record_offset":252,"inlinee":108553,"source_line":34,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/ducks_react_to_being_inside_buildings.rs
//     evidence={"module":3612,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":252,"inlinee":108554,"source_line":35,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A809
// 首条位置: {"module":3612,"inlinee":108553}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3612,"record_offset":2052,"parent_record_offset":2028,"ancestor_record_offset":2028,"inlinee":21384,"source_line":846,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::archetype::impl$24::eq
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/archetype.rs
//     evidence={"module":3612,"record_offset":2264,"parent_record_offset":2120,"ancestor_record_offset":2028,"inlinee":21390,"source_line":74,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::entity::Entities::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/entity/mod.rs
//     evidence={"module":3612,"record_offset":2120,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":21387,"source_line":739,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":2908,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_archetype
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":2540,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":21391,"source_line":933,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3612,"record_offset":2564,"parent_record_offset":2540,"ancestor_record_offset":2028,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::get_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3612,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":2028,"inlinee":21385,"source_line":859,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3612,"record_offset":2868,"parent_record_offset":2844,"ancestor_record_offset":2028,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":2028,"inlinee":6338,"source_line":56,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":2028,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2688,"parent_record_offset":2664,"ancestor_record_offset":2028,"inlinee":6340,"source_line":58,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3612,"record_offset":2776,"parent_record_offset":2752,"ancestor_record_offset":2028,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":2844,"parent_record_offset":2564,"ancestor_record_offset":2028,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":2592,"parent_record_offset":2564,"ancestor_record_offset":2028,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3612,"record_offset":2512,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3612,"record_offset":2932,"parent_record_offset":2908,"ancestor_record_offset":2028,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3612,"record_offset":2328,"parent_record_offset":2296,"ancestor_record_offset":2028,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$24::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3612,"record_offset":2232,"parent_record_offset":2208,"ancestor_record_offset":2028,"inlinee":13915,"source_line":1471,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::impl$2::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":3612,"record_offset":2208,"parent_record_offset":2120,"ancestor_record_offset":2028,"inlinee":21388,"source_line":136,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":2712,"parent_record_offset":2688,"ancestor_record_offset":2028,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3612,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":2028,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3612,"record_offset":2436,"parent_record_offset":2404,"ancestor_record_offset":2028,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3612,"record_offset":2484,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3612,"record_offset":2152,"parent_record_offset":2120,"ancestor_record_offset":2028,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3612,"record_offset":2176,"parent_record_offset":2152,"ancestor_record_offset":2028,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2296,"parent_record_offset":2076,"ancestor_record_offset":2028,"inlinee":4884,"source_line":392,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::contains_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2380,"parent_record_offset":2356,"ancestor_record_offset":2028,"inlinee":4889,"source_line":408,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::FixedBitSet::get_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2404,"parent_record_offset":2380,"ancestor_record_offset":2028,"inlinee":4890,"source_line":171,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] fixedbitset::impl$2::contains::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"fixedbitset","version":"0.5.7","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/fixedbitset-0.5.7/src/lib.rs
//     evidence={"module":3612,"record_offset":2356,"parent_record_offset":2328,"ancestor_record_offset":2028,"inlinee":4888,"source_line":395,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3612,"record_offset":2800,"parent_record_offset":2776,"ancestor_record_offset":2028,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03612.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A80A
// 首条位置: {"module":3612,"inlinee":108554}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings
// 信息 ID: 204584daa706e9b99cb86d0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 50d3045a8c3196d6c4c41af2: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> > @ 0x1587a54 -> 0x1b37b80 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >),void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::Duck>,ref$<utils::components::transform::Transform> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >)> @ 0x159258c -> 0x1b37b80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$0
// 信息 ID: 974164af2884e4281e7900b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings; module=3612, inline_record=2028; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings::closure$1
// 信息 ID: 60e57bacd16e662939f59a00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings; module=3612, inline_record=3032; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
