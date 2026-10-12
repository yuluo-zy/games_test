// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/tree-tool/src/tree_move_bend.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 3edef6c47c8770eaf4cfbdb9ab6e58cf00bfaf3fe4d8dc3550021859b00c51e0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: tree_move_bend.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3380
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
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_tree::tree_move_bend::tree_move_bend
// 记录形式: compiled_procedure; instances=1
// 源码行: 8
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3380,"record_offset":248,"rva":27851056,"va":5396560176}
// 编译实例 RVAs: 0x1a8f930
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::resources::placement_data::PlacementDataSsbo::position
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-22280594","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396560176,"raw_hex":"e86e06acfe","size":5,"soft_fail":false,"target_va":5374280016,"va":5396560605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::time_continuous_lerp
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-19620677","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396560176,"raw_hex":"e8bb9cd4fe","size":5,"soft_fail":false,"target_va":5376939968,"va":5396560640,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3380,"record_offset":800,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3380,"record_offset":856,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3380,"record_offset":412,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] core::array::equality::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":3380,"record_offset":608,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":8003,"source_line":14,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3380,"record_offset":2036,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":1820,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21373,"source_line":380,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":1912,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":21371,"source_line":450,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":2176,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":1764,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":2120,"parent_record_offset":248,"ancestor_record_offset":248,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=4
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3380,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":248,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3380,"record_offset":1500,"parent_record_offset":1476,"ancestor_record_offset":248,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3380,"record_offset":1188,"parent_record_offset":1164,"ancestor_record_offset":248,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3380,"record_offset":460,"parent_record_offset":436,"ancestor_record_offset":248,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3380,"record_offset":484,"parent_record_offset":460,"ancestor_record_offset":248,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3380,"record_offset":880,"parent_record_offset":856,"ancestor_record_offset":248,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3380,"record_offset":436,"parent_record_offset":412,"ancestor_record_offset":248,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3380,"record_offset":1476,"parent_record_offset":880,"ancestor_record_offset":248,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3380,"record_offset":1164,"parent_record_offset":880,"ancestor_record_offset":248,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3380,"record_offset":1436,"parent_record_offset":1412,"ancestor_record_offset":248,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3380,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":248,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3380,"record_offset":1288,"parent_record_offset":1264,"ancestor_record_offset":248,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3380,"record_offset":1412,"parent_record_offset":1188,"ancestor_record_offset":248,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3380,"record_offset":1136,"parent_record_offset":880,"ancestor_record_offset":248,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3380,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":248,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3380,"record_offset":1524,"parent_record_offset":1500,"ancestor_record_offset":248,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3380,"record_offset":1052,"parent_record_offset":880,"ancestor_record_offset":248,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3380,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":248,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$8::spec_ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":3380,"record_offset":636,"parent_record_offset":608,"ancestor_record_offset":248,"inlinee":22134,"source_line":141,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$33::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3380,"record_offset":748,"parent_record_offset":720,"ancestor_record_offset":248,"inlinee":4541,"source_line":1471,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::recip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3380,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":248,"inlinee":21457,"source_line":867,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3380,"record_offset":2000,"parent_record_offset":1940,"ancestor_record_offset":248,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3380,"record_offset":2200,"parent_record_offset":2176,"ancestor_record_offset":248,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3380,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":248,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3380,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":248,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3380,"record_offset":1360,"parent_record_offset":1336,"ancestor_record_offset":248,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3380,"record_offset":556,"parent_record_offset":532,"ancestor_record_offset":248,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3380,"record_offset":1016,"parent_record_offset":992,"ancestor_record_offset":248,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3380,"record_offset":964,"parent_record_offset":928,"ancestor_record_offset":248,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::SlicePartialEq::not_equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3380,"record_offset":692,"parent_record_offset":664,"ancestor_record_offset":248,"inlinee":13793,"source_line":48,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3380,"record_offset":664,"parent_record_offset":636,"ancestor_record_offset":248,"inlinee":13791,"source_line":19,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$4::equal
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":3380,"record_offset":720,"parent_record_offset":692,"ancestor_record_offset":248,"inlinee":4539,"source_line":58,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3380,"record_offset":1076,"parent_record_offset":1052,"ancestor_record_offset":248,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3380,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":248,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3380,"record_offset":508,"parent_record_offset":484,"ancestor_record_offset":248,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3380,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":248,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3380,"record_offset":1336,"parent_record_offset":1312,"ancestor_record_offset":248,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3380,"record_offset":532,"parent_record_offset":508,"ancestor_record_offset":248,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3380,"record_offset":992,"parent_record_offset":928,"ancestor_record_offset":248,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3380,"record_offset":928,"parent_record_offset":880,"ancestor_record_offset":248,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":248,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3380,"record_offset":1940,"parent_record_offset":1912,"ancestor_record_offset":248,"inlinee":21372,"source_line":399,"archive":"_pdb/A/modules/03380.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3380,"record_offset":1876,"parent_record_offset":1844,"ancestor_record_offset":248,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03380.jsonl.gz"}
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
// 完整结构及原始类型记录: tree_move_bend.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: tree_move_bend.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: tree_move_bend.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>

// ------------------------------------------------------------------
// 函数: system_tree::tree_move_bend::tree_move_bend
// 信息 ID: 9562eb1ad19150a1bd3b3eeb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 3bf6cb7150390dc20dbfd311: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1415c96 -> 0x1a8f930 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::TreeMoveBend>,ref_mut$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x142e7bb -> 0x1a8f930 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
