// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/critters/src/sheep/sheep_walk_animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b8b4191c7aa2fa34bc6413896d59ef6684e73d54d06fca128321058d5b814f4c
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sheep_walk_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3610
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use bevy_ecs; // namespace candidate
//   package identities: []
// use bevy_ptr; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// use system_critters; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_walk_animation::sheep_walk_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3610,"record_offset":252,"rva":28537744,"va":5397246864}
// 编译实例 RVAs: 0x1b37390
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142144da0
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: sheep_walk_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t6346560","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397246864,"raw_hex":"e840d76000","size":5,"soft_fail":false,"target_va":5403594144,"va":5397247579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::critters::sheep::Sheep::being_petted
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10680000","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397246864,"raw_hex":"e840095dff","size":5,"soft_fail":false,"target_va":5386567488,"va":5397247483,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3610,"record_offset":2420,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3610,"record_offset":616,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3610,"record_offset":420,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3610,"record_offset":2176,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":4773,"source_line":1127,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3610,"record_offset":2236,"parent_record_offset":252,"ancestor_record_offset":252,"inlinee":21439,"source_line":462,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=8
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3610,"record_offset":2444,"parent_record_offset":2420,"ancestor_record_offset":252,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":832,"parent_record_offset":800,"ancestor_record_offset":252,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":1284,"parent_record_offset":1252,"ancestor_record_offset":252,"inlinee":6327,"source_line":1372,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":1552,"parent_record_offset":1252,"ancestor_record_offset":252,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":728,"parent_record_offset":700,"ancestor_record_offset":252,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":800,"parent_record_offset":764,"ancestor_record_offset":252,"inlinee":24860,"source_line":1566,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":1252,"parent_record_offset":1224,"ancestor_record_offset":252,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3610,"record_offset":1312,"parent_record_offset":1284,"ancestor_record_offset":252,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3610,"record_offset":468,"parent_record_offset":444,"ancestor_record_offset":252,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3610,"record_offset":492,"parent_record_offset":468,"ancestor_record_offset":252,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3610,"record_offset":640,"parent_record_offset":616,"ancestor_record_offset":252,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3610,"record_offset":444,"parent_record_offset":420,"ancestor_record_offset":252,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3610,"record_offset":700,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3610,"record_offset":1224,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3610,"record_offset":1808,"parent_record_offset":1784,"ancestor_record_offset":252,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3610,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":252,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3610,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":252,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3610,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":252,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3610,"record_offset":1472,"parent_record_offset":1408,"ancestor_record_offset":252,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3610,"record_offset":1652,"parent_record_offset":1628,"ancestor_record_offset":252,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_changed_ticks_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":2132,"parent_record_offset":1848,"ancestor_record_offset":252,"inlinee":23284,"source_line":341,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":1784,"parent_record_offset":1552,"ancestor_record_offset":252,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":1044,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":1580,"parent_record_offset":1552,"ancestor_record_offset":252,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":252,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":864,"parent_record_offset":832,"ancestor_record_offset":252,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3610,"record_offset":1140,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3610,"record_offset":980,"parent_record_offset":956,"ancestor_record_offset":252,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3610,"record_offset":764,"parent_record_offset":728,"ancestor_record_offset":252,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3610,"record_offset":2292,"parent_record_offset":2260,"ancestor_record_offset":252,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3610,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":252,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3610,"record_offset":1628,"parent_record_offset":1604,"ancestor_record_offset":252,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3610,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":252,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3610,"record_offset":564,"parent_record_offset":540,"ancestor_record_offset":252,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3610,"record_offset":1108,"parent_record_offset":1076,"ancestor_record_offset":252,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3610,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":252,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3610,"record_offset":1976,"parent_record_offset":1952,"ancestor_record_offset":252,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3610,"record_offset":516,"parent_record_offset":492,"ancestor_record_offset":252,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3610,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":252,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3610,"record_offset":1708,"parent_record_offset":1676,"ancestor_record_offset":252,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3610,"record_offset":540,"parent_record_offset":516,"ancestor_record_offset":252,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3610,"record_offset":1076,"parent_record_offset":640,"ancestor_record_offset":252,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3610,"record_offset":2348,"parent_record_offset":2260,"ancestor_record_offset":252,"inlinee":13203,"source_line":195,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3610,"record_offset":2260,"parent_record_offset":2236,"ancestor_record_offset":252,"inlinee":21370,"source_line":1135,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3610,"record_offset":1676,"parent_record_offset":1652,"ancestor_record_offset":252,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3610,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":252,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   [pdb_inline_transitive] system_critters::sheep::sheep_walk_animation::sheep_walk_animation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/critters/src/sheep/sheep_walk_animation.rs
//     evidence={"module":3610,"record_offset":2204,"parent_record_offset":2176,"ancestor_record_offset":252,"inlinee":108543,"source_line":15,"archive":"_pdb/A/modules/03610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_critters::sheep::sheep_walk_animation::sheep_walk_animation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1A7FF
// 首条位置: {"module":3610,"inlinee":108543}
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
// 完整结构及原始类型记录: sheep_walk_animation.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sheep_walk_animation.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sheep_walk_animation.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_walk_animation::sheep_walk_animation
// 信息 ID: eedee5e282ba741ae18c6ce6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] ea99cadc06891ffadb2df873: void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x158aad6 -> 0x1b37390 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<enum2$<core::option::Option<ref_mut$<utils::components::transform::Transform> > >,ref_mut$<country_core::resources::critters::sheep::Sheep> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x15ab7eb -> 0x1b37390 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_critters::sheep::sheep_walk_animation::sheep_walk_animation::closure$0
// 信息 ID: 5246baf938148a5635d4404b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "system_critters::sheep::sheep_walk_animation::sheep_walk_animation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_critters::sheep::sheep_walk_animation::sheep_walk_animation; module=3610, inline_record=2204; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
