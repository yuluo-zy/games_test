// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/forest/src/tree_animation.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 0b91acea864d786f5defa62db110b5ff7aa8271397e0617a8b1c1b72f5a303fc
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: tree_animation.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3425
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use forest; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use slotmap; // namespace candidate
//   package identities: [{"name":"slotmap","source":"PDB_registry_path_only","version":"1.0.6"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation
// 记录形式: compiled_procedure; instances=1
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3425,"record_offset":244,"rva":27896432,"va":5396605552}
// 编译实例 RVAs: 0x1a9aa70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140593250
//     owner=unresolved_namespace; count=1
//     target_aliases=15 candidates; complete list: tree_animation.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-22051433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e89785affe","size":5,"soft_fail":false,"target_va":5374554704,"va":5396606132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::boxes
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10945953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e85ffa58ff","size":5,"soft_fail":false,"target_va":5385659968,"va":5396605916,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary::get_triggers
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10945857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e8bffa58ff","size":5,"soft_fail":false,"target_va":5385660048,"va":5396605900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] slotmap::KeyData::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-11659847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e8b9154eff","size":5,"soft_fail":false,"target_va":5384946272,"va":5396606114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_direction
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13444407","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e8c9da32ff","size":5,"soft_fail":false,"target_va":5383161488,"va":5396605890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::camera::CameraMatrices::eye_position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-13444455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e899da32ff","size":5,"soft_fail":false,"target_va":5383161424,"va":5396605874,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::intersect_segment
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-12184922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e8a61246ff","size":5,"soft_fail":false,"target_va":5384421392,"va":5396606309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::pack_2x16f_uint
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19664505","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396605552,"raw_hex":"e887f1d3fe","size":5,"soft_fail":false,"target_va":5376941952,"va":5396606452,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":452,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":1588,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":1644,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3425,"record_offset":1360,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3425,"record_offset":572,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3425,"record_offset":2612,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3425,"record_offset":688,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":9711,"source_line":2802,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] forest::tree_animation::should_remove_tree_canopy_from_view
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3425,"record_offset":416,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":106459,"source_line":150,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] std::f32::impl$0::powi
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3425,"record_offset":1560,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":21890,"source_line":390,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] utils::smoothstep
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/lib.rs
//     evidence={"module":3425,"record_offset":604,"parent_record_offset":244,"ancestor_record_offset":244,"inlinee":21527,"source_line":230,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":480,"parent_record_offset":452,"ancestor_record_offset":244,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":1612,"parent_record_offset":1588,"ancestor_record_offset":244,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":244,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":2360,"parent_record_offset":2336,"ancestor_record_offset":244,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":2072,"parent_record_offset":1716,"ancestor_record_offset":244,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":2452,"parent_record_offset":2336,"ancestor_record_offset":244,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":1744,"parent_record_offset":1716,"ancestor_record_offset":244,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":1804,"parent_record_offset":1772,"ancestor_record_offset":244,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":1412,"parent_record_offset":1388,"ancestor_record_offset":244,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":244,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":1668,"parent_record_offset":1644,"ancestor_record_offset":244,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3425,"record_offset":1388,"parent_record_offset":1360,"ancestor_record_offset":244,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3425,"record_offset":2336,"parent_record_offset":1668,"ancestor_record_offset":244,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3425,"record_offset":1716,"parent_record_offset":1668,"ancestor_record_offset":244,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":1888,"parent_record_offset":1860,"ancestor_record_offset":244,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":1860,"parent_record_offset":1832,"ancestor_record_offset":244,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":2120,"parent_record_offset":2096,"ancestor_record_offset":244,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":1960,"parent_record_offset":1916,"ancestor_record_offset":244,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":2160,"parent_record_offset":1668,"ancestor_record_offset":244,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":2096,"parent_record_offset":2072,"ancestor_record_offset":244,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":1832,"parent_record_offset":1804,"ancestor_record_offset":244,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":2384,"parent_record_offset":2360,"ancestor_record_offset":244,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":2252,"parent_record_offset":1668,"ancestor_record_offset":244,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3425,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":244,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$3::into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3425,"record_offset":976,"parent_record_offset":764,"ancestor_record_offset":244,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3425,"record_offset":804,"parent_record_offset":764,"ancestor_record_offset":244,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3425,"record_offset":1040,"parent_record_offset":712,"ancestor_record_offset":244,"inlinee":9715,"source_line":2809,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":3425,"record_offset":712,"parent_record_offset":688,"ancestor_record_offset":244,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3425,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":244,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3425,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":244,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":1316,"parent_record_offset":1064,"ancestor_record_offset":244,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":1008,"parent_record_offset":740,"ancestor_record_offset":244,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3425,"record_offset":1508,"parent_record_offset":1484,"ancestor_record_offset":244,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3425,"record_offset":880,"parent_record_offset":836,"ancestor_record_offset":244,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3425,"record_offset":2276,"parent_record_offset":2252,"ancestor_record_offset":244,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3425,"record_offset":1460,"parent_record_offset":1436,"ancestor_record_offset":244,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3425,"record_offset":2016,"parent_record_offset":1916,"ancestor_record_offset":244,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3425,"record_offset":1484,"parent_record_offset":1460,"ancestor_record_offset":244,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3425,"record_offset":836,"parent_record_offset":804,"ancestor_record_offset":244,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] forest::tree_animation::handplaced_tree_animation::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/forest/src/tree_animation.rs
//     evidence={"module":3425,"record_offset":1064,"parent_record_offset":1040,"ancestor_record_offset":244,"inlinee":106461,"source_line":72,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1288,"parent_record_offset":1064,"ancestor_record_offset":244,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$15::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1196,"parent_record_offset":1064,"ancestor_record_offset":244,"inlinee":25925,"source_line":1113,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$20::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1108,"parent_record_offset":1064,"ancestor_record_offset":244,"inlinee":25927,"source_line":1167,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3425,"record_offset":920,"parent_record_offset":764,"ancestor_record_offset":244,"inlinee":25367,"source_line":52,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::Slot::occupied
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3425,"record_offset":944,"parent_record_offset":920,"ancestor_record_offset":244,"inlinee":13413,"source_line":48,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$12::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3425,"record_offset":764,"parent_record_offset":740,"ancestor_record_offset":244,"inlinee":25366,"source_line":1008,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] slotmap::basic::impl$15::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"slotmap","version":"1.0.6","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/slotmap-1.0.6/src/basic.rs
//     evidence={"module":3425,"record_offset":740,"parent_record_offset":712,"ancestor_record_offset":244,"inlinee":25372,"source_line":1061,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::handplaced_tree_animation::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19FDD
// 首条位置: {"module":3425,"inlinee":106461}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":1316,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1288,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":14692,"source_line":1081,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$15::add_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1196,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":25925,"source_line":1113,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$20::sub_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec3.rs
//     evidence={"module":3425,"record_offset":1108,"parent_record_offset":1064,"ancestor_record_offset":1064,"inlinee":25927,"source_line":1167,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$308::add_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3425,"record_offset":1228,"parent_record_offset":1196,"ancestor_record_offset":1064,"inlinee":14664,"source_line":761,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$336::sub_assign
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3425,"record_offset":1136,"parent_record_offset":1108,"ancestor_record_offset":1064,"inlinee":25009,"source_line":828,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: forest::tree_animation::should_remove_tree_canopy_from_view
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x19FDB
// 首条位置: {"module":3425,"inlinee":106459}
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
// 源码行: 169
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3425,"record_offset":2760,"rva":27897888,"va":5396607008}
// 编译实例 RVAs: 0x1a9b020
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-25302956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396607008,"raw_hex":"e854e87dfe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396607831,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6371320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396607008,"raw_hex":"e8f8376100","size":5,"soft_fail":false,"target_va":5402979344,"va":5396608019,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19669593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396607008,"raw_hex":"e8a7ddd3fe","size":5,"soft_fail":false,"target_va":5376938064,"va":5396607652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3425,"record_offset":4772,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":4716,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":4228,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3425,"record_offset":4456,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":3120,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3425,"record_offset":2920,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3425,"record_offset":4340,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":4428,"parent_record_offset":2760,"ancestor_record_offset":2760,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3425,"record_offset":4644,"parent_record_offset":4612,"ancestor_record_offset":2760,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3425,"record_offset":4612,"parent_record_offset":4572,"ancestor_record_offset":2760,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3425,"record_offset":4572,"parent_record_offset":4536,"ancestor_record_offset":2760,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":2760,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":4480,"parent_record_offset":4456,"ancestor_record_offset":2760,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=3
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3425,"record_offset":4252,"parent_record_offset":4228,"ancestor_record_offset":2760,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3425,"record_offset":4536,"parent_record_offset":4456,"ancestor_record_offset":2760,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":3532,"parent_record_offset":3500,"ancestor_record_offset":2760,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":3800,"parent_record_offset":3500,"ancestor_record_offset":2760,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":4064,"parent_record_offset":4036,"ancestor_record_offset":2760,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":2760,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":4156,"parent_record_offset":4036,"ancestor_record_offset":2760,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":3500,"parent_record_offset":3392,"ancestor_record_offset":2760,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3425,"record_offset":3560,"parent_record_offset":3532,"ancestor_record_offset":2760,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":2972,"parent_record_offset":2948,"ancestor_record_offset":2760,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":2760,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3425,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2760,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3425,"record_offset":2948,"parent_record_offset":2920,"ancestor_record_offset":2760,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3425,"record_offset":4036,"parent_record_offset":3148,"ancestor_record_offset":2760,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3425,"record_offset":3392,"parent_record_offset":3148,"ancestor_record_offset":2760,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3425,"record_offset":3992,"parent_record_offset":3968,"ancestor_record_offset":2760,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":3632,"parent_record_offset":3608,"ancestor_record_offset":2760,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":3608,"parent_record_offset":3584,"ancestor_record_offset":2760,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":2760,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":3720,"parent_record_offset":3656,"ancestor_record_offset":2760,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3425,"record_offset":3900,"parent_record_offset":3876,"ancestor_record_offset":2760,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":3968,"parent_record_offset":3800,"ancestor_record_offset":2760,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":3208,"parent_record_offset":3148,"ancestor_record_offset":2760,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":2760,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":3584,"parent_record_offset":3560,"ancestor_record_offset":2760,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":4088,"parent_record_offset":4064,"ancestor_record_offset":2760,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3425,"record_offset":3308,"parent_record_offset":3148,"ancestor_record_offset":2760,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3425,"record_offset":4184,"parent_record_offset":4156,"ancestor_record_offset":2760,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":3744,"parent_record_offset":3720,"ancestor_record_offset":2760,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3425,"record_offset":3876,"parent_record_offset":3852,"ancestor_record_offset":2760,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3425,"record_offset":3068,"parent_record_offset":3044,"ancestor_record_offset":2760,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3425,"record_offset":3276,"parent_record_offset":3244,"ancestor_record_offset":2760,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3425,"record_offset":4676,"parent_record_offset":4572,"ancestor_record_offset":2760,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3425,"record_offset":3332,"parent_record_offset":3308,"ancestor_record_offset":2760,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3425,"record_offset":3020,"parent_record_offset":2996,"ancestor_record_offset":2760,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3425,"record_offset":4800,"parent_record_offset":4772,"ancestor_record_offset":2760,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3425,"record_offset":3356,"parent_record_offset":3332,"ancestor_record_offset":2760,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3425,"record_offset":4828,"parent_record_offset":4800,"ancestor_record_offset":2760,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3425,"record_offset":3044,"parent_record_offset":3020,"ancestor_record_offset":2760,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3425,"record_offset":3244,"parent_record_offset":3148,"ancestor_record_offset":2760,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03425.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3425,"record_offset":3924,"parent_record_offset":3900,"ancestor_record_offset":2760,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03425.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>

