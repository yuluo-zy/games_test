// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/systems/color/src/color_menu.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: e1ae3055ed3eb78fc142bf2780764d31e8ea867cb82673ec4bede32438f9e55f
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: color_menu.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 3467
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use bevy_ecs; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: []
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"PDB_registry_path_only","version":"0.27.0"}]
// use lerp; // namespace candidate
//   package identities: [{"name":"lerp","source":"PDB_registry_path_only","version":"0.4.0"}]
// use nonmax; // namespace candidate
//   package identities: [{"name":"nonmax","source":"PDB_registry_path_only","version":"0.5.5"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_button_animations
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3467,"record_offset":13384,"rva":27962320,"va":5396671440}
// 编译实例 RVAs: 0x1aaabd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] utils::easing_functions::EasingFunction::apply
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-19733645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396671440,"raw_hex":"e873e3d2fe","size":5,"soft_fail":false,"target_va":5376938064,"va":5396671704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=6
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":14008,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":14404,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3467,"record_offset":13552,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::clamp
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3467,"record_offset":14120,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":6093,"source_line":1519,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::f32::impl$0::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\f32.rs
//     evidence={"module":3467,"record_offset":15600,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":4665,"source_line":944,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::lerp
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":13860,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":14699,"source_line":675,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$10::mul_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":14348,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":14658,"source_line":982,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$9::mul
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":15716,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":14701,"source_line":972,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] lerp::impl$0::lerp
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lerp","version":"0.4.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lerp-0.4.0/src/lib.rs
//     evidence={"module":3467,"record_offset":13748,"parent_record_offset":13384,"ancestor_record_offset":13384,"inlinee":10380,"source_line":171,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=7
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":14032,"parent_record_offset":14008,"ancestor_record_offset":13384,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::matches_component_set
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":14824,"parent_record_offset":14792,"ancestor_record_offset":13384,"inlinee":4919,"source_line":1006,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":15092,"parent_record_offset":14792,"ancestor_record_offset":13384,"inlinee":14620,"source_line":948,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":15360,"parent_record_offset":15328,"ancestor_record_offset":13384,"inlinee":23287,"source_line":1305,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":14708,"parent_record_offset":14684,"ancestor_record_offset":13384,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":15416,"parent_record_offset":15328,"ancestor_record_offset":13384,"inlinee":14630,"source_line":1558,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":14792,"parent_record_offset":14684,"ancestor_record_offset":13384,"inlinee":14622,"source_line":1547,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$36::set_table::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":14852,"parent_record_offset":14824,"ancestor_record_offset":13384,"inlinee":14624,"source_line":1548,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":13600,"parent_record_offset":13576,"ancestor_record_offset":13384,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":13624,"parent_record_offset":13600,"ancestor_record_offset":13384,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":14436,"parent_record_offset":14404,"ancestor_record_offset":13384,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3467,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":13384,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3467,"record_offset":15328,"parent_record_offset":14436,"ancestor_record_offset":13384,"inlinee":14639,"source_line":201,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$4::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3467,"record_offset":14684,"parent_record_offset":14436,"ancestor_record_offset":13384,"inlinee":14640,"source_line":192,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3467,"record_offset":15284,"parent_record_offset":15260,"ancestor_record_offset":13384,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseArray::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":14924,"parent_record_offset":14900,"ancestor_record_offset":13384,"inlinee":4879,"source_line":47,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::contains
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":14900,"parent_record_offset":14876,"ancestor_record_offset":13384,"inlinee":4877,"source_line":362,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":14756,"parent_record_offset":14732,"ancestor_record_offset":13384,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$11::contains::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":15012,"parent_record_offset":14948,"ancestor_record_offset":13384,"inlinee":4881,"source_line":49,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":13384,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":15260,"parent_record_offset":15092,"ancestor_record_offset":13384,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":14504,"parent_record_offset":14436,"ancestor_record_offset":13384,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=2
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":14732,"parent_record_offset":14708,"ancestor_record_offset":13384,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::has_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":14876,"parent_record_offset":14852,"ancestor_record_offset":13384,"inlinee":14625,"source_line":713,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":13384,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":14600,"parent_record_offset":14436,"ancestor_record_offset":13384,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3467,"record_offset":15460,"parent_record_offset":15416,"ancestor_record_offset":13384,"inlinee":4886,"source_line":59,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$124::mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":13800,"parent_record_offset":13748,"ancestor_record_offset":13384,"inlinee":5492,"source_line":344,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":13828,"parent_record_offset":13748,"ancestor_record_offset":13384,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$364::mul_assign
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":14372,"parent_record_offset":14348,"ancestor_record_offset":13384,"inlinee":14660,"source_line":886,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":13772,"parent_record_offset":13748,"ancestor_record_offset":13384,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":15036,"parent_record_offset":15012,"ancestor_record_offset":13384,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":15168,"parent_record_offset":15144,"ancestor_record_offset":13384,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3467,"record_offset":13696,"parent_record_offset":13672,"ancestor_record_offset":13384,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":14568,"parent_record_offset":14536,"ancestor_record_offset":13384,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":14624,"parent_record_offset":14600,"ancestor_record_offset":13384,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":13648,"parent_record_offset":13624,"ancestor_record_offset":13384,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":14648,"parent_record_offset":14624,"ancestor_record_offset":13384,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3467,"record_offset":13672,"parent_record_offset":13648,"ancestor_record_offset":13384,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":14536,"parent_record_offset":14436,"ancestor_record_offset":13384,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":13884,"parent_record_offset":13860,"ancestor_record_offset":13384,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] nonmax::NonMaxUsize::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"nonmax","version":"0.5.5","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/nonmax-0.5.5/src/lib.rs
//     evidence={"module":3467,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":13384,"inlinee":4372,"source_line":153,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::color_menu
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3467,"record_offset":636,"rva":27953008,"va":5396662128}
// 编译实例 RVAs: 0x1aa8770
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=9
//     target_aliases=1 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23914611","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e88d1793fe","size":5,"soft_fail":false,"target_va":5372749696,"va":5396664302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=8
//     target_aliases=1 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-23914476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8141893fe","size":5,"soft_fail":false,"target_va":5372749712,"va":5396664183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406edb80
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20695403","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e89536c4fe","size":5,"soft_fail":false,"target_va":5375974272,"va":5396669670,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t116656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8b0c70100","size":5,"soft_fail":false,"target_va":5396783360,"va":5396666699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141ac6270
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: color_menu.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t119591","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e827d30100","size":5,"soft_fail":false,"target_va":5396783728,"va":5396664132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t6308628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e814436000","size":5,"soft_fail":false,"target_va":5402978144,"va":5396669511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::fmt::format::format_inner
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-19134015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8c109dcfe","size":5,"soft_fail":false,"target_va":5377529888,"va":5396663898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=17
//     evidence={"assembly":"\tcallq\t-25358635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8d50e7dfe","size":5,"soft_fail":false,"target_va":5371304880,"va":5396663510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::query::state::QueryState$LT$D$C$F$GT$::get_unchecked_manual
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-6295863","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8c9ee9fff","size":5,"soft_fail":false,"target_va":5390367728,"va":5396663586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6309337","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8d9456000","size":5,"soft_fail":false,"target_va":5402978880,"va":5396669538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t6310627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8e34a6000","size":5,"soft_fail":false,"target_va":5402980128,"va":5396669496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::history::History::add
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21699822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e812e3b4fe","size":5,"soft_fail":false,"target_va":5374966368,"va":5396666185,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21952395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e87508b1fe","size":5,"soft_fail":false,"target_va":5374716704,"va":5396669094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::resources::walls::public_walls::PublicWalls::get
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-21231763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e86d07bcfe","size":5,"soft_fail":false,"target_va":5375434976,"va":5396666734,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-21113964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e894d3bdfe","size":5,"soft_fail":false,"target_va":5375550288,"va":5396664247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-21113756","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e864d4bdfe","size":5,"soft_fail":false,"target_va":5375550448,"va":5396664199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::empty
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20892480","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8c034c1fe","size":5,"soft_fail":false,"target_va":5375771552,"va":5396664027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_click
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-20892425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8f734c1fe","size":5,"soft_fail":false,"target_va":5375771664,"va":5396664084,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20892460","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8d434c1fe","size":5,"soft_fail":false,"target_va":5375771600,"va":5396664055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::new
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20892849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e84f33c1fe","size":5,"soft_fail":false,"target_va":5375771168,"va":5396664012,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-20894053","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e89b2ec1fe","size":5,"soft_fail":false,"target_va":5375771344,"va":5396665392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::systems::ui::resources::widget_params::WidgetParams::sounds
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t-20892835","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e85d33c1fe","size":5,"soft_fail":false,"target_va":5375771280,"va":5396664110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::angle_range::AngleRange::clamp
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t3268436","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e854df3100","size":5,"soft_fail":false,"target_va":5399934800,"va":5396666359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::cartesian_to_polar
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19723749","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e81b0ad3fe","size":5,"soft_fail":false,"target_va":5376942576,"va":5396666320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::polar_to_cartesian
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19723721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396662128,"raw_hex":"e8370ad3fe","size":5,"soft_fail":false,"target_va":5376942656,"va":5396666372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3467,"record_offset":2308,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4097,"source_line":629,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::Arc::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3467,"record_offset":6476,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=10
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":2252,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$50::deref_mut
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":2168,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":7127,"source_line":326,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::event::EventWriter::send
//     owner=git_dependency_definition; count=18
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3467,"record_offset":1844,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":7450,"source_line":624,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::get_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3467,"record_offset":2100,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":14710,"source_line":963,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter_mut
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3467,"record_offset":836,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":14631,"source_line":473,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":3467,"record_offset":7408,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4576,"source_line":263,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":3467,"record_offset":2768,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4972,"source_line":758,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":3467,"record_offset":1036,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":2224,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":5024,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":7372,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":8096,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4779,"source_line":953,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":2504,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":3467,"record_offset":2128,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":1668,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":6968,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":14696,"source_line":406,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$12::add
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":6912,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":17198,"source_line":1002,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":2448,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::impl$4::div
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":2392,"parent_record_offset":636,"ancestor_record_offset":636,"inlinee":17306,"source_line":923,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":3056,"parent_record_offset":3032,"ancestor_record_offset":636,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":3080,"parent_record_offset":3056,"ancestor_record_offset":636,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":2664,"parent_record_offset":2640,"ancestor_record_offset":636,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":6532,"parent_record_offset":6504,"ancestor_record_offset":636,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":3032,"parent_record_offset":3008,"ancestor_record_offset":636,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3467,"record_offset":2640,"parent_record_offset":2584,"ancestor_record_offset":636,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3467,"record_offset":6504,"parent_record_offset":6476,"ancestor_record_offset":636,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\fmt.rs
//     evidence={"module":3467,"record_offset":2356,"parent_record_offset":2332,"ancestor_record_offset":636,"inlinee":4101,"source_line":637,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":2612,"parent_record_offset":2584,"ancestor_record_offset":636,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":2028,"parent_record_offset":1996,"ancestor_record_offset":636,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":636,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":636,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":3592,"parent_record_offset":3448,"ancestor_record_offset":636,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3467,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":636,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3467,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":636,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3467,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":636,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3467,"record_offset":2864,"parent_record_offset":2840,"ancestor_record_offset":636,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3467,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":636,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":3467,"record_offset":2840,"parent_record_offset":2816,"ancestor_record_offset":636,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":3467,"record_offset":2816,"parent_record_offset":2792,"ancestor_record_offset":636,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$44::from
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\string.rs
//     evidence={"module":3467,"record_offset":2792,"parent_record_offset":2768,"ancestor_record_offset":636,"inlinee":4938,"source_line":2705,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":1996,"parent_record_offset":1960,"ancestor_record_offset":636,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":1960,"parent_record_offset":1924,"ancestor_record_offset":636,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3467,"record_offset":2960,"parent_record_offset":2936,"ancestor_record_offset":636,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$29::set_changed
//     owner=git_dependency_definition; count=27
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":636,"inlinee":5812,"source_line":309,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$30::deref_mut
//     owner=git_dependency_definition; count=17
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":636,"inlinee":5810,"source_line":326,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$49::set_changed
//     owner=git_dependency_definition; count=9
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/change_detection.rs
//     evidence={"module":3467,"record_offset":2192,"parent_record_offset":2168,"ancestor_record_offset":636,"inlinee":7129,"source_line":309,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::event::Events::send
//     owner=git_dependency_definition; count=16
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/event.rs
//     evidence={"module":3467,"record_offset":1924,"parent_record_offset":1844,"ancestor_record_offset":636,"inlinee":7451,"source_line":231,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$31::set_table
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":1372,"parent_record_offset":1092,"ancestor_record_offset":636,"inlinee":23286,"source_line":1291,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":888,"parent_record_offset":864,"ancestor_record_offset":636,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":912,"parent_record_offset":888,"ancestor_record_offset":636,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":1092,"parent_record_offset":1068,"ancestor_record_offset":636,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":1068,"parent_record_offset":1036,"ancestor_record_offset":636,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3467,"record_offset":864,"parent_record_offset":836,"ancestor_record_offset":636,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::blob_vec::BlobVec::get_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/blob_vec.rs
//     evidence={"module":3467,"record_offset":1620,"parent_record_offset":1596,"ancestor_record_offset":636,"inlinee":14643,"source_line":410,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::ImmutableSparseSet::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":1424,"parent_record_offset":1400,"ancestor_record_offset":636,"inlinee":6337,"source_line":369,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::sparse_set::impl$16::get::closure$0
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/sparse_set.rs
//     evidence={"module":3467,"record_offset":1472,"parent_record_offset":1448,"ancestor_record_offset":636,"inlinee":6342,"source_line":370,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Column::get_data_slice
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":1596,"parent_record_offset":1372,"ancestor_record_offset":636,"inlinee":14642,"source_line":321,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":1344,"parent_record_offset":1092,"ancestor_record_offset":636,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::get_column
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":1400,"parent_record_offset":1372,"ancestor_record_offset":636,"inlinee":14621,"source_line":692,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":1260,"parent_record_offset":1092,"ancestor_record_offset":636,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":3467,"record_offset":3652,"parent_record_offset":3628,"ancestor_record_offset":636,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$12::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":6936,"parent_record_offset":6912,"ancestor_record_offset":636,"inlinee":5494,"source_line":105,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$216::div
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":636,"inlinee":5488,"source_line":502,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$68::sub
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\arith.rs
//     evidence={"module":3467,"record_offset":2472,"parent_record_offset":2448,"ancestor_record_offset":636,"inlinee":5486,"source_line":214,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":1448,"parent_record_offset":1424,"ancestor_record_offset":636,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":3467,"record_offset":2332,"parent_record_offset":2308,"ancestor_record_offset":636,"inlinee":4099,"source_line":1172,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3467,"record_offset":1544,"parent_record_offset":1520,"ancestor_record_offset":636,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3467,"record_offset":3628,"parent_record_offset":3384,"ancestor_record_offset":636,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":2528,"parent_record_offset":2504,"ancestor_record_offset":636,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3467,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":636,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":1224,"parent_record_offset":1200,"ancestor_record_offset":636,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":1172,"parent_record_offset":1136,"ancestor_record_offset":636,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":3104,"parent_record_offset":3080,"ancestor_record_offset":636,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3467,"record_offset":2060,"parent_record_offset":1960,"ancestor_record_offset":636,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":1284,"parent_record_offset":1260,"ancestor_record_offset":636,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":1496,"parent_record_offset":1472,"ancestor_record_offset":636,"inlinee":5964,"source_line":679,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":936,"parent_record_offset":912,"ancestor_record_offset":636,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":636,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":1520,"parent_record_offset":1496,"ancestor_record_offset":636,"inlinee":4373,"source_line":232,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3467,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":636,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":1200,"parent_record_offset":1136,"ancestor_record_offset":636,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":1136,"parent_record_offset":1092,"ancestor_record_offset":636,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::color_id::impl$11::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/color_id.rs
//     evidence={"module":3467,"record_offset":8420,"parent_record_offset":8376,"ancestor_record_offset":636,"inlinee":61513,"source_line":26,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::walls::inner_wall_state::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/inner_wall_state.rs
//     evidence={"module":3467,"record_offset":7432,"parent_record_offset":7408,"ancestor_record_offset":636,"inlinee":106637,"source_line":26,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":7048,"parent_record_offset":6992,"ancestor_record_offset":636,"inlinee":14697,"source_line":157,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$17::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.27.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/glam-0.27.0/src/f32/vec2.rs
//     evidence={"module":3467,"record_offset":6992,"parent_record_offset":6968,"ancestor_record_offset":636,"inlinee":14656,"source_line":1051,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\f32.rs
//     evidence={"module":3467,"record_offset":7080,"parent_record_offset":7048,"ancestor_record_offset":636,"inlinee":13701,"source_line":441,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: system_color::color_menu::delete_color_icon_entities
// 记录形式: compiled_procedure; instances=1
// 源码行: 484
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":3467,"record_offset":15832,"rva":27963392,"va":5396672512}
// 编译实例 RVAs: 0x1aab000
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] bevy_ecs::entity::Entities::contains
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19427468","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672512,"raw_hex":"e8748fd7fe","size":5,"soft_fail":false,"target_va":5377245296,"va":5396672759,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::Commands::entity::panic_no_entity
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t6299209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672512,"raw_hex":"e8491e6000","size":5,"soft_fail":false,"target_va":5402972080,"va":5396672866,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bevy_ecs::system::commands::EntityCommands::despawn
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-19367266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396672512,"raw_hex":"e89e7ad8fe","size":5,"soft_fail":false,"target_va":5377305552,"va":5396672813,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::query::iter::impl$1::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":16172,"parent_record_offset":15832,"ancestor_record_offset":15832,"inlinee":14613,"source_line":881,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::commands::Commands::entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3467,"record_offset":16568,"parent_record_offset":15832,"ancestor_record_offset":15832,"inlinee":14632,"source_line":398,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_direct] bevy_ecs::system::query::Query::iter
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/query.rs
//     evidence={"module":3467,"record_offset":15976,"parent_record_offset":15832,"ancestor_record_offset":15832,"inlinee":14606,"source_line":435,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::fetch::impl$21::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/fetch.rs
//     evidence={"module":3467,"record_offset":16500,"parent_record_offset":16476,"ancestor_record_offset":15832,"inlinee":14627,"source_line":963,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIter::new
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":16024,"parent_record_offset":16000,"ancestor_record_offset":15832,"inlinee":14609,"source_line":36,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::init
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":16048,"parent_record_offset":16024,"ancestor_record_offset":15832,"inlinee":14611,"source_line":1520,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::iter::QueryIterationCursor::next
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/iter.rs
//     evidence={"module":3467,"record_offset":16196,"parent_record_offset":16172,"ancestor_record_offset":15832,"inlinee":14614,"source_line":1587,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::state::QueryState::iter_unchecked_manual
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/state.rs
//     evidence={"module":3467,"record_offset":16000,"parent_record_offset":15976,"ancestor_record_offset":15832,"inlinee":14607,"source_line":1240,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::query::world_query::impl$3::fetch
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/query/world_query.rs
//     evidence={"module":3467,"record_offset":16476,"parent_record_offset":16196,"ancestor_record_offset":15832,"inlinee":22691,"source_line":201,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Table::entities
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":16448,"parent_record_offset":16196,"ancestor_record_offset":15832,"inlinee":14616,"source_line":568,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::TableRow::as_usize
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":16524,"parent_record_offset":16500,"ancestor_record_offset":15832,"inlinee":14629,"source_line":125,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::storage::table::Tables::get
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/storage/table.rs
//     evidence={"module":3467,"record_offset":16364,"parent_record_offset":16196,"ancestor_record_offset":15832,"inlinee":14618,"source_line":834,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::system::commands::Commands::get_entity
//     owner=git_dependency_definition; count=1
//     definition_source=/home/h3/.cargo/git/checkouts/bevy-72733c6739490ea8/0dd724d/crates/bevy_ecs/src/system/commands/mod.rs
//     evidence={"module":3467,"record_offset":16604,"parent_record_offset":16568,"ancestor_record_offset":15832,"inlinee":14633,"source_line":447,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::bool::impl$0::then_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\bool.rs
//     evidence={"module":3467,"record_offset":16628,"parent_record_offset":16604,"ancestor_record_offset":15832,"inlinee":14634,"source_line":33,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3467,"record_offset":16120,"parent_record_offset":16096,"ancestor_record_offset":15832,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":16328,"parent_record_offset":16304,"ancestor_record_offset":15832,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3467,"record_offset":16276,"parent_record_offset":16240,"ancestor_record_offset":15832,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":16388,"parent_record_offset":16364,"ancestor_record_offset":15832,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":3467,"record_offset":16072,"parent_record_offset":16048,"ancestor_record_offset":15832,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":3467,"record_offset":16412,"parent_record_offset":16388,"ancestor_record_offset":15832,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":3467,"record_offset":16096,"parent_record_offset":16072,"ancestor_record_offset":15832,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":16304,"parent_record_offset":16240,"ancestor_record_offset":15832,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/03467.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3467,"record_offset":16240,"parent_record_offset":16196,"ancestor_record_offset":15832,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/03467.jsonl.gz"}
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
// 完整结构及原始类型记录: color_menu.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: color_menu.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: color_menu.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >", "bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>", "bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>", "bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>", "bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>", "tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >", "bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>", "bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::history::History>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_button_animations
// 信息 ID: 2978db26b15a61c2c8300eb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 700d28d21d1bc8ddd520f542: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> > @ 0x1498306 -> 0x1aaabd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,ref_mut$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::UiDrawOrder>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> > > >,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x14b1afb -> 0x1aaabd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::color_menu
// 信息 ID: 4973cfc8fbed893bb0dfa526
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 5d1dbfbba64a4461f9dcad13: void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>)> > @ 0x149dfab -> 0x1aa8770 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>),void (*)(bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::resources::color_icon::CustomizationIcon>,enum2$<core::option::Option<ref$<country_core::resources::color_icon::ColorIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::StyleIcon> > >,enum2$<core::option::Option<ref$<country_core::resources::color_icon::FlagConnectionIcon> > >,ref$<country_core::components::Ui2dExtent>,ref_mut$<country_core::components::Ui2dImage> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<system_color::radial_menu::ColorRadialMenu>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_roof_color_id::SetRoofColorIdCmd>,bevy_ecs::event::EventWriter<system_color::set_decorator_color_id::SetDecoratorColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetGateColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::walls::decorator::SetFlagConnectionCmd>,bevy_ecs::event::EventWriter<system_color::show_color_menu::HideColorMenuCmd>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::game_config::GameConfig>,tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState> >,bevy_ecs::event::EventWriter<country_core::systems::wall::set_wall_style::SetWallStyleCmd>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::resources::glade_border::NotInDemoHint>,bevy_ecs::event::EventWriter<country_core::resources::events::LightLanternCmd>)> @ 0x14b1659 -> 0x1aa8770 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: system_color::color_menu::delete_color_icon_entities
// 信息 ID: d36bdf0957e353f245bd8464
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] b72ca01a2f46e826efb23874: void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> > @ 0x149e78b -> 0x1aab000 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::run_unsafe<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::color_icon::CustomizationIcon>,ref$<country_core::resources::color_icon::CustomizationIconMarkedForDeletion> >,tuple$<> >,bevy_ecs::system::commands::Commands)> @ 0x14b0b2d -> 0x1aab000 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
