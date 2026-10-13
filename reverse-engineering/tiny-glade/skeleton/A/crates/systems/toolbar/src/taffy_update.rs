// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/toolbar/src/taffy_update.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 861d0035e2eba089fd291a050fbb21614348472aeb97e0babac72bc767620fe0
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: taffy_update.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 2420, 2492, 3640
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
// use toolbar; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: toolbar::taffy_update::taffy_finalize_element_size
// 记录形式: compiled_procedure; instances=1
// 源码行: 15
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3640,"record_offset":848,"rva":28665968,"va":5397375088}
// 编译实例 RVAs: 0x1b56870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141b9c210
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: taffy_update.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t284889","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397375088,"raw_hex":"e8d9580400","size":5,"soft_fail":false,"target_va":5397660176,"va":5397375282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_text::TextFragment::bounds
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21330979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397375088,"raw_hex":"e8dd83bafe","size":5,"soft_fail":false,"target_va":5376044688,"va":5397375662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":1204,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":1288,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":1344,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3640,"record_offset":1008,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::max
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3640,"record_offset":2876,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":21427,"source_line":185,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3640,"record_offset":2764,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::style_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3640,"record_offset":1260,"parent_record_offset":848,"ancestor_record_offset":848,"inlinee":84152,"source_line":66,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":1228,"parent_record_offset":1204,"ancestor_record_offset":848,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":1312,"parent_record_offset":1288,"ancestor_record_offset":848,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":1736,"parent_record_offset":1708,"ancestor_record_offset":848,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":2348,"parent_record_offset":2008,"ancestor_record_offset":848,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":2608,"parent_record_offset":2584,"ancestor_record_offset":848,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":848,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":2664,"parent_record_offset":2584,"ancestor_record_offset":848,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":1708,"parent_record_offset":1600,"ancestor_record_offset":848,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":1764,"parent_record_offset":1736,"ancestor_record_offset":848,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":848,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":1080,"parent_record_offset":1056,"ancestor_record_offset":848,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":1368,"parent_record_offset":1344,"ancestor_record_offset":848,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3640,"record_offset":1032,"parent_record_offset":1008,"ancestor_record_offset":848,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3640,"record_offset":2584,"parent_record_offset":1368,"ancestor_record_offset":848,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3640,"record_offset":1600,"parent_record_offset":1368,"ancestor_record_offset":848,"inlinee":22690,"source_line":192,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3640,"record_offset":2540,"parent_record_offset":2516,"ancestor_record_offset":848,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3640,"record_offset":1836,"parent_record_offset":1812,"ancestor_record_offset":848,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3640,"record_offset":1812,"parent_record_offset":1788,"ancestor_record_offset":848,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3640,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":848,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3640,"record_offset":1924,"parent_record_offset":1860,"ancestor_record_offset":848,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3640,"record_offset":2448,"parent_record_offset":2424,"ancestor_record_offset":848,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":2516,"parent_record_offset":2348,"ancestor_record_offset":848,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":1420,"parent_record_offset":1368,"ancestor_record_offset":848,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":848,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":1788,"parent_record_offset":1764,"ancestor_record_offset":848,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":2632,"parent_record_offset":2608,"ancestor_record_offset":848,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":1516,"parent_record_offset":1368,"ancestor_record_offset":848,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3640,"record_offset":2692,"parent_record_offset":2664,"ancestor_record_offset":848,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::impl$0::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3640,"record_offset":2900,"parent_record_offset":2876,"ancestor_record_offset":848,"inlinee":5741,"source_line":924,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3640,"record_offset":2788,"parent_record_offset":2764,"ancestor_record_offset":848,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3640,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":848,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3640,"record_offset":2424,"parent_record_offset":2400,"ancestor_record_offset":848,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3640,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":848,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3640,"record_offset":1484,"parent_record_offset":1452,"ancestor_record_offset":848,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3640,"record_offset":1540,"parent_record_offset":1516,"ancestor_record_offset":848,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3640,"record_offset":1104,"parent_record_offset":1080,"ancestor_record_offset":848,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3640,"record_offset":1564,"parent_record_offset":1540,"ancestor_record_offset":848,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3640,"record_offset":1128,"parent_record_offset":1104,"ancestor_record_offset":848,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3640,"record_offset":1452,"parent_record_offset":1368,"ancestor_record_offset":848,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3640,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":848,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::taffy_update::taffy_perform_layout
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 56
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14470
// 首条位置: {"module":2420,"inlinee":83056}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2420,"record_offset":383256,"parent_record_offset":383232,"ancestor_record_offset":383232,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/02420.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":2420,"record_offset":383280,"parent_record_offset":383256,"ancestor_record_offset":383232,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/02420.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: toolbar::taffy_update::taffy_update_drawables
// 记录形式: compiled_procedure; instances=1
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3640,"record_offset":2936,"rva":28666624,"va":5397375744}
// 编译实例 RVAs: 0x1b56b00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5603965","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397375744,"raw_hex":"e87d825500","size":5,"soft_fail":false,"target_va":5402980128,"va":5397376158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] taffy::node::Taffy::layout
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-17615086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397375744,"raw_hex":"e81237f3fe","size":5,"soft_fail":false,"target_va":5379760912,"va":5397375993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":4132,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":3288,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3640,"record_offset":3092,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3640,"record_offset":3980,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3640,"record_offset":4076,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3640,"record_offset":4020,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_direct] toolbar::resouces_and_components::taffy_layout::TaffyLayout::layout
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/systems/toolbar/src/resouces_and_components/taffy_layout.rs
//     evidence={"module":3640,"record_offset":3952,"parent_record_offset":2936,"ancestor_record_offset":2936,"inlinee":84160,"source_line":79,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3640,"record_offset":4156,"parent_record_offset":4132,"ancestor_record_offset":2936,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":3856,"parent_record_offset":3728,"ancestor_record_offset":2936,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":2936,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3640,"record_offset":3756,"parent_record_offset":3728,"ancestor_record_offset":2936,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":3140,"parent_record_offset":3116,"ancestor_record_offset":2936,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":3164,"parent_record_offset":3140,"ancestor_record_offset":2936,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3640,"record_offset":3320,"parent_record_offset":3288,"ancestor_record_offset":2936,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3640,"record_offset":3116,"parent_record_offset":3092,"ancestor_record_offset":2936,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3640,"record_offset":3728,"parent_record_offset":3320,"ancestor_record_offset":2936,"inlinee":14626,"source_line":201,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$2::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3640,"record_offset":3616,"parent_record_offset":3320,"ancestor_record_offset":2936,"inlinee":14619,"source_line":192,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3640,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":2936,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":3664,"parent_record_offset":3640,"ancestor_record_offset":2936,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":3588,"parent_record_offset":3320,"ancestor_record_offset":2936,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":3784,"parent_record_offset":3756,"ancestor_record_offset":2936,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3640,"record_offset":3504,"parent_record_offset":3320,"ancestor_record_offset":2936,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ptr::ThinSlicePtr::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ptr/src/lib.rs
//     evidence={"module":3640,"record_offset":3880,"parent_record_offset":3856,"ancestor_record_offset":2936,"inlinee":21393,"source_line":500,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3640,"record_offset":4100,"parent_record_offset":4076,"ancestor_record_offset":2936,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3640,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2936,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3640,"record_offset":3236,"parent_record_offset":3212,"ancestor_record_offset":2936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3640,"record_offset":3468,"parent_record_offset":3444,"ancestor_record_offset":2936,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3640,"record_offset":3416,"parent_record_offset":3380,"ancestor_record_offset":2936,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3640,"record_offset":3528,"parent_record_offset":3504,"ancestor_record_offset":2936,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3640,"record_offset":3188,"parent_record_offset":3164,"ancestor_record_offset":2936,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3640,"record_offset":3552,"parent_record_offset":3528,"ancestor_record_offset":2936,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3640,"record_offset":3212,"parent_record_offset":3188,"ancestor_record_offset":2936,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3640,"record_offset":3444,"parent_record_offset":3380,"ancestor_record_offset":2936,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03640.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3640,"record_offset":3380,"parent_record_offset":3320,"ancestor_record_offset":2936,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03640.jsonl.gz"}
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
// 完整结构及原始类型记录: taffy_update.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: taffy_update.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: taffy_update.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 57; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >", "bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<toolbar::resouces_and_components::UiShowIfParentIsInFocus>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInFpvAndFree> > > >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<country_core::components::UiInteractionState>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::init_photomode_state::PhotomodeState>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>", "bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>", "bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::ShowOnlyInPhotomode> > > >,bevy_ecs::query::filter::With<toolbar::resouces_and_components::ToolbarExpand> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::ToolbarLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<toolbar::resouces_and_components::toolbar_layout::PhotomodeToolbarLayout>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >,bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>,bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>", "bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<country_core::resources::onboarding::WelcomeScreenState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::render_loop::vermeer::VermeerState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_profile::GameProfile>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::game_profile::SaveGameProfileCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>,bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>", "bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<tg_text::TextRenderer>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<tg_text::TextFragment>,ref$<country_core::components::Ui2dExtent> >,tuple$<> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>,bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>,bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >,bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>,bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>,bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >)
// 候选参数类型: ["bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>", "bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>", "bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >", "bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>", "bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>", "bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >"]
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<toolbar::menu::report_bug::FeedbackReportEditorMemory>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::PrevEscMenuState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::NextEscMenuState>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<enum2$<toolbar::menu::report_bug::FeedbackReportUiState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<toolbar::mei::core::MeiContext>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<feedback_reporter::ReportPlayerFeedbackCmd>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<enum2$<feedback_reporter::PlayerFeedbackReportSendingFinished> >
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: toolbar::taffy_update::taffy_finalize_element_size
// 信息 ID: 8e94ff6be19f60860e9f5fd5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] fe6e147fc13b3bbf4f6620a6: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> > @ 0x15e0e88 -> 0x1b56870 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<toolbar::resouces_and_components::TaffyNode>,enum2$<core::option::Option<ref$<toolbar::resouces_and_components::UiScaleWithUserSetting> > >,enum2$<core::option::Option<ref$<tg_text::TextFragment> > > >,tuple$<> >,bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> @ 0x165b73d -> 0x1b56870 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::taffy_update::taffy_perform_layout
// 信息 ID: 0ec0729aeec924353a850eb9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] f66b333540dde192af01277f: void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)> >; module=2420, inline_record=383232; 不是独立调用边。
// [PDB 内联宿主] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>),void (*)(bevy_ecs::change_detection::ResMut<toolbar::resouces_and_components::taffy_layout::TaffyLayout>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>)>; module=2492, inline_record=6352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: toolbar::taffy_update::taffy_update_drawables
// 信息 ID: f5ade4c9f471b71a0e644f4c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] bee0896d9afe50de42bf8daf: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> > @ 0x15e7726 -> 0x1b56b00 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::components::Ui2dExtent>,ref$<toolbar::resouces_and_components::TaffyNode> >,tuple$<> >,bevy_ecs::change_detection::Res<toolbar::resouces_and_components::taffy_layout::TaffyLayout>)> @ 0x165b51b -> 0x1b56b00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