// ------------------------------------------------------------------
// 函数: forest::tree_animation::forest_tree_animation
// 信息 ID: c75c37f48c8fc207d9003f8f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] e156090e402414b922d98ed4: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> > @ 0x1442d90 -> 0x1a9b020 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::resources::tree_storage::InteractableForestTree>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::event::EventWriter<country_core::systems::prefab::DespawnPrefabCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::InteractableForestPlacements>)> @ 0x1443d42 -> 0x1a9b020 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation
// 信息 ID: 905111c96bebf6215fd9cd91
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 154a1c8c75146f343d5796ba: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> > @ 0x1441e31 -> 0x1a9aa70 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeAnimation>,ref_mut$<country_core::systems::prefab::PrefabInstanceData>,enum2$<core::option::Option<ref$<country_core::components::TreeMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::resources::history::tree_edit::TreeCanopyMaxCullDistance>)> @ 0x1443986 -> 0x1a9aa70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::handplaced_tree_animation::closure$0
// 信息 ID: 584995aedf6ab978148f3816
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "forest::tree_animation::handplaced_tree_animation", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3425, inline_record=1064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: forest::tree_animation::should_remove_tree_canopy_from_view
// 信息 ID: 57550630d16ef459b4ad6086
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] forest::tree_animation::handplaced_tree_animation; module=3425, inline_record=416; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
